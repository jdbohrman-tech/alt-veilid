use super::*;
use num_traits::{PrimInt, Unsigned};

impl_veilid_log_facility!("stor");

#[derive(ThisError, Debug, Clone, Copy, Eq, PartialEq)]
pub enum LimitError {
    #[error("limit overflow")]
    OverLimit,
}

#[derive(ThisError, Debug, Clone, Copy, Eq, PartialEq)]
pub enum NumericError {
    #[error("numeric overflow")]
    Overflow,
    #[error("numeric underflow")]
    Underflow,
}

#[derive(Clone)]
pub struct LimitedSize<T: PrimInt + Unsigned + fmt::Display + fmt::Debug> {
    registry: VeilidComponentRegistry,
    description: String,
    value: T,
    limit: Option<T>,
    uncommitted_value: Option<T>,
}

impl<T> VeilidComponentRegistryAccessor for LimitedSize<T>
where
    T: PrimInt + Unsigned + fmt::Display + fmt::Debug,
{
    fn registry(&self) -> VeilidComponentRegistry {
        self.registry.clone()
    }
}

impl<T> fmt::Debug for LimitedSize<T>
where
    T: PrimInt + Unsigned + fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LimitedSize")
            //.field("registry", &self.registry)
            .field("description", &self.description)
            .field("value", &self.value)
            .field("limit", &self.limit)
            .field("uncommitted_value", &self.uncommitted_value)
            .finish()
    }
}

impl<T: PrimInt + Unsigned + fmt::Display + fmt::Debug> LimitedSize<T> {
    pub fn new(
        registry: VeilidComponentRegistry,
        description: &str,
        value: T,
        limit: Option<T>,
    ) -> Self {
        Self {
            registry,
            description: description.to_owned(),
            value,
            limit,
            uncommitted_value: None,
        }
    }

    fn current_value(&self) -> T {
        self.uncommitted_value.unwrap_or(self.value)
    }

    pub fn set(&mut self, new_value: T) {
        self.uncommitted_value = Some(new_value);
    }

    pub fn add(&mut self, v: T) -> Result<T, NumericError> {
        let current_value = self.current_value();
        let max_v = T::max_value() - current_value;
        if v > max_v {
            return Err(NumericError::Overflow);
        }
        let new_value = current_value + v;
        self.uncommitted_value = Some(new_value);
        Ok(new_value)
    }
    pub fn sub(&mut self, v: T) -> Result<T, NumericError> {
        let current_value = self.current_value();
        let max_v = current_value - T::min_value();
        if v > max_v {
            return Err(NumericError::Underflow);
        }
        let new_value = current_value - v;
        self.uncommitted_value = Some(new_value);
        Ok(new_value)
    }
    pub fn saturating_sub(&mut self, mut v: T) -> T {
        let current_value = self.current_value();
        let max_v = current_value - T::min_value();
        if v > max_v {
            veilid_log!(self debug "Numeric underflow ({})", self.description);
            v = max_v;
        }
        let new_value = current_value - v;
        self.uncommitted_value = Some(new_value);
        new_value
    }

    pub fn check_limit(&self) -> bool {
        if let Some(uncommitted_value) = self.uncommitted_value {
            if let Some(limit) = self.limit {
                if uncommitted_value > limit {
                    return false;
                }
            }
        }
        true
    }

    pub fn commit(&mut self) -> Result<T, LimitError> {
        if let Some(uncommitted_value) = self.uncommitted_value {
            if let Some(limit) = self.limit {
                if uncommitted_value > limit {
                    veilid_log!(self debug "Commit over limit failed ({}): {} > {}", self.description, uncommitted_value, limit);
                    return Err(LimitError::OverLimit);
                }
            }
            veilid_log!(self trace "Commit ({}): {} => {}", self.description, self.value, uncommitted_value);
            self.uncommitted_value = None;
            self.value = uncommitted_value;
        }
        Ok(self.value)
    }

    pub fn rollback(&mut self) -> T {
        if let Some(uv) = self.uncommitted_value.take() {
            veilid_log!(self trace "Rollback ({}): {} (drop {})", self.description, self.value, uv);
        }
        self.value
    }

    pub fn get(&self) -> T {
        self.value
    }
}
