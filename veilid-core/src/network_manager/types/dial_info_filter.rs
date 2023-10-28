use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DialInfoFilter {
    pub protocol_type_set: ProtocolTypeSet,
    pub address_type_set: AddressTypeSet,
}

impl Default for DialInfoFilter {
    fn default() -> Self {
        Self {
            protocol_type_set: ProtocolTypeSet::all(),
            address_type_set: AddressTypeSet::all(),
        }
    }
}

impl DialInfoFilter {
    pub fn all() -> Self {
        Self {
            protocol_type_set: ProtocolTypeSet::all(),
            address_type_set: AddressTypeSet::all(),
        }
    }
    pub fn with_protocol_type(mut self, protocol_type: ProtocolType) -> Self {
        self.protocol_type_set = ProtocolTypeSet::only(protocol_type);
        self
    }
    pub fn with_protocol_type_set(mut self, protocol_set: ProtocolTypeSet) -> Self {
        self.protocol_type_set = protocol_set;
        self
    }
    pub fn with_address_type(mut self, address_type: AddressType) -> Self {
        self.address_type_set = AddressTypeSet::only(address_type);
        self
    }
    pub fn with_address_type_set(mut self, address_set: AddressTypeSet) -> Self {
        self.address_type_set = address_set;
        self
    }
    pub fn filtered(mut self, other_dif: &DialInfoFilter) -> Self {
        self.protocol_type_set &= other_dif.protocol_type_set;
        self.address_type_set &= other_dif.address_type_set;
        self
    }
    pub fn is_dead(&self) -> bool {
        self.protocol_type_set.is_empty() || self.address_type_set.is_empty()
    }
    pub fn with_sequencing(self, sequencing: Sequencing) -> (bool, DialInfoFilter) {
        // Get first filtered dialinfo
        match sequencing {
            Sequencing::NoPreference => (false, self),
            Sequencing::PreferOrdered => (true, self),
            Sequencing::EnsureOrdered => (
                true,
                self.filtered(
                    &DialInfoFilter::all().with_protocol_type_set(ProtocolType::all_ordered_set()),
                ),
            ),
        }
        // return ordered sort and filter with ensure applied
    }
}

impl fmt::Debug for DialInfoFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut out = String::new();
        if self.protocol_type_set != ProtocolTypeSet::all() {
            out += &format!("+{:?}", self.protocol_type_set);
        } else {
            out += "*";
        }
        if self.address_type_set != AddressTypeSet::all() {
            out += &format!("+{:?}", self.address_type_set);
        } else {
            out += "*";
        }
        write!(f, "[{}]", out)
    }
}

impl From<ProtocolType> for DialInfoFilter {
    fn from(other: ProtocolType) -> Self {
        Self {
            protocol_type_set: ProtocolTypeSet::from(other),
            address_type_set: AddressTypeSet::all(),
        }
    }
}

impl From<AddressType> for DialInfoFilter {
    fn from(other: AddressType) -> Self {
        Self {
            protocol_type_set: ProtocolTypeSet::all(),
            address_type_set: AddressTypeSet::from(other),
        }
    }
}

impl From<ConnectionDescriptor> for DialInfoFilter {
    fn from(other: ConnectionDescriptor) -> Self {
        Self {
            protocol_type_set: ProtocolTypeSet::from(other.protocol_type()),
            address_type_set: AddressTypeSet::from(other.address_type()),
        }
    }
}

pub trait MatchesDialInfoFilter {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool;
}
