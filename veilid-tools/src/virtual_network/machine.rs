use super::*;

pub type MachineId = u64;

#[derive(Debug, Clone)]
pub struct Machine {
    pub router_client: RouterClient,
    pub id: MachineId,
}

pub fn set_default_machine(machine: Machine) {
    *DEFAULT_MACHINE.lock() = Some(machine);
}

pub fn take_default_machine() -> Option<Machine> {
    DEFAULT_MACHINE.lock().take()
}

pub fn default_machine() -> Option<Machine> {
    (*DEFAULT_MACHINE.lock()).clone()
}

static DEFAULT_MACHINE: Mutex<Option<Machine>> = Mutex::new(None);
