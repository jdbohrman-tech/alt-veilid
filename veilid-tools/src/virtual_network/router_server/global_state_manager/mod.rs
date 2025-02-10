mod address_pool;
mod global_state_manager_inner;
mod state;

use super::*;

use address_pool::*;
use global_state_manager_inner::*;
use state::*;

#[derive(Debug)]
struct Machine {}

#[derive(Debug)]
struct GlobalStateManagerUnlockedInner {}

#[derive(Debug, Clone, ThisError, PartialEq, Eq)]
pub enum GlobalStateManagerError {
    #[error("Invalid id: {0}")]
    InvalidId(u64),
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Invalid prefix: {0}")]
    InvalidPrefix(u8),
    #[error("Already attached")]
    AlreadyAttached,
    #[error("Not attached")]
    NotAttached,
    #[error("Duplicate name: {0}")]
    DuplicateName(String),
    #[error("Profile complete: {0}")]
    ProfileComplete(String),
    #[error("Template complete: {0}")]
    TemplateComplete(String),
    #[error("Network complete: {0}")]
    NetworkComplete(String),
    #[error("Blueprint complete: {0}")]
    BlueprintComplete(String),
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    #[error("Machine not found: {0}")]
    MachineNotFound(String),
    #[error("Network not found: {0}")]
    NetworkNotFound(String),
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    #[error("Blueprint not found: {0}")]
    BlueprintNotFound(String),
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Allocation not found: {0}")]
    AllocationNotFound(String),
    #[error("No default model")]
    NoDefaultModel,
    #[error("No default network")]
    NoDefaultNetwork,
    #[error("No default pool")]
    NoDefaultPool,
    #[error("No allocation available")]
    NoAllocation,
    #[error("Resource in use: {0}")]
    ResourceInUse(String),
    #[error("Invalid gateway")]
    InvalidGateway,
}

pub type GlobalStateManagerResult<T> = Result<T, GlobalStateManagerError>;

#[derive(Debug, Clone)]
pub struct GlobalStateManager {
    unlocked_inner: Arc<GlobalStateManagerUnlockedInner>,
    inner: Arc<Mutex<GlobalStateManagerInner>>,
}

impl GlobalStateManager {
    ///////////////////////////////////////////////////////////
    /// Public Interface
    pub fn new() -> Self {
        let unlocked_inner = Arc::new(GlobalStateManagerUnlockedInner {});
        Self {
            inner: Arc::new(Mutex::new(GlobalStateManagerInner::new(
                unlocked_inner.clone(),
            ))),
            unlocked_inner,
        }
    }

    pub fn execute_config(&self, cfg: config::Config) -> GlobalStateManagerResult<()> {
        let mut inner = self.inner.lock();
        let saved_state = (*inner).clone();
        match inner.execute_config(cfg) {
            Ok(v) => Ok(v),
            Err(e) => {
                *inner = saved_state;
                Err(e)
            }
        }
    }

    pub fn allocate(&self, profile: String) -> GlobalStateManagerResult<MachineId> {
        let mut inner = self.inner.lock();
        let saved_state = (*inner).clone();
        match inner.allocate(profile) {
            Ok(v) => Ok(v),
            Err(e) => {
                *inner = saved_state;
                Err(e)
            }
        }
    }

    pub fn release(&self, machine_id: MachineId) -> GlobalStateManagerResult<()> {
        let mut inner = self.inner.lock();
        let saved_state = (*inner).clone();
        match inner.release(machine_id) {
            Ok(v) => Ok(v),
            Err(e) => {
                *inner = saved_state;
                Err(e)
            }
        }
    }
}
