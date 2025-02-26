use super::*;

/// Aligned u64 type generator
///
/// Required on 32-bit platforms for serialization because Rust aligns u64 on 4 byte boundaries.
/// Some zero-copy serialization frameworks also want 8-byte alignment.
/// Supports serializing to string for JSON as well, since JSON can't handle 64-bit numbers to Javascript.

macro_rules! aligned_u64_type {
    ($name:ident) => {
        #[derive(
            Clone,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Copy,
            Hash,
            Serialize,
            Deserialize,
            JsonSchema,
        )]
        #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
        #[repr(C, align(8))]
        #[serde(transparent)]
        #[must_use]
        pub struct $name(
            #[serde(with = "as_human_string")]
            #[schemars(with = "String")]
            #[cfg_attr(
                all(target_arch = "wasm32", target_os = "unknown"),
                tsify(type = "string")
            )]
            u64,
        );

        impl From<u64> for $name {
            fn from(v: u64) -> Self {
                $name(v)
            }
        }
        impl From<$name> for u64 {
            fn from(v: $name) -> Self {
                v.0
            }
        }

        impl $name {
            pub const fn new(v: u64) -> Self {
                Self(v)
            }
            #[must_use]
            pub fn as_u64(self) -> u64 {
                self.0
            }
        }
    };
}

macro_rules! aligned_u64_type_default_math_impl {
    ($name:ident) => {
        impl<Rhs: Into<u64>> core::ops::Add<Rhs> for $name {
            type Output = Self;

            fn add(self, rhs: Rhs) -> Self {
                Self(self.0 + rhs.into())
            }
        }

        impl<Rhs: Into<u64>> core::ops::AddAssign<Rhs> for $name {
            fn add_assign(&mut self, rhs: Rhs) {
                self.0 += rhs.into();
            }
        }

        impl<Rhs: Into<u64>> core::ops::Sub<Rhs> for $name {
            type Output = Self;

            fn sub(self, rhs: Rhs) -> Self {
                Self(self.0 - rhs.into())
            }
        }

        impl<Rhs: Into<u64>> core::ops::SubAssign<Rhs> for $name {
            fn sub_assign(&mut self, rhs: Rhs) {
                self.0 -= rhs.into();
            }
        }

        impl<Rhs: Into<u64>> core::ops::Mul<Rhs> for $name {
            type Output = Self;

            fn mul(self, rhs: Rhs) -> Self {
                Self(self.0 * rhs.into())
            }
        }

        impl<Rhs: Into<u64>> core::ops::MulAssign<Rhs> for $name {
            fn mul_assign(&mut self, rhs: Rhs) {
                self.0 *= rhs.into();
            }
        }

        impl<Rhs: Into<u64>> core::ops::Div<Rhs> for $name {
            type Output = Self;

            fn div(self, rhs: Rhs) -> Self {
                Self(self.0 / rhs.into())
            }
        }

        impl<Rhs: Into<u64>> core::ops::DivAssign<Rhs> for $name {
            fn div_assign(&mut self, rhs: Rhs) {
                self.0 /= rhs.into();
            }
        }

        impl $name {
            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }
        }
    };
}

macro_rules! aligned_u64_type_default_display_impl {
    ($name:ident) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                (&self.0 as &dyn fmt::Display).fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = <u64 as FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(u64::from_str(s)?))
            }
        }
    };
}

macro_rules! aligned_u64_type_default_debug_impl {
    ($name:ident) => {
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                (&self.0 as &dyn fmt::Debug).fmt(f)
            }
        }
    };
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

aligned_u64_type!(OperationId);
aligned_u64_type_default_display_impl!(OperationId);
aligned_u64_type_default_debug_impl!(OperationId);

aligned_u64_type!(ByteCount);
aligned_u64_type_default_display_impl!(ByteCount);
aligned_u64_type_default_debug_impl!(ByteCount);
aligned_u64_type_default_math_impl!(ByteCount);

aligned_u64_type!(AlignedU64);
aligned_u64_type_default_display_impl!(AlignedU64);
aligned_u64_type_default_debug_impl!(AlignedU64);
aligned_u64_type_default_math_impl!(AlignedU64);
