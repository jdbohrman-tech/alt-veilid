use crate::*;

struct BlockStoreInner {
    //
}

#[derive(Clone)]
pub struct BlockStore {
    config: VeilidConfig,
    inner: Arc<Mutex<BlockStoreInner>>,
}

impl BlockStore {
    fn new_inner() -> BlockStoreInner {
        BlockStoreInner {}
    }
    pub fn new(config: VeilidConfig) -> Self {
        Self {
            config,
            inner: Arc::new(Mutex::new(Self::new_inner())),
        }
    }

    pub async fn init(&self) -> EyreResult<()> {
        Ok(())
    }

    pub async fn terminate(&self) {}
}
