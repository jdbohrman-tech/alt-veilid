use super::*;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString},
    Argon2,
};
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::XChaCha20;
use chacha20poly1305 as ch;
use chacha20poly1305::aead::AeadInPlace;
use chacha20poly1305::KeyInit;
use curve25519_dalek::digest::Digest;
use ed25519_dalek as ed;
use x25519_dalek as xd;

const AEAD_OVERHEAD: usize = 16;
pub const CRYPTO_KIND_VLD0: CryptoKind = FourCC(*b"VLD0");

fn public_to_x25519_pk(public: &PublicKey) -> VeilidAPIResult<xd::PublicKey> {
    let pk_ed = ed::VerifyingKey::from_bytes(&public.bytes).map_err(VeilidAPIError::internal)?;
    Ok(xd::PublicKey::from(*pk_ed.to_montgomery().as_bytes()))
}
fn secret_to_x25519_sk(secret: &SecretKey) -> VeilidAPIResult<xd::StaticSecret> {
    // NOTE: ed::SigningKey.to_scalar() does not produce an unreduced scalar, we want the raw bytes here
    // See https://github.com/dalek-cryptography/curve25519-dalek/issues/565
    let hash: [u8; SIGNATURE_LENGTH] = ed::Sha512::default()
        .chain_update(secret.bytes)
        .finalize()
        .into();
    let mut output = [0u8; 32];
    output.copy_from_slice(&hash[..32]);

    Ok(xd::StaticSecret::from(output))
}

pub fn vld0_generate_keypair() -> KeyPair {
    let mut csprng = VeilidRng {};
    let signing_key = ed::SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    let dht_key = PublicKey::new(verifying_key.to_bytes());
    let dht_key_secret = SecretKey::new(signing_key.to_bytes());

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
    fn cached_dh(&self, key: &PublicKey, secret: &SecretKey) -> VeilidAPIResult<SharedSecret> {
        self.crypto
            .cached_dh_internal::<CryptoSystemVLD0>(self, key, secret)
    }

    // Generation
    fn random_bytes(&self, len: u32) -> Vec<u8> {
        let mut bytes = unsafe { unaligned_u8_vec_uninit(len as usize) };
        random_bytes(bytes.as_mut());
        bytes
    }
    fn default_salt_length(&self) -> u32 {
        16
    }
    fn hash_password(&self, password: &[u8], salt: &[u8]) -> VeilidAPIResult<String> {
        if salt.len() < Salt::MIN_LENGTH || salt.len() > Salt::MAX_LENGTH {
            apibail_generic!("invalid salt length");
        }

        // Hash password to PHC string ($argon2id$v=19$...)
        let salt = SaltString::encode_b64(salt).map_err(VeilidAPIError::generic)?;

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password, &salt)
            .map_err(VeilidAPIError::generic)?
            .to_string();
        Ok(password_hash)
    }
    fn verify_password(&self, password: &[u8], password_hash: &str) -> VeilidAPIResult<bool> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(VeilidAPIError::generic)?;
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        Ok(argon2.verify_password(password, &parsed_hash).is_ok())
    }

    fn derive_shared_secret(&self, password: &[u8], salt: &[u8]) -> VeilidAPIResult<SharedSecret> {
        if salt.len() < Salt::MIN_LENGTH || salt.len() > Salt::MAX_LENGTH {
            apibail_generic!("invalid salt length");
        }

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        let mut output_key_material = [0u8; SHARED_SECRET_LENGTH];
        argon2
            .hash_password_into(password, salt, &mut output_key_material)
            .map_err(VeilidAPIError::generic)?;
        Ok(SharedSecret::new(output_key_material))
    }

    fn random_nonce(&self) -> Nonce {
        let mut nonce = [0u8; NONCE_LENGTH];
        random_bytes(&mut nonce);
        Nonce::new(nonce)
    }
    fn random_shared_secret(&self) -> SharedSecret {
        let mut s = [0u8; SHARED_SECRET_LENGTH];
        random_bytes(&mut s);
        SharedSecret::new(s)
    }
    fn compute_dh(&self, key: &PublicKey, secret: &SecretKey) -> VeilidAPIResult<SharedSecret> {
        let pk_xd = public_to_x25519_pk(&key)?;
        let sk_xd = secret_to_x25519_sk(&secret)?;

        Ok(SharedSecret::new(sk_xd.diffie_hellman(&pk_xd).to_bytes()))
    }
    fn generate_keypair(&self) -> KeyPair {
        vld0_generate_keypair()
    }
    fn generate_hash(&self, data: &[u8]) -> PublicKey {
        PublicKey::new(*blake3::hash(data).as_bytes())
    }
    fn generate_hash_reader(&self, reader: &mut dyn std::io::Read) -> VeilidAPIResult<PublicKey> {
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
    ) -> VeilidAPIResult<bool> {
        let mut hasher = blake3::Hasher::new();
        std::io::copy(reader, &mut hasher).map_err(VeilidAPIError::generic)?;
        let bytes = *hasher.finalize().as_bytes();
        Ok(bytes == dht_key.bytes)
    }
    // Distance Metric
    fn distance(&self, key1: &PublicKey, key2: &PublicKey) -> CryptoKeyDistance {
        let mut bytes = [0u8; CRYPTO_KEY_LENGTH];

        for n in 0..CRYPTO_KEY_LENGTH {
            bytes[n] = key1.bytes[n] ^ key2.bytes[n];
        }

        CryptoKeyDistance::new(bytes)
    }

    // Authentication
    fn sign(
        &self,
        dht_key: &PublicKey,
        dht_key_secret: &SecretKey,
        data: &[u8],
    ) -> VeilidAPIResult<Signature> {
        let mut kpb: [u8; SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH] =
            [0u8; SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH];

        kpb[..SECRET_KEY_LENGTH].copy_from_slice(&dht_key_secret.bytes);
        kpb[SECRET_KEY_LENGTH..].copy_from_slice(&dht_key.bytes);
        let keypair = ed::SigningKey::from_keypair_bytes(&kpb)
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
    ) -> VeilidAPIResult<()> {
        let pk = ed::VerifyingKey::from_bytes(&dht_key.bytes)
            .map_err(|e| VeilidAPIError::parse_error("Public key is invalid", e))?;
        let sig = ed::Signature::from_bytes(&signature.bytes);
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
    ) -> VeilidAPIResult<()> {
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
    ) -> VeilidAPIResult<Vec<u8>> {
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
    ) -> VeilidAPIResult<()> {
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
    ) -> VeilidAPIResult<Vec<u8>> {
        let mut out = body.to_vec();
        self.encrypt_in_place_aead(&mut out, nonce, shared_secret, associated_data)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)?;
        Ok(out)
    }

    // NoAuth Encrypt/Decrypt
    fn crypt_in_place_no_auth(
        &self,
        body: &mut [u8],
        nonce: &[u8; NONCE_LENGTH],
        shared_secret: &SharedSecret,
    ) {
        let mut cipher = <XChaCha20 as KeyIvInit>::new(&shared_secret.bytes.into(), nonce.into());
        cipher.apply_keystream(body);
    }

    fn crypt_b2b_no_auth(
        &self,
        in_buf: &[u8],
        out_buf: &mut [u8],
        nonce: &[u8; NONCE_LENGTH],
        shared_secret: &SharedSecret,
    ) {
        let mut cipher = <XChaCha20 as KeyIvInit>::new(&shared_secret.bytes.into(), nonce.into());
        cipher.apply_keystream_b2b(in_buf, out_buf).unwrap();
    }

    fn crypt_no_auth_aligned_8(
        &self,
        in_buf: &[u8],
        nonce: &[u8; NONCE_LENGTH],
        shared_secret: &SharedSecret,
    ) -> Vec<u8> {
        let mut out_buf = unsafe { aligned_8_u8_vec_uninit(in_buf.len()) };
        self.crypt_b2b_no_auth(in_buf, &mut out_buf, nonce, shared_secret);
        out_buf
    }

    fn crypt_no_auth_unaligned(
        &self,
        in_buf: &[u8],
        nonce: &[u8; NONCE_LENGTH],
        shared_secret: &SharedSecret,
    ) -> Vec<u8> {
        let mut out_buf = unsafe { unaligned_u8_vec_uninit(in_buf.len()) };
        self.crypt_b2b_no_auth(in_buf, &mut out_buf, nonce, shared_secret);
        out_buf
    }
}
