mod dflt;
mod smpl;

use super::*;

pub use dflt::*;
pub use smpl::*;

/// Enum over all the supported DHT Schemas
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(from_wasm_abi))]
pub enum DHTSchema {
    DFLT(DHTSchemaDFLT),
    SMPL(DHTSchemaSMPL),
}

impl DHTSchema {
    pub fn dflt(o_cnt: u16) -> DHTSchema {
        DHTSchema::DFLT(DHTSchemaDFLT { o_cnt })
    }
    pub fn smpl(o_cnt: u16, members: Vec<DHTSchemaSMPLMember>) -> DHTSchema {
        DHTSchema::SMPL(DHTSchemaSMPL { o_cnt, members })
    }

    /// Build the data representation of the schema
    pub fn compile(&self) -> Vec<u8> {
        match self {
            DHTSchema::DFLT(d) => d.compile(),
            DHTSchema::SMPL(s) => s.compile(),
        }
    }

    /// Get the number of subkeys this schema allocates
    pub fn subkey_count(&self) -> usize {
        match self {
            DHTSchema::DFLT(d) => d.subkey_count(),
            DHTSchema::SMPL(s) => s.subkey_count(),
        }
    }

    /// Get the data size of this schema beyond the size of the structure itself
    pub fn data_size(&self) -> usize {
        match self {
            DHTSchema::DFLT(d) => d.data_size(),
            DHTSchema::SMPL(s) => s.data_size(),
        }
    }

    /// Check a subkey value data against the schema
    pub fn check_subkey_value_data(
        &self,
        owner: &PublicKey,
        subkey: ValueSubkey,
        value_data: &ValueData,
    ) -> bool {
        match self {
            DHTSchema::DFLT(d) => d.check_subkey_value_data(owner, subkey, value_data),
            DHTSchema::SMPL(s) => s.check_subkey_value_data(owner, subkey, value_data),
        }
    }
}

impl Default for DHTSchema {
    fn default() -> Self {
        Self::dflt(1)
    }
}

impl TryFrom<&[u8]> for DHTSchema {
    type Error = VeilidAPIError;
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        if b.len() < 4 {
            apibail_generic!("invalid size");
        }
        let fcc: [u8; 4] = b[0..4].try_into().unwrap();
        match fcc {
            DHTSchemaDFLT::FCC => Ok(DHTSchema::DFLT(DHTSchemaDFLT::try_from(b)?)),
            DHTSchemaSMPL::FCC => Ok(DHTSchema::SMPL(DHTSchemaSMPL::try_from(b)?)),
            _ => {
                apibail_generic!("unknown fourcc");
            }
        }
    }
}
