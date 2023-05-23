use super::*;
use argon2::password_hash::Salt;
use data_encoding::BASE64URL_NOPAD;
use digest::Digest;
use rand::RngCore;
const AEAD_OVERHEAD: usize = PUBLIC_KEY_LENGTH;
pub const CRYPTO_KIND_NONE: CryptoKind = FourCC([b'N', b'O', b'N', b'E']);

pub fn none_generate_keypair() -> KeyPair {
    let mut csprng = VeilidRng {};
    let mut pub_bytes = [0u8; PUBLIC_KEY_LENGTH];
    let mut sec_bytes = [0u8; SECRET_KEY_LENGTH];
    csprng.fill_bytes(&mut pub_bytes);
    for n in 0..PUBLIC_KEY_LENGTH {
        sec_bytes[n] = !pub_bytes[n];
    }
    let dht_key = PublicKey::new(pub_bytes);
    let dht_key_secret = SecretKey::new(sec_bytes);
    KeyPair::new(dht_key, dht_key_secret)
}

fn do_xor_32(a: &[u8], b: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for n in 0..32 {
        out[n] = a[n] ^ b[n];
    }
    out
}

fn do_xor_inplace(a: &mut [u8], key: &[u8]) {
    for n in 0..a.len() {
        a[n] ^= key[n % key.len()];
    }
}

fn do_xor_b2b(a: &[u8], b: &mut [u8], key: &[u8]) {
    for n in 0..a.len() {
        b[n] = a[n] ^ key[n % key.len()];
    }
}

fn is_bytes_eq_32(a: &[u8], v: u8) -> bool {
    for n in 0..32 {
        if a[n] != v {
            return false;
        }
    }
    true
}

/// None CryptoSystem
#[derive(Clone)]
pub struct CryptoSystemNONE {
    crypto: Crypto,
}

impl CryptoSystemNONE {
    pub fn new(crypto: Crypto) -> Self {
        Self { crypto }
    }
}

impl CryptoSystem for CryptoSystemNONE {
    // Accessors
    fn kind(&self) -> CryptoKind {
        CRYPTO_KIND_NONE
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
            .cached_dh_internal::<CryptoSystemNONE>(self, key, secret)
    }

    // Generation
    fn random_bytes(&self, len: u32) -> Vec<u8> {
        let mut bytes = unsafe { unaligned_u8_vec_uninit(len as usize) };
        random_bytes(bytes.as_mut());
        bytes
    }
    fn default_salt_length(&self) -> u32 {
        4
    }
    fn hash_password(&self, password: &[u8], salt: &[u8]) -> Result<String, VeilidAPIError> {
        if salt.len() < Salt::MIN_LENGTH || salt.len() > Salt::MAX_LENGTH {
            apibail_generic!("invalid salt length");
        }
        Ok(format!(
            "{}:{}",
            BASE64URL_NOPAD.encode(salt),
            BASE64URL_NOPAD.encode(password)
        ))
    }
    fn verify_password(
        &self,
        password: &[u8],
        password_hash: &str,
    ) -> Result<bool, VeilidAPIError> {
        let Some((salt, _)) = password_hash.split_once(":") else {
            apibail_generic!("invalid format");
        };
        let Ok(salt) = BASE64URL_NOPAD.decode(salt.as_bytes()) else {
            apibail_generic!("invalid salt");
        };
        return Ok(&self.hash_password(password, &salt)? == password_hash);
    }

    fn derive_shared_secret(
        &self,
        password: &[u8],
        salt: &[u8],
    ) -> Result<SharedSecret, VeilidAPIError> {
        if salt.len() < Salt::MIN_LENGTH || salt.len() > Salt::MAX_LENGTH {
            apibail_generic!("invalid salt length");
        }
        Ok(SharedSecret::new(
            *blake3::hash(self.hash_password(password, salt)?.as_bytes()).as_bytes(),
        ))
    }

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
        let s = do_xor_32(&key.bytes, &secret.bytes);
        Ok(SharedSecret::new(s))
    }
    fn generate_keypair(&self) -> KeyPair {
        none_generate_keypair()
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
        if !is_bytes_eq_32(&do_xor_32(&dht_key.bytes, &dht_key_secret.bytes), 0xFFu8) {
            return Err(VeilidAPIError::parse_error(
                "Keypair is invalid",
                "invalid keys",
            ));
        }

        let mut dig = Blake3Digest512::new();
        dig.update(data);
        let sig = dig.finalize();
        let in_sig_bytes: [u8; SIGNATURE_LENGTH] = sig.into();
        let mut sig_bytes = [0u8; SIGNATURE_LENGTH];
        sig_bytes[0..32].copy_from_slice(&in_sig_bytes[0..32]);
        sig_bytes[32..64].copy_from_slice(&do_xor_32(&in_sig_bytes[32..64], &dht_key_secret.bytes));
        let dht_sig = Signature::new(sig_bytes.into());
        Ok(dht_sig)
    }
    fn verify(
        &self,
        dht_key: &PublicKey,
        data: &[u8],
        signature: &Signature,
    ) -> Result<(), VeilidAPIError> {
        let mut dig = Blake3Digest512::new();
        dig.update(data);
        let sig = dig.finalize();
        let in_sig_bytes: [u8; SIGNATURE_LENGTH] = sig.into();
        let mut verify_bytes = [0u8; SIGNATURE_LENGTH];
        verify_bytes[0..32]
            .copy_from_slice(&do_xor_32(&in_sig_bytes[0..32], &signature.bytes[0..32]));
        verify_bytes[32..64]
            .copy_from_slice(&do_xor_32(&in_sig_bytes[32..64], &signature.bytes[32..64]));

        if !is_bytes_eq_32(&verify_bytes[0..32], 0u8) {
            return Err(VeilidAPIError::parse_error(
                "Verification failed",
                "signature 0..32 is invalid",
            ));
        }
        if !is_bytes_eq_32(&do_xor_32(&verify_bytes[32..64], &dht_key.bytes), 0xFFu8) {
            return Err(VeilidAPIError::parse_error(
                "Verification failed",
                "signature 32..64 is invalid",
            ));
        }

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
        _associated_data: Option<&[u8]>,
    ) -> Result<(), VeilidAPIError> {
        let mut blob = nonce.bytes.to_vec();
        blob.extend_from_slice(&[0u8; 8]);
        let blob = do_xor_32(&blob, &shared_secret.bytes);

        if body.len() < AEAD_OVERHEAD {
            return Err(VeilidAPIError::generic("invalid length"));
        }
        if &body[body.len() - AEAD_OVERHEAD..] != &blob {
            return Err(VeilidAPIError::generic("invalid keyblob"));
        }
        body.truncate(body.len() - AEAD_OVERHEAD);
        do_xor_inplace(body, &blob);
        Ok(())
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
        _associated_data: Option<&[u8]>,
    ) -> Result<(), VeilidAPIError> {
        let mut blob = nonce.bytes.to_vec();
        blob.extend_from_slice(&[0u8; 8]);
        let blob = do_xor_32(&blob, &shared_secret.bytes);
        do_xor_inplace(body, &blob);
        body.append(&mut blob.to_vec());
        Ok(())
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
    fn crypt_in_place_no_auth(&self, body: &mut [u8], nonce: &Nonce, shared_secret: &SharedSecret) {
        let mut blob = nonce.bytes.to_vec();
        blob.extend_from_slice(&[0u8; 8]);
        let blob = do_xor_32(&blob, &shared_secret.bytes);
        do_xor_inplace(body, &blob);
    }

    fn crypt_b2b_no_auth(
        &self,
        in_buf: &[u8],
        out_buf: &mut [u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
    ) {
        let mut blob = nonce.bytes.to_vec();
        blob.extend_from_slice(&[0u8; 8]);
        let blob = do_xor_32(&blob, &shared_secret.bytes);
        do_xor_b2b(in_buf, out_buf, &blob);
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
