use super::*;
use core::hash::Hash;

pub struct AsyncTagLockGuard<T>
where
    T: Hash + Eq + Clone,
{
    table: AsyncTagLockTable<T>,
    tag: T,
    _guard: AsyncMutexGuardArc<()>,
}

impl<T> Drop for AsyncTagLockGuard<T>
where
    T: Hash + Eq + Clone,
{
    fn drop(&mut self) {
        let mut inner = self.table.inner.lock();
        // Inform the table we're dropping this guard
        let waiters = {
            // Get the table entry, it must exist since we have a guard locked
            let entry = inner.table.get_mut(&self.tag).unwrap();
            // Decrement the number of waiters
            entry.waiters -= 1;
            // Return the number of waiters left
            entry.waiters
        };
        // If there are no waiters left, we remove the tag from the table
        if waiters == 0 {
            inner.table.remove(&self.tag).unwrap();
        }
        // Proceed with releasing _guard, which may cause some concurrent tag lock to acquire
    }
}

#[derive(Clone)]
struct AsyncTagLockTableEntry {
    mutex: Arc<AsyncMutex<()>>,
    waiters: usize,
}

struct AsyncTagLockTableInner<T>
where
    T: Hash + Eq + Clone,
{
    table: HashMap<T, AsyncTagLockTableEntry>,
}

#[derive(Clone)]
pub struct AsyncTagLockTable<T>
where
    T: Hash + Eq + Clone,
{
    inner: Arc<Mutex<AsyncTagLockTableInner<T>>>,
}

impl<T> fmt::Debug for AsyncTagLockTable<T>
where
    T: Hash + Eq + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncTagLockTable").finish()
    }
}

impl<T> AsyncTagLockTable<T>
where
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(AsyncTagLockTableInner {
                table: HashMap::new(),
            })),
        }
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.lock();
        inner.table.len()
    }

    pub async fn lock_tag(&self, tag: T) -> AsyncTagLockGuard<T> {
        // Get or create a tag lock entry
        let mutex = {
            let mut inner = self.inner.lock();

            // See if this tag is in the table
            // and if not, add a new mutex for this tag
            let entry = inner
                .table
                .entry(tag.clone())
                .or_insert_with(|| AsyncTagLockTableEntry {
                    mutex: Arc::new(AsyncMutex::new(())),
                    waiters: 0,
                });

            // Increment the number of waiters
            entry.waiters += 1;

            // Return the mutex associated with the tag
            entry.mutex.clone()

            // Drop the table guard
        };

        // Lock the tag lock
        let guard;
        cfg_if! {
            if #[cfg(feature="rt-tokio")] {
                // tokio version
                guard = mutex.lock_owned().await;
            } else {
                // async_std and wasm async_mutex version
                guard = mutex.lock_arc().await;
            }
        }

        // Return the locked guard
        AsyncTagLockGuard {
            table: self.clone(),
            tag,
            _guard: guard,
        }
    }
}
