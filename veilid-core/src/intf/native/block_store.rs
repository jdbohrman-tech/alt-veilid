use crate::*;

impl_veilid_log_facility!("bstore");

struct BlockStoreInner {
    //
}

impl fmt::Debug for BlockStoreInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockStoreInner").finish()
    }
}

#[derive(Debug)]
#[must_use]
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

    #[instrument(level = "debug", skip(self))]
    async fn init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip(self), err)]
    async fn post_init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    async fn pre_terminate_async(&self) {}

    #[instrument(level = "debug", skip(self))]
    async fn terminate_async(&self) {}
}
