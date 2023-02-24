use super::*;
use crate::*;

// Diffie-Hellman key exchange cache
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DHCacheKey {
    pub key: PublicKey,
    pub secret: SecretKey,
}

#[derive(Serialize, Deserialize)]
pub struct DHCacheValue {
    pub shared_secret: SharedSecret,
}

pub type DHCache = LruCache<DHCacheKey, DHCacheValue>;
pub const DH_CACHE_SIZE: usize = 4096;

pub fn cache_to_bytes(cache: &DHCache) -> Vec<u8> {
    let cnt: usize = cache.len();
    let mut out: Vec<u8> = Vec::with_capacity(cnt * (32 + 32 + 32));
    for e in cache.iter() {
        out.extend(&e.0.key.bytes);
        out.extend(&e.0.secret.bytes);
        out.extend(&e.1.shared_secret.bytes);
    }
    let mut rev: Vec<u8> = Vec::with_capacity(out.len());
    for d in out.chunks(32 + 32 + 32).rev() {
        rev.extend(d);
    }
    rev
}

pub fn bytes_to_cache(bytes: &[u8], cache: &mut DHCache) {
    for d in bytes.chunks(32 + 32 + 32) {
        let k = DHCacheKey {
            key: PublicKey::new(d[0..32].try_into().expect("asdf")),
            secret: SecretKey::new(d[32..64].try_into().expect("asdf")),
        };
        let v = DHCacheValue {
            shared_secret: SharedSecret::new(d[64..96].try_into().expect("asdf")),
        };
        cache.insert(k, v, |_k, _v| {});
    }
}
