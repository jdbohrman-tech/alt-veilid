use super::*;

pub type PinBox<T> = Pin<Box<T>>;
pub type PinBoxFuture<'a, T> = PinBox<dyn Future<Output = T> + Send + 'a>;
pub type PinBoxFutureStatic<T> = PinBoxFuture<'static, T>;

// Pins a future to the heap and returns a concrete boxed future
// Moves the future to the heap from the caller's stack
// May allocate the future on the stack first and then move it
#[macro_export]
macro_rules! pin_future {
    ($call: expr) => {
        Box::pin($call)
    };
}

// Pins a future to the heap inside a closure and returns a concrete boxed future
// Keeps the future off the calling function's stack completely
// Closure is still on the stack, but smaller than the future will be
#[macro_export]
macro_rules! pin_future_closure {
    ($call: expr) => {
        (|| Box::pin($call))()
    };
}

// Pins a future to the heap and returns a dynamic boxed future
// Moves the future to the heap from the caller's stack
// May allocate the future on the stack first and then move it
#[macro_export]
macro_rules! pin_dyn_future {
    ($call: expr) => {
        Box::pin($call) as PinBoxFuture<_>
    };
}

// Pins a future to the heap inside a closure and returns a dynamic boxed future
// Keeps the future off the calling function's stack completely
// Closure is still on the stack, but smaller than the future will be
#[macro_export]
macro_rules! pin_dyn_future_closure {
    ($call: expr) => {
        (|| Box::pin($call) as PinBoxFuture<_>)()
    };
}
