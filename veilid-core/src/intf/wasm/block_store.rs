use crate::*;

struct BlockStoreInner {
    //
}

impl fmt::Debug for BlockStoreInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockStoreInner").finish()
    }
}

#[derive(Debug)]
pub struct BlockStore {
    registry: VeilidComponentRegistry,
    inner: Mutex<BlockStoreInner>,
}

impl_veilid_component!(BlockStore);

impl BlockStore {
    fn new_inner() -> BlockStoreInner {
        BlockStoreInner {}
    }
    pub fn new(registry: VeilidComponentRegistry) -> Self {
        Self {
            registry,
            inner: Mutex::new(Self::new_inner()),
        }
    }

    async fn init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    async fn terminate_async(&self) {}
}
