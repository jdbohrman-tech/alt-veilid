use super::*;
use crate::storage_manager::{MAX_RECORD_DATA_SIZE, MAX_SUBKEY_SIZE};

/// Default DHT Schema (DFLT)
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi)
)]
#[must_use]
pub struct DHTSchemaDFLT {
    /// Owner subkey count
    o_cnt: u16,
}

impl DHTSchemaDFLT {
    pub const FCC: [u8; 4] = *b"DFLT";
    pub const FIXED_SIZE: usize = 6;

    /// Make a schema
    pub fn new(o_cnt: u16) -> VeilidAPIResult<Self> {
        let out = Self { o_cnt };
        out.validate()?;
        Ok(out)
    }

    /// Validate the data representation
    pub fn validate(&self) -> VeilidAPIResult<()> {
        if self.o_cnt == 0 {
            apibail_invalid_argument!("must have at least one subkey", "o_cnt", self.o_cnt);
        }
        Ok(())
    }

    /// Get the owner subkey count
    #[must_use]
    pub fn o_cnt(&self) -> u16 {
        self.o_cnt
    }

    /// Build the data representation of the schema
    #[must_use]
    pub fn compile(&self) -> Vec<u8> {
        let mut out = Vec::<u8>::with_capacity(Self::FIXED_SIZE);
        // kind
        out.extend_from_slice(&Self::FCC);
        // o_cnt
        out.extend_from_slice(&self.o_cnt.to_le_bytes());
        out
    }

    /// Get the maximum subkey this schema allocates
    #[must_use]
    pub fn max_subkey(&self) -> ValueSubkey {
        self.o_cnt as ValueSubkey - 1
    }
    /// Get the data size of this schema beyond the size of the structure itself
    #[must_use]
    pub fn data_size(&self) -> usize {
        0
    }

    /// Check a subkey value data against the schema
    #[must_use]
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
                let max_value_len =
                    usize::min(MAX_SUBKEY_SIZE, MAX_RECORD_DATA_SIZE / self.o_cnt as usize);

                // Ensure value size is within additional limit
                if value_data.data_size() <= max_value_len {
                    return true;
                }

                // Value too big
                return false;
            }

            // Wrong writer
            return false;
        }

        // Subkey out of range
        false
    }

    /// Check if a key is a schema member
    #[must_use]
    pub fn is_member(&self, _key: &PublicKey) -> bool {
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

        Self::new(o_cnt)
    }
}
