use super::*;

/// Default DHT Schema (DFLT)
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(from_wasm_abi))]
pub struct DHTSchemaDFLT {
    /// Owner subkey count
    pub o_cnt: u16,
}

impl DHTSchemaDFLT {
    pub const FCC: [u8; 4] = *b"DFLT";
    pub const FIXED_SIZE: usize = 6;

    /// Build the data representation of the schema
    pub fn compile(&self) -> Vec<u8> {
        let mut out = Vec::<u8>::with_capacity(Self::FIXED_SIZE);
        // kind
        out.extend_from_slice(&Self::FCC);
        // o_cnt
        out.extend_from_slice(&self.o_cnt.to_le_bytes());
        out
    }

    /// Get the number of subkeys this schema allocates
    pub fn subkey_count(&self) -> usize {
        self.o_cnt as usize
    }
    /// Get the data size of this schema beyond the size of the structure itself
    pub fn data_size(&self) -> usize {
        0
    }

    /// Check a subkey value data against the schema
    pub fn check_subkey_value_data(
        &self,
        owner: &PublicKey,
        subkey: ValueSubkey,
        value_data: &ValueData,
    ) -> bool {
        let subkey = subkey as usize;

        // Check if subkey is in owner range
        if subkey < (self.o_cnt as usize) {
            // Check value data has valid writer
            if value_data.writer() == owner {
                return true;
            }
            // Wrong writer
            return false;
        }

        // Subkey out of range
        false
    }
}

impl TryFrom<&[u8]> for DHTSchemaDFLT {
    type Error = VeilidAPIError;
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        if b.len() != Self::FIXED_SIZE {
            apibail_generic!("invalid size");
        }
        if b[0..4] != Self::FCC {
            apibail_generic!("wrong fourcc");
        }

        let o_cnt = u16::from_le_bytes(b[4..6].try_into().map_err(VeilidAPIError::internal)?);

        Ok(Self { o_cnt })
    }
}
