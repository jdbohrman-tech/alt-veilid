use crate::*;

struct BlockStoreInner {
    //
}

#[derive(Clone)]
pub struct BlockStore {
    event_bus: EventBus,
    config: VeilidConfig,
    inner: Arc<Mutex<BlockStoreInner>>,
}

impl BlockStore {
    fn new_inner() -> BlockStoreInner {
        BlockStoreInner {}
    }
    pub fn new(event_bus: EventBus, config: VeilidConfig) -> Self {
        Self {
            event_bus,
            config,
            inner: Arc::new(Mutex::new(Self::new_inner())),
        }
    }

    pub async fn init(&self) -> EyreResult<()> {
        // Ensure permissions are correct
        // ensure_file_private_owner(&dbpath)?;

        Ok(())
    }

    pub async fn terminate(&self) {}
}
