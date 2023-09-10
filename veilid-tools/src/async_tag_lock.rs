use super::*;

use core::fmt::Debug;
use core::hash::Hash;

#[derive(Debug)]
pub struct AsyncTagLockGuard<T>
where
    T: Hash + Eq + Clone + Debug,
{
    table: AsyncTagLockTable<T>,
    tag: T,
    _guard: AsyncMutexGuardArc<()>,
}

impl<T> AsyncTagLockGuard<T>
where
    T: Hash + Eq + Clone + Debug,
{
    fn new(table: AsyncTagLockTable<T>, tag: T, guard: AsyncMutexGuardArc<()>) -> Self {
        Self {
            table,
            tag,
            _guard: guard,
        }
    }
}

impl<T> Drop for AsyncTagLockGuard<T>
where
    T: Hash + Eq + Clone + Debug,
{
    fn drop(&mut self) {
        let mut inner = self.table.inner.lock();
        // Inform the table we're dropping this guard
        let guards = {
            // Get the table entry, it must exist since we have a guard locked
            let entry = inner.table.get_mut(&self.tag).unwrap();
            // Decrement the number of guards
            entry.guards -= 1;
            // Return the number of guards left
            entry.guards
        };
        // If there are no guards left, we remove the tag from the table
        if guards == 0 {
            inner.table.remove(&self.tag).unwrap();
        }
        // Proceed with releasing _guard, which may cause some concurrent tag lock to acquire
    }
}

#[derive(Clone, Debug)]
struct AsyncTagLockTableEntry {
    mutex: Arc<AsyncMutex<()>>,
    guards: usize,
}

struct AsyncTagLockTableInner<T>
where
    T: Hash + Eq + Clone + Debug,
{
    table: HashMap<T, AsyncTagLockTableEntry>,
}

#[derive(Clone)]
pub struct AsyncTagLockTable<T>
where
    T: Hash + Eq + Clone + Debug,
{
    inner: Arc<Mutex<AsyncTagLockTableInner<T>>>,
}

impl<T> fmt::Debug for AsyncTagLockTable<T>
where
    T: Hash + Eq + Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncTagLockTable").finish()
    }
}

impl<T> AsyncTagLockTable<T>
where
    T: Hash + Eq + Clone + Debug,
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
                    guards: 0,
                });

            // Increment the number of guards
            entry.guards += 1;

            // Return the mutex associated with the tag
            entry.mutex.clone()

            // Drop the table guard
        };

        // Lock the tag lock
        let guard = asyncmutex_lock_arc!(mutex);

        // Return the locked guard
        AsyncTagLockGuard::new(self.clone(), tag, guard)
    }

    pub fn try_lock_tag(&self, tag: T) -> Option<AsyncTagLockGuard<T>> {
        // Get or create a tag lock entry
        let mut inner = self.inner.lock();

        // See if this tag is in the table
        // and if not, add a new mutex for this tag
        let entry = inner.table.entry(tag.clone());

        // Lock the tag lock
        let guard = match entry {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let e = o.get_mut();
                let guard = asyncmutex_try_lock_arc!(e.mutex)?;
                e.guards += 1;
                guard
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                let mutex = Arc::new(AsyncMutex::new(()));
                let guard = asyncmutex_try_lock_arc!(mutex)?;
                v.insert(AsyncTagLockTableEntry { mutex, guards: 1 });
                guard
            }
        };
        // Return guard
        Some(AsyncTagLockGuard::new(self.clone(), tag, guard))
    }
}
