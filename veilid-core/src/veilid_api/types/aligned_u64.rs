use super::*;

/// Aligned u64
/// Required on 32-bit platforms for serialization because Rust aligns u64 on 4 byte boundaries
/// Some zero-copy serialization frameworks also want 8-byte alignment
/// Supports serializing to string for JSON as well, since JSON can't handle 64-bit numbers to Javascript

#[derive(
    Clone, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Hash, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[repr(C, align(8))]
#[serde(transparent)]
pub struct AlignedU64(
    #[serde(with = "as_human_string")]
    #[schemars(with = "String")]
    u64,
);

impl From<u64> for AlignedU64 {
    fn from(v: u64) -> Self {
        AlignedU64(v)
    }
}
impl From<AlignedU64> for u64 {
    fn from(v: AlignedU64) -> Self {
        v.0
    }
}

impl fmt::Display for AlignedU64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&self.0 as &dyn fmt::Display).fmt(f)
    }
}

impl fmt::Debug for AlignedU64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&self.0 as &dyn fmt::Debug).fmt(f)
    }
}

impl FromStr for AlignedU64 {
    type Err = <u64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AlignedU64(u64::from_str(s)?))
    }
}

impl<Rhs: Into<u64>> core::ops::Add<Rhs> for AlignedU64 {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self {
        Self(self.0 + rhs.into())
    }
}

impl<Rhs: Into<u64>> core::ops::AddAssign<Rhs> for AlignedU64 {
    fn add_assign(&mut self, rhs: Rhs) {
        self.0 += rhs.into();
    }
}

impl<Rhs: Into<u64>> core::ops::Sub<Rhs> for AlignedU64 {
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self {
        Self(self.0 - rhs.into())
    }
}

impl<Rhs: Into<u64>> core::ops::SubAssign<Rhs> for AlignedU64 {
    fn sub_assign(&mut self, rhs: Rhs) {
        self.0 -= rhs.into();
    }
}

impl<Rhs: Into<u64>> core::ops::Mul<Rhs> for AlignedU64 {
    type Output = Self;

    fn mul(self, rhs: Rhs) -> Self {
        Self(self.0 * rhs.into())
    }
}

impl<Rhs: Into<u64>> core::ops::MulAssign<Rhs> for AlignedU64 {
    fn mul_assign(&mut self, rhs: Rhs) {
        self.0 *= rhs.into();
    }
}

impl<Rhs: Into<u64>> core::ops::Div<Rhs> for AlignedU64 {
    type Output = Self;

    fn div(self, rhs: Rhs) -> Self {
        Self(self.0 / rhs.into())
    }
}

impl<Rhs: Into<u64>> core::ops::DivAssign<Rhs> for AlignedU64 {
    fn div_assign(&mut self, rhs: Rhs) {
        self.0 /= rhs.into();
    }
}

impl AlignedU64 {
    pub const fn new(v: u64) -> Self {
        Self(v)
    }
    pub fn as_u64(self) -> u64 {
        self.0
    }
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

/// Microseconds since epoch
#[cfg_attr(target_arch = "wasm32", declare)]
pub type Timestamp = AlignedU64;
pub fn get_aligned_timestamp() -> Timestamp {
    get_timestamp().into()
}
/// Microseconds duration
#[cfg_attr(target_arch = "wasm32", declare)]
pub type TimestampDuration = AlignedU64;
/// Request/Response matching id
#[cfg_attr(target_arch = "wasm32", declare)]
pub type OperationId = AlignedU64;
/// Number of bytes
#[cfg_attr(target_arch = "wasm32", declare)]
pub type ByteCount = AlignedU64;
