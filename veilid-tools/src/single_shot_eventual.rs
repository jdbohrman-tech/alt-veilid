use super::*;

pub struct SingleShotEventual<T>
where
    T: Unpin,
{
    eventual: EventualValue<T>,
    drop_value: Option<T>,
}

impl<T> Drop for SingleShotEventual<T>
where
    T: Unpin,
{
    fn drop(&mut self) {
        if let Some(drop_value) = self.drop_value.take() {
            self.eventual.resolve(drop_value);
        }
    }
}

impl<T> SingleShotEventual<T>
where
    T: Unpin,
{
    pub fn new(drop_value: Option<T>) -> Self {
        Self {
            eventual: EventualValue::new(),
            drop_value,
        }
    }

    // Can only call this once, it consumes the eventual
    pub fn resolve(mut self, value: T) -> EventualResolvedFuture<EventualValue<T>> {
        // If we resolve, we don't want to resolve again to the drop value
        self.drop_value = None;
        // Resolve to the specified value
        self.eventual.resolve(value)
    }

    pub fn instance(&self) -> EventualValueFuture<T> {
        self.eventual.instance()
    }
}
