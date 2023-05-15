use super::*;

use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::XChaCha20;
use chacha20poly1305 as ch;
use chacha20poly1305::aead::{AeadInPlace, NewAead};
use core::convert::TryInto;
use curve25519_dalek as cd;
use digest::Digest;
use ed25519_dalek as ed;
use x25519_dalek as xd;

const AEAD_OVERHEAD: usize = 16;
pub const CRYPTO_KIND_VLD0: CryptoKind = FourCC([b'V', b'L', b'D', b'0']);

fn ed25519_to_x25519_pk(key: &ed::PublicKey) -> Result<xd::PublicKey, VeilidAPIError> {
    let bytes = key.to_bytes();
    let compressed = cd::edwards::CompressedEdwardsY(bytes);
    let point = compressed
        .decompress()
        .ok_or_else(|| VeilidAPIError::internal("ed25519_to_x25519_pk failed"))?;
    let mp = point.to_montgomery();
    Ok(xd::PublicKey::from(mp.to_bytes()))
}
fn ed25519_to_x25519_sk(key: &ed::SecretKey) -> Result<xd::StaticSecret, VeilidAPIError> {
    let exp = ed::ExpandedSecretKey::from(key);
    let bytes: [u8; ed::EXPANDED_SECRET_KEY_LENGTH] = exp.to_bytes();
    let lowbytes: [u8; 32] = bytes[0..32].try_into().map_err(VeilidAPIError::internal)?;
    Ok(xd::StaticSecret::from(lowbytes))
}

pub fn vld0_generate_keypair() -> KeyPair {
    let mut csprng = VeilidRng {};
    let keypair = ed::Keypair::generate(&mut csprng);
    let dht_key = PublicKey::new(keypair.public.to_bytes());
    let dht_key_secret = SecretKey::new(keypair.secret.to_bytes());

    KeyPair::new(dht_key, dht_key_secret)
}

/// V0 CryptoSystem
#[derive(Clone)]
pub struct CryptoSystemVLD0 {
    crypto: Crypto,
}

impl CryptoSystemVLD0 {
    pub fn new(crypto: Crypto) -> Self {
        Self { crypto }
    }
}

impl CryptoSystem for CryptoSystemVLD0 {
    // Accessors
    fn kind(&self) -> CryptoKind {
        CRYPTO_KIND_VLD0
    }

    fn crypto(&self) -> Crypto {
        self.crypto.clone()
    }

    // Cached Operations
    fn cached_dh(
        &self,
        key: &PublicKey,
        secret: &SecretKey,
    ) -> Result<SharedSecret, VeilidAPIError> {
        self.crypto
            .cached_dh_internal::<CryptoSystemVLD0>(self, key, secret)
    }

    // Generation
    fn random_nonce(&self) -> Nonce {
        let mut nonce = [0u8; NONCE_LENGTH];
        random_bytes(&mut nonce).unwrap();
        Nonce::new(nonce)
    }
    fn random_shared_secret(&self) -> SharedSecret {
        let mut s = [0u8; SHARED_SECRET_LENGTH];
        random_bytes(&mut s).unwrap();
        SharedSecret::new(s)
    }
    fn compute_dh(
        &self,
        key: &PublicKey,
        secret: &SecretKey,
    ) -> Result<SharedSecret, VeilidAPIError> {
        let pk_ed = ed::PublicKey::from_bytes(&key.bytes).map_err(VeilidAPIError::internal)?;
        let pk_xd = ed25519_to_x25519_pk(&pk_ed)?;
        let sk_ed = ed::SecretKey::from_bytes(&secret.bytes).map_err(VeilidAPIError::internal)?;
        let sk_xd = ed25519_to_x25519_sk(&sk_ed)?;
        Ok(SharedSecret::new(sk_xd.diffie_hellman(&pk_xd).to_bytes()))
    }
    fn generate_keypair(&self) -> KeyPair {
        vld0_generate_keypair()
    }
    fn generate_hash(&self, data: &[u8]) -> PublicKey {
        PublicKey::new(*blake3::hash(data).as_bytes())
    }
    fn generate_hash_reader(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<PublicKey, VeilidAPIError> {
        let mut hasher = blake3::Hasher::new();
        std::io::copy(reader, &mut hasher).map_err(VeilidAPIError::generic)?;
        Ok(PublicKey::new(*hasher.finalize().as_bytes()))
    }

    // Validation
    fn validate_keypair(&self, dht_key: &PublicKey, dht_key_secret: &SecretKey) -> bool {
        let data = vec![0u8; 512];
        let sig = match self.sign(dht_key, dht_key_secret, &data) {
            Ok(s) => s,
            Err(_) => {
                return false;
            }
        };
        self.verify(dht_key, &data, &sig).is_ok()
    }
    fn validate_hash(&self, data: &[u8], dht_key: &PublicKey) -> bool {
        let bytes = *blake3::hash(data).as_bytes();

        bytes == dht_key.bytes
    }
    fn validate_hash_reader(
        &self,
        reader: &mut dyn std::io::Read,
        dht_key: &PublicKey,
    ) -> Result<bool, VeilidAPIError> {
        let mut hasher = blake3::Hasher::new();
        std::io::copy(reader, &mut hasher).map_err(VeilidAPIError::generic)?;
        let bytes = *hasher.finalize().as_bytes();
        Ok(bytes == dht_key.bytes)
    }
    // Distance Metric
    fn distance(&self, key1: &PublicKey, key2: &PublicKey) -> CryptoKeyDistance {
        let mut bytes = [0u8; PUBLIC_KEY_LENGTH];

        for (n, byte) in bytes.iter_mut().enumerate() {
            *byte = key1.bytes[n] ^ key2.bytes[n];
        }

        CryptoKeyDistance::new(bytes)
    }

    // Authentication
    fn sign(
        &self,
        dht_key: &PublicKey,
        dht_key_secret: &SecretKey,
        data: &[u8],
    ) -> Result<Signature, VeilidAPIError> {
        let mut kpb: [u8; SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH] =
            [0u8; SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH];

        kpb[..SECRET_KEY_LENGTH].copy_from_slice(&dht_key_secret.bytes);
        kpb[SECRET_KEY_LENGTH..].copy_from_slice(&dht_key.bytes);
        let keypair = ed::Keypair::from_bytes(&kpb)
            .map_err(|e| VeilidAPIError::parse_error("Keypair is invalid", e))?;

        let mut dig = Blake3Digest512::new();
        dig.update(data);

        let sig_bytes = keypair
            .sign_prehashed(dig, None)
            .map_err(VeilidAPIError::internal)?;

        let sig = Signature::new(sig_bytes.to_bytes());

        self.verify(dht_key, &data, &sig)?;

        Ok(sig)
    }
    fn verify(
        &self,
        dht_key: &PublicKey,
        data: &[u8],
        signature: &Signature,
    ) -> Result<(), VeilidAPIError> {
        let pk = ed::PublicKey::from_bytes(&dht_key.bytes)
            .map_err(|e| VeilidAPIError::parse_error("Public key is invalid", e))?;
        let sig = ed::Signature::from_bytes(&signature.bytes)
            .map_err(|e| VeilidAPIError::parse_error("Signature is invalid", e))?;

        let mut dig = Blake3Digest512::new();
        dig.update(data);

        pk.verify_prehashed(dig, None, &sig)
            .map_err(|e| VeilidAPIError::parse_error("Verification failed", e))?;
        Ok(())
    }

    // AEAD Encrypt/Decrypt
    fn aead_overhead(&self) -> usize {
        AEAD_OVERHEAD
    }
    fn decrypt_in_place_aead(
        &self,
        body: &mut Vec<u8>,
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<(), VeilidAPIError> {
        let key = ch::Key::from(shared_secret.bytes);
        let xnonce = ch::XNonce::from(nonce.bytes);
        let aead = ch::XChaCha20Poly1305::new(&key);
        aead.decrypt_in_place(&xnonce, associated_data.unwrap_or(b""), body)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)
    }

    fn decrypt_aead(
        &self,
        body: &[u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut out = body.to_vec();
        self.decrypt_in_place_aead(&mut out, nonce, shared_secret, associated_data)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)?;
        Ok(out)
    }

    fn encrypt_in_place_aead(
        &self,
        body: &mut Vec<u8>,
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<(), VeilidAPIError> {
        let key = ch::Key::from(shared_secret.bytes);
        let xnonce = ch::XNonce::from(nonce.bytes);
        let aead = ch::XChaCha20Poly1305::new(&key);

        aead.encrypt_in_place(&xnonce, associated_data.unwrap_or(b""), body)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)
    }

    fn encrypt_aead(
        &self,
        body: &[u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut out = body.to_vec();
        self.encrypt_in_place_aead(&mut out, nonce, shared_secret, associated_data)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)?;
        Ok(out)
    }

    // NoAuth Encrypt/Decrypt
    fn crypt_in_place_no_auth(
        &self,
        body: &mut Vec<u8>,
        nonce: &Nonce,
        shared_secret: &SharedSecret,
    ) {
        let mut cipher = XChaCha20::new(&shared_secret.bytes.into(), &nonce.bytes.into());
        cipher.apply_keystream(body);
    }

    fn crypt_b2b_no_auth(
        &self,
        in_buf: &[u8],
        out_buf: &mut [u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
    ) {
        let mut cipher = XChaCha20::new(&shared_secret.bytes.into(), &nonce.bytes.into());
        cipher.apply_keystream_b2b(in_buf, out_buf).unwrap();
    }

    fn crypt_no_auth_aligned_8(
        &self,
        in_buf: &[u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
    ) -> Vec<u8> {
        let mut out_buf = unsafe { aligned_8_u8_vec_uninit(in_buf.len()) };
        self.crypt_b2b_no_auth(in_buf, &mut out_buf, nonce, shared_secret);
        out_buf
    }

    fn crypt_no_auth_unaligned(
        &self,
        in_buf: &[u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
    ) -> Vec<u8> {
        let mut out_buf = unsafe { unaligned_u8_vec_uninit(in_buf.len()) };
        self.crypt_b2b_no_auth(in_buf, &mut out_buf, nonce, shared_secret);
        out_buf
    }
}
