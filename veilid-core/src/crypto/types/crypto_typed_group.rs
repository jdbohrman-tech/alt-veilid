use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[serde(from = "Vec<CryptoTyped<K>>", into = "Vec<CryptoTyped<K>>")]
pub struct CryptoTypedGroup<K = PublicKey>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    items: Vec<CryptoTyped<K>>,
}

impl<K> CryptoTypedGroup<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::with_capacity(cap),
        }
    }
    pub fn kinds(&self) -> Vec<CryptoKind> {
        let mut out = Vec::new();
        for tk in &self.items {
            out.push(tk.kind);
        }
        out.sort_by(compare_crypto_kind);
        out
    }
    pub fn keys(&self) -> Vec<K> {
        let mut out = Vec::new();
        for tk in &self.items {
            out.push(tk.value);
        }
        out
    }
    pub fn get(&self, kind: CryptoKind) -> Option<CryptoTyped<K>> {
        self.items.iter().find(|x| x.kind == kind).copied()
    }
    pub fn add(&mut self, typed_key: CryptoTyped<K>) {
        for x in &mut self.items {
            if x.kind == typed_key.kind {
                *x = typed_key;
                return;
            }
        }
        self.items.push(typed_key);
        self.items.sort()
    }
    pub fn add_all(&mut self, typed_keys: &[CryptoTyped<K>]) {
        'outer: for typed_key in typed_keys {
            for x in &mut self.items {
                if x.kind == typed_key.kind {
                    *x = *typed_key;
                    continue 'outer;
                }
            }
            self.items.push(*typed_key);
        }
        self.items.sort()
    }
    pub fn remove(&mut self, kind: CryptoKind) {
        if let Some(idx) = self.items.iter().position(|x| x.kind == kind) {
            self.items.remove(idx);
        }
    }
    pub fn remove_all(&mut self, kinds: &[CryptoKind]) {
        for k in kinds {
            self.remove(*k);
        }
    }
    /// Return preferred typed key of our supported crypto kinds
    pub fn best(&self) -> Option<CryptoTyped<K>> {
        self.items
            .first()
            .copied()
            .filter(|k| VALID_CRYPTO_KINDS.contains(&k.kind))
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn iter(&self) -> core::slice::Iter<'_, CryptoTyped<K>> {
        self.items.iter()
    }
    pub fn contains(&self, typed_key: &CryptoTyped<K>) -> bool {
        self.items.contains(typed_key)
    }
    pub fn contains_any(&self, typed_keys: &[CryptoTyped<K>]) -> bool {
        for typed_key in typed_keys {
            if self.items.contains(typed_key) {
                return true;
            }
        }
        false
    }
    pub fn contains_value(&self, value: &K) -> bool {
        for tk in &self.items {
            if tk.value == *value {
                return true;
            }
        }
        false
    }
}

impl<K> core::ops::Deref for CryptoTypedGroup<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    type Target = [CryptoTyped<K>];

    #[inline]
    fn deref(&self) -> &[CryptoTyped<K>] {
        &self.items
    }
}

impl<K> fmt::Display for CryptoTypedGroup<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[")?;
        let mut first = true;
        for x in &self.items {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }
            write!(f, "{}", x)?;
        }
        write!(f, "]")
    }
}
impl<K> FromStr for CryptoTypedGroup<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        if s.len() < 2 {
            apibail_parse_error!("invalid length", s);
        }
        if &s[0..1] != "[" || &s[(s.len() - 1)..] != "]" {
            apibail_parse_error!("invalid format", s);
        }
        for x in s[1..s.len() - 1].split(',') {
            let tk = CryptoTyped::<K>::from_str(x.trim())?;
            items.push(tk);
        }

        Ok(Self { items })
    }
}
impl<K> From<CryptoTyped<K>> for CryptoTypedGroup<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    fn from(x: CryptoTyped<K>) -> Self {
        let mut tks = CryptoTypedGroup::<K>::with_capacity(1);
        tks.add(x);
        tks
    }
}
impl<K> From<Vec<CryptoTyped<K>>> for CryptoTypedGroup<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    fn from(x: Vec<CryptoTyped<K>>) -> Self {
        let mut tks = CryptoTypedGroup::<K>::with_capacity(x.len());
        tks.add_all(&x);
        tks
    }
}
impl<K> From<&[CryptoTyped<K>]> for CryptoTypedGroup<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    fn from(x: &[CryptoTyped<K>]) -> Self {
        let mut tks = CryptoTypedGroup::<K>::with_capacity(x.len());
        tks.add_all(x);
        tks
    }
}
impl<K> From<CryptoTypedGroup<K>> for Vec<CryptoTyped<K>>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Encodable,
{
    fn from(val: CryptoTypedGroup<K>) -> Self {
        val.items
    }
}
