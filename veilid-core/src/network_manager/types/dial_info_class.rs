use super::*;

// Keep member order appropriate for sorting < preference
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DialInfoClass {
    Direct = 0, // D = Directly reachable with public IP and no firewall, with statically configured port
    Mapped = 1, // M = Directly reachable with via portmap behind any NAT or firewalled with dynamically negotiated port
    FullConeNAT = 2, // F = Directly reachable device without portmap behind full-cone NAT (or manually mapped firewall port with no configuration change)
    Blocked = 3,     // B = Inbound blocked at firewall but may hole punch with public address
    AddressRestrictedNAT = 4, // A = Device without portmap behind address-only restricted NAT
    PortRestrictedNAT = 5, // P = Device without portmap behind address-and-port restricted NAT
}

impl DialInfoClass {
    // Is a signal required to do an inbound hole-punch?
    pub fn requires_signal(&self) -> bool {
        matches!(
            self,
            Self::Blocked | Self::AddressRestrictedNAT | Self::PortRestrictedNAT
        )
    }

    // Does a relay node need to be allocated for this dial info?
    // For full cone NAT, the relay itself may not be used but the keepalive sent to it
    // is required to keep the NAT mapping valid in the router state table
    pub fn requires_relay(&self) -> bool {
        matches!(
            self,
            Self::FullConeNAT
                | Self::Blocked
                | Self::AddressRestrictedNAT
                | Self::PortRestrictedNAT
        )
    }
}
