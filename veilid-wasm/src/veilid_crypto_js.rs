#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen(js_name = veilidCrypto)]
pub struct VeilidCrypto {}

// Since this implementation doesn't contain a `new` fn that's marked as a constructor,
// and none of the member fns take a &self arg,
// this is just a namespace/class of static functions.
#[wasm_bindgen(js_class = veilidCrypto)]
impl VeilidCrypto {
    #[must_use]
    pub fn validCryptoKinds() -> StringArray {
        let res = veilid_core::VALID_CRYPTO_KINDS
            .iter()
            .map(|k| (*k).to_string())
            .collect();
        into_unchecked_string_array(res)
    }

    #[must_use]
    pub fn bestCryptoKind() -> String {
        veilid_core::best_crypto_kind().to_string()
    }

    pub fn cachedDh(kind: String, key: PublicKey, secret: SecretKey) -> APIResult<SharedSecret> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_cached_dh",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.cached_dh(&key, &secret)?;
        APIResult::Ok(out)
    }

    pub fn computeDh(kind: String, key: PublicKey, secret: SecretKey) -> APIResult<SharedSecret> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_compute_dh",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.compute_dh(&key, &secret)?;
        APIResult::Ok(out)
    }

    pub fn generateSharedSecret(
        kind: String,
        key: PublicKey,
        secret: SecretKey,
        domain: Box<[u8]>,
    ) -> APIResult<SharedSecret> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.generate_shared_secret(&key, &secret, &domain)?;
        APIResult::Ok(out)
    }

    pub fn randomBytes(kind: String, len: u32) -> APIResult<Box<[u8]>> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_bytes",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.random_bytes(len);
        let out = out.into_boxed_slice();
        APIResult::Ok(out)
    }

    pub fn defaultSaltLength(kind: String) -> APIResult<u32> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_default_salt_length",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.default_salt_length();
        APIResult::Ok(out)
    }

    pub fn hashPassword(kind: String, password: Box<[u8]>, salt: Box<[u8]>) -> APIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_hash_password",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.hash_password(&password, &salt)?;
        APIResult::Ok(out)
    }

    pub fn verifyPassword(
        kind: String,
        password: Box<[u8]>,
        password_hash: String,
    ) -> APIResult<bool> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_verify_password",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.verify_password(&password, &password_hash)?;
        APIResult::Ok(out)
    }

    pub fn deriveSharedSecret(
        kind: String,
        password: Box<[u8]>,
        salt: Box<[u8]>,
    ) -> APIResult<SharedSecret> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_derive_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.derive_shared_secret(&password, &salt)?;
        APIResult::Ok(out)
    }

    pub fn randomNonce(kind: String) -> APIResult<Nonce> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_nonce",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.random_nonce();
        APIResult::Ok(out)
    }

    pub fn randomSharedSecret(kind: String) -> APIResult<SharedSecret> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.random_shared_secret();
        APIResult::Ok(out)
    }

    pub fn verifySignatures(
        node_ids: StringArray,
        data: Box<[u8]>,
        signatures: StringArray,
    ) -> VeilidAPIResult<Option<StringArray>> {
        let node_ids = into_unchecked_string_vec(node_ids);
        let node_ids: Vec<TypedPublicKey> = node_ids
            .iter()
            .map(|k| {
                veilid_core::TypedPublicKey::from_str(k).map_err(|e| {
                    VeilidAPIError::invalid_argument(
                        "verifySignatures()",
                        format!("error decoding nodeid in node_ids[]: {}", e),
                        k,
                    )
                })
            })
            .collect::<APIResult<Vec<TypedPublicKey>>>()?;

        let typed_signatures = into_unchecked_string_vec(signatures);
        let typed_signatures: Vec<TypedSignature> = typed_signatures
            .iter()
            .map(|k| {
                TypedSignature::from_str(k).map_err(|e| {
                    VeilidAPIError::invalid_argument(
                        "verifySignatures()",
                        format!("error decoding keypair in key_pairs[]: {}", e),
                        k,
                    )
                })
            })
            .collect::<APIResult<Vec<TypedSignature>>>()?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let out = crypto
            .verify_signatures(&node_ids, &data, &typed_signatures)?
            .map(|sigs| {
                let out = sigs
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>();
                into_unchecked_string_array(out)
            });
        APIResult::Ok(out)
    }

    pub fn generateSignatures(data: Box<[u8]>, key_pairs: StringArray) -> APIResult<StringArray> {
        let key_pairs = into_unchecked_string_vec(key_pairs);
        let key_pairs: Vec<TypedKeyPair> = key_pairs
            .iter()
            .map(|k| {
                veilid_core::TypedKeyPair::from_str(k).map_err(|e| {
                    VeilidAPIError::invalid_argument(
                        "generateSignatures()",
                        format!("error decoding keypair in key_pairs[]: {}", e),
                        k,
                    )
                })
            })
            .collect::<APIResult<Vec<veilid_core::TypedKeyPair>>>()?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let out = crypto.generate_signatures(&data, &key_pairs, |k, s| {
            veilid_core::TypedSignature::new(k.kind, s).to_string()
        })?;
        let out = into_unchecked_string_array(out);
        APIResult::Ok(out)
    }

    pub fn generateKeyPair(kind: String) -> APIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_key_pair",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.generate_keypair();
        let out = out.encode();
        APIResult::Ok(out)
    }

    pub fn generateHash(kind: String, data: Box<[u8]>) -> APIResult<HashDigest> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_hash",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.generate_hash(&data);
        APIResult::Ok(out)
    }

    pub fn validateKeyPair(kind: String, key: String, secret: String) -> APIResult<bool> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let key: veilid_core::PublicKey = veilid_core::PublicKey::from_str(&key)?;
        let secret: veilid_core::SecretKey = veilid_core::SecretKey::from_str(&secret)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_validate_key_pair",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.validate_keypair(&key, &secret);
        APIResult::Ok(out)
    }

    pub fn validateHash(kind: String, data: Box<[u8]>, hash: HashDigest) -> APIResult<bool> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_validate_hash",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.validate_hash(&data, &hash);
        APIResult::Ok(out)
    }

    pub fn distance(kind: String, hash1: HashDigest, hash2: HashDigest) -> APIResult<HashDistance> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_distance",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.distance(&hash1, &hash2);
        APIResult::Ok(out)
    }

    pub fn sign(
        kind: String,
        key: String,
        secret: String,
        data: Box<[u8]>,
    ) -> APIResult<Signature> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let key: veilid_core::PublicKey = veilid_core::PublicKey::from_str(&key)?;
        let secret: veilid_core::SecretKey = veilid_core::SecretKey::from_str(&secret)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument("crypto_sign", "kind", kind.to_string())
        })?;
        let out = crypto_system.sign(&key, &secret, &data)?;
        APIResult::Ok(out)
    }

    pub fn verify(
        kind: String,
        key: PublicKey,
        data: Box<[u8]>,
        signature: Signature,
    ) -> APIResult<bool> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument("crypto_verify", "kind", kind.to_string())
        })?;
        let out = crypto_system.verify(&key, &data, &signature)?;
        APIResult::Ok(out)
    }

    pub fn aeadOverhead(kind: String) -> APIResult<usize> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_aead_overhead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.aead_overhead();
        APIResult::Ok(out)
    }

    pub fn decryptAead(
        kind: String,
        body: Box<[u8]>,
        nonce: Nonce,
        shared_secret: SharedSecret,
        associated_data: Option<Box<[u8]>>,
    ) -> APIResult<Box<[u8]>> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_decrypt_aead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.decrypt_aead(
            &body,
            &nonce,
            &shared_secret,
            match &associated_data {
                Some(ad) => Some(ad),
                None => None,
            },
        )?;
        let out = out.into_boxed_slice();
        APIResult::Ok(out)
    }

    pub fn encryptAead(
        kind: String,
        body: Box<[u8]>,
        nonce: Nonce,
        shared_secret: SharedSecret,
        associated_data: Option<Box<[u8]>>,
    ) -> APIResult<Box<[u8]>> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_encrypt_aead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = crypto_system.encrypt_aead(
            &body,
            &nonce,
            &shared_secret,
            match &associated_data {
                Some(ad) => Some(ad),
                None => None,
            },
        )?;
        APIResult::Ok(out.into_boxed_slice())
    }

    pub fn cryptNoAuth(
        kind: String,
        mut body: Box<[u8]>,
        nonce: Nonce,
        shared_secret: SharedSecret,
    ) -> APIResult<Box<[u8]>> {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let crypto_system = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_crypt_no_auth",
                "kind",
                kind.to_string(),
            )
        })?;
        crypto_system.crypt_in_place_no_auth(&mut body, &nonce, &shared_secret);
        APIResult::Ok(body)
    }

    // --------------------------------
    // Constants
    // (written as getters since wasm_bindgen doesn't support export of const)
    // --------------------------------

    /// Length of a crypto key in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn CRYPTO_KEY_LENGTH() -> usize {
        veilid_core::CRYPTO_KEY_LENGTH
    }

    /// Length of a crypto key in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn CRYPTO_KEY_LENGTH_ENCODED() -> usize {
        veilid_core::CRYPTO_KEY_LENGTH_ENCODED
    }

    /// Length of a hash digest in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn HASH_DIGEST_LENGTH() -> usize {
        veilid_core::HASH_DIGEST_LENGTH
    }

    /// Length of a hash digest in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn HASH_DIGEST_LENGTH_ENCODED() -> usize {
        veilid_core::HASH_DIGEST_LENGTH_ENCODED
    }

    /// Length of a nonce in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn NONCE_LENGTH() -> usize {
        veilid_core::NONCE_LENGTH
    }

    /// Length of a nonce in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn NONCE_LENGTH_ENCODED() -> usize {
        veilid_core::NONCE_LENGTH_ENCODED
    }

    /// Length of a crypto key in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn PUBLIC_KEY_LENGTH() -> usize {
        veilid_core::PUBLIC_KEY_LENGTH
    }

    /// Length of a crypto key in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn PUBLIC_KEY_LENGTH_ENCODED() -> usize {
        veilid_core::PUBLIC_KEY_LENGTH_ENCODED
    }

    /// Length of a route id in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn ROUTE_ID_LENGTH() -> usize {
        veilid_core::ROUTE_ID_LENGTH
    }

    /// Length of a route id in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn ROUTE_ID_LENGTH_ENCODED() -> usize {
        veilid_core::ROUTE_ID_LENGTH_ENCODED
    }

    /// Length of a secret key in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn SECRET_KEY_LENGTH() -> usize {
        veilid_core::SECRET_KEY_LENGTH
    }

    /// Length of a secret key in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn SECRET_KEY_LENGTH_ENCODED() -> usize {
        veilid_core::SECRET_KEY_LENGTH_ENCODED
    }

    /// Length of a shared secret in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn SHARED_SECRET_LENGTH() -> usize {
        veilid_core::SHARED_SECRET_LENGTH
    }

    /// Length of a shared secret in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn SHARED_SECRET_LENGTH_ENCODED() -> usize {
        veilid_core::SHARED_SECRET_LENGTH_ENCODED
    }

    /// Length of a signature in bytes
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn SIGNATURE_LENGTH() -> usize {
        veilid_core::SIGNATURE_LENGTH
    }

    /// Length of a signature in bytes after encoding to base64url
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn SIGNATURE_LENGTH_ENCODED() -> usize {
        veilid_core::SIGNATURE_LENGTH_ENCODED
    }
}
