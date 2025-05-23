use std::marker::PhantomData;

use super::*;

/// Pointer-identity hashing for unique objects
/// Considers the `===` identity equals rather than the `==` Eq/PartialEq equals for objects
/// that are guaranteed to be fixed in memory
pub struct HashAtom<'a, T> {
    val: usize,
    is_arc: bool,
    _phantom: &'a PhantomData<T>,
}

impl<T> Drop for HashAtom<'_, T> {
    fn drop(&mut self) {
        if self.is_arc {
            unsafe {
                let ptr = self.val as *const T;
                Arc::from_raw(ptr);
            };
        }
    }
}

impl<T> core::fmt::Debug for HashAtom<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashAtom").field("val", &self.val).finish()
    }
}

impl<'a, T> From<Pin<&'a T>> for HashAtom<'a, T> {
    fn from(value: Pin<&'a T>) -> Self {
        Self {
            val: (value.get_ref() as *const T) as usize,
            is_arc: false,
            _phantom: &PhantomData {},
        }
    }
}

impl<'a, T> From<Pin<&'a mut T>> for HashAtom<'a, T> {
    fn from(value: Pin<&'a mut T>) -> Self {
        Self {
            val: (value.as_ref().get_ref() as *const T) as usize,
            is_arc: false,
            _phantom: &PhantomData {},
        }
    }
}

impl<T> From<Arc<T>> for HashAtom<'_, T> {
    fn from(value: Arc<T>) -> Self {
        let val = {
            let ptr = Arc::into_raw(value);
            ptr as usize
        };
        Self {
            val,
            is_arc: true,
            _phantom: &PhantomData {},
        }
    }
}

impl<T> PartialEq for HashAtom<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T> Eq for HashAtom<'_, T> {}

impl<T> core::hash::Hash for HashAtom<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
