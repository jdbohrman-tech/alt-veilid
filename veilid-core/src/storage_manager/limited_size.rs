use super::*;
use num_traits::{PrimInt, Unsigned};

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

#[derive(Debug, Clone)]
pub struct LimitedSize<T: PrimInt + Unsigned + fmt::Display + fmt::Debug> {
    description: String,
    value: T,
    limit: Option<T>,
    uncommitted_value: Option<T>,
}

impl<T: PrimInt + Unsigned + fmt::Display + fmt::Debug> LimitedSize<T> {
    pub fn new(description: &str, value: T, limit: Option<T>) -> Self {
        Self {
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
            log_stor!(debug "Numeric underflow ({})", self.description);
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
                    log_stor!(debug "Commit over limit failed ({}): {} > {}", self.description, uncommitted_value, limit);
                    return Err(LimitError::OverLimit);
                }
            }
            log_stor!(debug "Commit ({}): {} => {}", self.description, self.value, uncommitted_value);
            self.uncommitted_value = None;
            self.value = uncommitted_value;
        }
        Ok(self.value)
    }

    pub fn rollback(&mut self) -> T {
        if let Some(uv) = self.uncommitted_value.take() {
            log_stor!(debug "Rollback ({}): {} (drop {})", self.description, self.value, uv);
        }
        return self.value;
    }

    pub fn get(&self) -> T {
        return self.value;
    }
}
