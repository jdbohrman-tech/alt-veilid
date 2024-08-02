/// Microseconds since epoch
use super::*;

aligned_u64_type!(Timestamp);

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", debug_ts(self.as_u64()))
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", debug_ts(self.as_u64()))
    }
}

impl core::ops::Add<TimestampDuration> for Timestamp {
    type Output = Self;

    fn add(self, rhs: TimestampDuration) -> Self {
        Self(self.0 + rhs.as_u64())
    }
}

impl core::ops::AddAssign<TimestampDuration> for Timestamp {
    fn add_assign(&mut self, rhs: TimestampDuration) {
        self.0 += rhs.as_u64();
    }
}

impl core::ops::Sub<Timestamp> for Timestamp {
    type Output = TimestampDuration;

    fn sub(self, rhs: Timestamp) -> TimestampDuration {
        TimestampDuration::new(self.0 - rhs.as_u64())
    }
}

impl core::ops::Sub<TimestampDuration> for Timestamp {
    type Output = Timestamp;

    fn sub(self, rhs: TimestampDuration) -> Timestamp {
        Timestamp(self.0 - rhs.as_u64())
    }
}

impl core::ops::SubAssign<TimestampDuration> for Timestamp {
    fn sub_assign(&mut self, rhs: TimestampDuration) {
        self.0 -= rhs.as_u64();
    }
}

impl Timestamp {
    pub fn now() -> Timestamp {
        Timestamp::new(get_timestamp())
    }

    pub fn saturating_sub(self, rhs: Self) -> TimestampDuration {
        TimestampDuration::new(self.0.saturating_sub(rhs.0))
    }
}
