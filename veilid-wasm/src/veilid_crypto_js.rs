#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type ValidCryptoKinds;
}

#[wasm_bindgen(js_class = veilidCrypto)]
pub struct VeilidCrypto {}

#[wasm_bindgen(js_class = veilidCrypto)]
impl VeilidCrypto {
    pub fn validCryptoKinds() -> ValidCryptoKinds {
        let res = veilid_core::VALID_CRYPTO_KINDS
            .iter()
            .map(|k| (*k).to_string());
        res.map(JsValue::from)
            .collect::<js_sys::Array>()
            .unchecked_into::<ValidCryptoKinds>()
    }

    pub fn bestCryptoKind() -> String {
        veilid_core::best_crypto_kind().to_string()
    }

    pub fn cachedDh(kind: String, key: String, secret: String) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;
        let key: veilid_core::PublicKey = veilid_core::PublicKey::from_str(&key)?;
        let secret: veilid_core::SecretKey = veilid_core::SecretKey::from_str(&secret)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_cached_dh",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.cached_dh(&key, &secret)?;
        APIResult::Ok(out.to_string())
    }

    pub fn computeDh(kind: String, key: String, secret: String) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let key: veilid_core::PublicKey = veilid_core::PublicKey::from_str(&key)?;
        let secret: veilid_core::SecretKey = veilid_core::SecretKey::from_str(&secret)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_compute_dh",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.compute_dh(&key, &secret)?;
        APIResult::Ok(out.to_string())
    }

    pub fn randomBytes(kind: String, len: u32) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_bytes",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.random_bytes(len);
        let out = data_encoding::BASE64URL_NOPAD.encode(&out);
        APIResult::Ok(out)
    }

    pub fn defaultSaltLength(kind: String) -> VeilidAPIResult<u32> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_default_salt_length",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.default_salt_length();
        APIResult::Ok(out)
    }

    pub fn hashPassword(kind: String, password: String, salt: String) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;
        let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(password.as_bytes())
            .unwrap();
        let salt: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(salt.as_bytes())
            .unwrap();

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_hash_password",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.hash_password(&password, &salt)?;
        APIResult::Ok(out)
    }

    pub fn verifyPassword(
        kind: String,
        password: String,
        password_hash: String,
    ) -> VeilidAPIResult<bool> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;
        let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(password.as_bytes())
            .unwrap();

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_verify_password",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.verify_password(&password, &password_hash)?;
        APIResult::Ok(out)
    }

    pub fn deriveSharedSecret(
        kind: String,
        password: String,
        salt: String,
    ) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;
        let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(password.as_bytes())
            .unwrap();
        let salt: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(salt.as_bytes())
            .unwrap();

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_derive_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.derive_shared_secret(&password, &salt)?;
        APIResult::Ok(out.to_string())
    }

    pub fn randomNonce(kind: String) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_nonce",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.random_nonce();
        APIResult::Ok(out.to_string())
    }

    pub fn randomSharedSecret(kind: String) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.random_shared_secret();
        APIResult::Ok(out.to_string())
    }

    pub fn generateKeyPair(kind: String) -> VeilidAPIResult<KeyPair> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_key_pair",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.generate_keypair();
        APIResult::Ok(out)
    }

    pub fn generateHash(kind: String, data: String) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .unwrap();

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_hash",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.generate_hash(&data);
        APIResult::Ok(out.to_string())
    }

    pub fn validateKeyPair(kind: String, key: String, secret: String) -> VeilidAPIResult<bool> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let key: veilid_core::PublicKey = veilid_core::PublicKey::from_str(&key)?;
        let secret: veilid_core::SecretKey = veilid_core::SecretKey::from_str(&secret)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_validate_key_pair",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.validate_keypair(&key, &secret);
        APIResult::Ok(out)
    }

    pub fn validateHash(kind: String, data: String, hash: String) -> VeilidAPIResult<bool> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .unwrap();

        let hash: veilid_core::HashDigest = veilid_core::HashDigest::from_str(&hash)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_validate_hash",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.validate_hash(&data, &hash);
        APIResult::Ok(out)
    }

    pub fn distance(kind: String, key1: String, key2: String) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let key1: veilid_core::CryptoKey = veilid_core::CryptoKey::from_str(&key1)?;
        let key2: veilid_core::CryptoKey = veilid_core::CryptoKey::from_str(&key2)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_distance",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.distance(&key1, &key2);
        APIResult::Ok(out.to_string())
    }

    pub fn sign(
        kind: String,
        key: String,
        secret: String,
        data: String,
    ) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let key: veilid_core::PublicKey = veilid_core::PublicKey::from_str(&key)?;
        let secret: veilid_core::SecretKey = veilid_core::SecretKey::from_str(&secret)?;

        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .unwrap();

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument("crypto_sign", "kind", kind.to_string())
        })?;
        let out = csv.sign(&key, &secret, &data)?;
        APIResult::Ok(out.to_string())
    }

    pub fn verify(
        kind: String,
        key: String,
        data: String,
        signature: String,
    ) -> VeilidAPIResult<()> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let key: veilid_core::PublicKey = veilid_core::PublicKey::from_str(&key)?;
        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .unwrap();
        let signature: veilid_core::Signature = veilid_core::Signature::from_str(&signature)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument("crypto_verify", "kind", kind.to_string())
        })?;
        csv.verify(&key, &data, &signature)?;
        APIRESULT_UNDEFINED
    }

    pub fn aeadOverhead(kind: String) -> VeilidAPIResult<usize> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_aead_overhead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.aead_overhead();
        APIResult::Ok(out)
    }

    pub fn decryptAead(
        kind: String,
        body: String,
        nonce: String,
        shared_secret: String,
        associated_data: Option<String>,
    ) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let body: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(body.as_bytes())
            .unwrap();

        let nonce: veilid_core::Nonce = veilid_core::Nonce::from_str(&nonce)?;

        let shared_secret: veilid_core::SharedSecret =
            veilid_core::SharedSecret::from_str(&shared_secret)?;

        let associated_data: Option<Vec<u8>> = associated_data.map(|ad| {
            data_encoding::BASE64URL_NOPAD
                .decode(ad.as_bytes())
                .unwrap()
        });

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_decrypt_aead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.decrypt_aead(
            &body,
            &nonce,
            &shared_secret,
            match &associated_data {
                Some(ad) => Some(ad.as_slice()),
                None => None,
            },
        )?;
        let out = data_encoding::BASE64URL_NOPAD.encode(&out);
        APIResult::Ok(out)
    }

    pub fn encryptAead(
        kind: String,
        body: String,
        nonce: String,
        shared_secret: String,
        associated_data: Option<String>,
    ) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let body: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(body.as_bytes())
            .unwrap();

        let nonce: veilid_core::Nonce = veilid_core::Nonce::from_str(&nonce).unwrap();

        let shared_secret: veilid_core::SharedSecret =
            veilid_core::SharedSecret::from_str(&shared_secret).unwrap();

        let associated_data: Option<Vec<u8>> = associated_data.map(|ad| {
            data_encoding::BASE64URL_NOPAD
                .decode(ad.as_bytes())
                .unwrap()
        });

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_encrypt_aead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.encrypt_aead(
            &body,
            &nonce,
            &shared_secret,
            match &associated_data {
                Some(ad) => Some(ad.as_slice()),
                None => None,
            },
        )?;
        let out = data_encoding::BASE64URL_NOPAD.encode(&out);
        APIResult::Ok(out)
    }

    pub fn cryptNoAuth(
        kind: String,
        body: String,
        nonce: String,
        shared_secret: String,
    ) -> VeilidAPIResult<String> {
        let kind: veilid_core::CryptoKind = veilid_core::FourCC::from_str(&kind)?;

        let mut body: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(body.as_bytes())
            .unwrap();

        let nonce: veilid_core::Nonce = veilid_core::Nonce::from_str(&nonce).unwrap();

        let shared_secret: veilid_core::SharedSecret =
            veilid_core::SharedSecret::from_str(&shared_secret).unwrap();

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_crypt_no_auth",
                "kind",
                kind.to_string(),
            )
        })?;
        csv.crypt_in_place_no_auth(&mut body, &nonce, &shared_secret);
        let out = data_encoding::BASE64URL_NOPAD.encode(&body);
        APIResult::Ok(out)
    }
}
