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
    pub fn dflt(o_cnt: u16) -> VeilidAPIResult<DHTSchema> {
        Ok(DHTSchema::DFLT(DHTSchemaDFLT::new(o_cnt)?))
    }
    pub fn smpl(o_cnt: u16, members: Vec<DHTSchemaSMPLMember>) -> VeilidAPIResult<DHTSchema> {
        Ok(DHTSchema::SMPL(DHTSchemaSMPL::new(o_cnt, members)?))
    }

    /// Validate the data representation
    pub fn validate(&self) -> VeilidAPIResult<()> {
        match self {
            DHTSchema::DFLT(d) => d.validate(),
            DHTSchema::SMPL(s) => s.validate(),
        }
    }

    /// Build the data representation of the schema
    pub fn compile(&self) -> Vec<u8> {
        match self {
            DHTSchema::DFLT(d) => d.compile(),
            DHTSchema::SMPL(s) => s.compile(),
        }
    }

    /// Get maximum subkey number for this schema
    pub fn max_subkey(&self) -> ValueSubkey {
        match self {
            DHTSchema::DFLT(d) => d.max_subkey(),
            DHTSchema::SMPL(s) => s.max_subkey(),
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

    /// Check if a key is a schema member
    pub fn is_member(&self, key: &PublicKey) -> bool {
        match self {
            DHTSchema::DFLT(d) => d.is_member(key),
            DHTSchema::SMPL(s) => s.is_member(key),
        }
    }

    /// Truncate a subkey range set to the schema
    /// Optionally also trim to maximum number of subkeys in the range
    pub fn truncate_subkeys(
        &self,
        subkeys: &ValueSubkeyRangeSet,
        opt_max_subkey_len: Option<usize>,
    ) -> ValueSubkeyRangeSet {
        // Get number of subkeys from schema and trim to the bounds of the schema
        let in_schema_subkeys =
            subkeys.intersect(&ValueSubkeyRangeSet::single_range(0, self.max_subkey()));

        // Cap the number of total subkeys being inspected to the amount we can send across the wire
        if let Some(max_subkey_len) = opt_max_subkey_len {
            if let Some(nth_subkey) = in_schema_subkeys.nth_subkey(max_subkey_len) {
                in_schema_subkeys.difference(&ValueSubkeyRangeSet::single_range(
                    nth_subkey,
                    ValueSubkey::MAX,
                ))
            } else {
                in_schema_subkeys
            }
        } else {
            in_schema_subkeys
        }
    }
}

impl Default for DHTSchema {
    fn default() -> Self {
        Self::dflt(1).unwrap()
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
