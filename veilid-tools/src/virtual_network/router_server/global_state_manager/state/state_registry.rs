use super::*;
use std::marker::PhantomData;

pub trait State: fmt::Debug + Clone {
    fn id(&self) -> StateId<Self>;
    fn name(&self) -> Option<String>;
    fn debug_name(&self) -> String {
        self.name()
            .unwrap_or_else(|| format!("<{}>", self.id().external_id()))
    }
}

type StateIdInternal = u64;

#[derive(Debug, Clone)]
pub struct StateId<S: State>(pub StateIdInternal, core::marker::PhantomData<S>);
impl<S: State> StateId<S> {
    pub fn new(external_id: u64) -> Self {
        Self(external_id, PhantomData {})
    }

    pub fn external_id(&self) -> u64 {
        self.0
    }
}

impl<S: State> Copy for StateId<S> {}
impl<S: State> PartialEq for StateId<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<S: State> Eq for StateId<S> {}
impl<S: State> PartialOrd for StateId<S> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}
impl<S: State> Ord for StateId<S> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<S: State> core::hash::Hash for StateId<S> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[derive(Debug, Clone)]
pub struct StateRegistry<S: State> {
    state_id_by_name: imbl::HashMap<String, StateIdInternal>,
    state_by_id: imbl::HashMap<StateIdInternal, Option<S>>,
    next_state_id: StateIdInternal,
    free_state_ids: imbl::Vector<StateIdInternal>,
}

impl<S: State> StateRegistry<S> {
    pub fn new() -> Self {
        Self {
            state_id_by_name: imbl::HashMap::new(),
            state_by_id: imbl::HashMap::new(),
            next_state_id: 0,
            free_state_ids: imbl::Vector::new(),
        }
    }

    pub fn allocate_id(&mut self) -> StateId<S> {
        // Allocate new internal id
        let state_id = self.free_state_ids.pop_back().unwrap_or_else(|| {
            let x = self.next_state_id;
            self.next_state_id += 1;
            x
        });

        // Associate with an empty state slot
        self.state_by_id.insert(state_id, None);

        // Return the type-safe wrapped id
        StateId(state_id, PhantomData {})
    }

    pub fn release_id(&mut self, id: StateId<S>) -> GlobalStateManagerResult<()> {
        // Remove id to state mapping
        let Some(old_opt_state) = self.state_by_id.remove(&id.0) else {
            return Err(GlobalStateManagerError::InvalidId(id.external_id()));
        };

        // Release state if it is attached
        if let Some(old_state) = old_opt_state {
            // Release name of state if it is named
            if let Some(name) = old_state.name() {
                self.state_id_by_name
                    .remove(&name)
                    .expect("named states should be registered");
            }
        }

        // Keep old id in the free list
        self.free_state_ids.push_back(id.0);

        Ok(())
    }

    pub fn attach_state(&mut self, state: S) -> GlobalStateManagerResult<()> {
        // Get the id from the state
        let id = state.id();

        // Get the allocator slot
        let Some(opt_state) = self.state_by_id.get_mut(&id.0) else {
            return Err(GlobalStateManagerError::InvalidId(id.external_id()));
        };

        // Ensure the state slot isn't attached already
        if opt_state.is_some() {
            return Err(GlobalStateManagerError::AlreadyAttached);
        }

        // Ensure the name isn't duplicated
        if let Some(name) = state.name() {
            if self.state_id_by_name.contains_key(&name) {
                return Err(GlobalStateManagerError::DuplicateName(name));
            }
            // Register the named state
            assert!(
                self.state_id_by_name.insert(name, id.0).is_none(),
                "should not have a duplicated name here"
            );
        }

        // Attach the state to the state slot
        *opt_state = Some(state);

        Ok(())
    }

    pub fn detach_state(&mut self, id: StateId<S>) -> GlobalStateManagerResult<S> {
        // Get the allocator slot
        let Some(opt_state) = self.state_by_id.get_mut(&id.0) else {
            return Err(GlobalStateManagerError::InvalidId(id.external_id()));
        };

        // Take the state out of the slot and ensure the state slot isn't detached already
        let Some(state) = opt_state.take() else {
            return Err(GlobalStateManagerError::NotAttached);
        };

        // Release the name if it exists
        if let Some(name) = state.name() {
            let dead_name_id = self
                .state_id_by_name
                .remove(&name)
                .expect("name should be registered");
            assert_eq!(dead_name_id, id.0, "name id and state id should match");
        }

        Ok(state)
    }

    pub fn get_state(&self, id: StateId<S>) -> GlobalStateManagerResult<S> {
        // Get the allocator slot
        let Some(opt_state) = self.state_by_id.get(&id.0) else {
            return Err(GlobalStateManagerError::InvalidId(id.external_id()));
        };
        let Some(state) = opt_state else {
            return Err(GlobalStateManagerError::NotAttached);
        };
        Ok(state.clone())
    }

    pub fn set_state(&mut self, state: S) {
        self.state_by_id.insert(state.id().0, Some(state));
    }

    pub fn get_state_id_by_name(&self, name: &str) -> Option<StateId<S>> {
        // Get the id associated with this name
        let id = self.state_id_by_name.get(name)?;
        Some(StateId::new(*id))
    }
}

impl<S: State> Default for StateRegistry<S> {
    fn default() -> Self {
        Self::new()
    }
}
