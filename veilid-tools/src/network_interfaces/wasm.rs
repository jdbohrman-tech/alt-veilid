use super::*;
use std::io;

pub struct PlatformSupportWasm {}

impl PlatformSupportWasm {
    pub fn new() -> Self {
        PlatformSupportWasm {}
    }

    pub async fn get_interfaces(
        &mut self,
        interfaces: &mut BTreeMap<String, NetworkInterface>,
    ) -> io::Result<()> {
        interfaces.clear();
        Ok(())
    }
}
