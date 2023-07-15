use super::*;

/// Simple DHT Schema (SMPL) Member
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct DHTSchemaSMPLMember {
    /// Member key
    #[schemars(with = "String")]
    pub m_key: PublicKey,
    /// Member subkey count
    pub m_cnt: u16,
}

/// Simple DHT Schema (SMPL)
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema)]
pub struct DHTSchemaSMPL {
    /// Owner subkey count
    pub o_cnt: u16,
    /// Members
    pub members: Vec<DHTSchemaSMPLMember>,
}

impl DHTSchemaSMPL {
    pub const FCC: [u8; 4] = *b"SMPL";
    pub const FIXED_SIZE: usize = 6;

    /// Build the data representation of the schema
    pub fn compile(&self) -> Vec<u8> {
        let mut out = Vec::<u8>::with_capacity(
            Self::FIXED_SIZE + (self.members.len() * (PUBLIC_KEY_LENGTH + 2)),
        );
        // kind
        out.extend_from_slice(&Self::FCC);
        // o_cnt
        out.extend_from_slice(&self.o_cnt.to_le_bytes());
        // members
        for m in &self.members {
            // m_key
            out.extend_from_slice(&m.m_key.bytes);
            // m_cnt
            out.extend_from_slice(&m.m_cnt.to_le_bytes());
        }
        out
    }

    /// Get the number of subkeys this schema allocates
    pub fn subkey_count(&self) -> usize {
        self.members
            .iter()
            .fold(self.o_cnt as usize, |acc, x| acc + (x.m_cnt as usize))
    }

    /// Get the data size of this schema beyond the size of the structure itself
    pub fn data_size(&self) -> usize {
        self.members.len() * mem::size_of::<DHTSchemaSMPLMember>()
    }

    /// Check a subkey value data against the schema
    pub fn check_subkey_value_data(
        &self,
        owner: &PublicKey,
        subkey: ValueSubkey,
        value_data: &ValueData,
    ) -> bool {
        let mut cur_subkey = subkey as usize;

        // Check is subkey is in owner range
        if cur_subkey < (self.o_cnt as usize) {
            // Check value data has valid writer
            if value_data.writer() == owner {
                return true;
            }
            // Wrong writer
            return false;
        }
        cur_subkey -= self.o_cnt as usize;

        // Check all member ranges
        for m in &self.members {
            // Check if subkey is in member range
            if cur_subkey < (m.m_cnt as usize) {
                // Check value data has valid writer
                if value_data.writer() == &m.m_key {
                    return true;
                }
                // Wrong writer
                return false;
            }
            cur_subkey -= m.m_cnt as usize;
        }

        // Subkey out of range
        false
    }
}

impl TryFrom<&[u8]> for DHTSchemaSMPL {
    type Error = VeilidAPIError;
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        if b.len() != Self::FIXED_SIZE {
            apibail_generic!("invalid size");
        }
        if &b[0..4] != &Self::FCC {
            apibail_generic!("wrong fourcc");
        }
        if (b.len() - Self::FIXED_SIZE) % (PUBLIC_KEY_LENGTH + 2) != 0 {
            apibail_generic!("invalid member length");
        }

        let o_cnt = u16::from_le_bytes(b[4..6].try_into().map_err(VeilidAPIError::internal)?);

        let members_len = (b.len() - Self::FIXED_SIZE) / (PUBLIC_KEY_LENGTH + 2);
        let mut members: Vec<DHTSchemaSMPLMember> = Vec::with_capacity(members_len);
        for n in 0..members_len {
            let mstart = Self::FIXED_SIZE + n * (PUBLIC_KEY_LENGTH + 2);
            let m_key = PublicKey::try_from(&b[mstart..mstart + PUBLIC_KEY_LENGTH])
                .map_err(VeilidAPIError::internal)?;
            let m_cnt = u16::from_le_bytes(
                b[mstart + PUBLIC_KEY_LENGTH..mstart + PUBLIC_KEY_LENGTH + 2]
                    .try_into()
                    .map_err(VeilidAPIError::internal)?,
            );
            members.push(DHTSchemaSMPLMember { m_key, m_cnt });
        }

        Ok(Self { o_cnt, members })
    }
}
