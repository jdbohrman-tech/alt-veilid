use super::*;

enum RoutingDomainChange {}

pub struct RoutingDomainEditor {
    routing_table: RoutingTable,
    routing_domain: RoutingDomain,
    changes: Vec<RoutingDomainChange>,
}

impl RoutingDomainEditor {
    pub(super) fn new(routing_table: RoutingTable, routing_domain: RoutingDomain) -> Self {
        Self {
            routing_table,
            routing_domain,
            changes: Vec::new(),
        }
    }

    pub fn commit(self) {}
}
