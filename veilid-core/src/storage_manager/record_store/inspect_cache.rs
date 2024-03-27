use super::*;

const L2_CACHE_DEPTH: usize = 4; // XXX: i just picked this. we could probably do better than this someday

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InspectCacheL2Value {
    pub seqs: Vec<ValueSeqNum>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct InspectCacheL2 {
    pub cache: LruCache<ValueSubkeyRangeSet, InspectCacheL2Value>,
}

impl InspectCacheL2 {
    pub fn new(l2_cache_limit: usize) -> Self {
        Self {
            cache: LruCache::new(l2_cache_limit),
        }
    }
}

pub struct InspectCache {
    cache: LruCache<TypedKey, InspectCacheL2>,
}

impl InspectCache {
    pub fn new(l1_cache_limit: usize) -> Self {
        Self {
            cache: LruCache::new(l1_cache_limit),
        }
    }

    pub fn get(
        &mut self,
        key: &TypedKey,
        subkeys: &ValueSubkeyRangeSet,
    ) -> Option<InspectCacheL2Value> {
        if let Some(l2c) = self.cache.get_mut(key) {
            if let Some(l2v) = l2c.cache.get(subkeys) {
                return Some(l2v.clone());
            }
        }
        None
    }

    pub fn put(&mut self, key: TypedKey, subkeys: ValueSubkeyRangeSet, value: InspectCacheL2Value) {
        self.cache
            .entry(key)
            .or_insert_with(|| InspectCacheL2::new(L2_CACHE_DEPTH))
            .cache
            .insert(subkeys, value);
    }

    pub fn invalidate(&mut self, key: &TypedKey) {
        self.cache.remove(key);
    }

    pub fn replace_subkey_seq(&mut self, key: &TypedKey, subkey: ValueSubkey, seq: ValueSeqNum) {
        let Some(l2) = self.cache.get_mut(key) else {
            return;
        };

        for entry in &mut l2.cache {
            let Some(idx) = entry.0.idx_of_subkey(subkey) else {
                continue;
            };
            if idx < entry.1.seqs.len() {
                entry.1.seqs[idx] = seq;
            } else if idx > entry.1.seqs.len() {
                panic!(
                    "representational error in l2 inspect cache: {} > {}",
                    idx,
                    entry.1.seqs.len()
                )
            }
        }
    }
}
