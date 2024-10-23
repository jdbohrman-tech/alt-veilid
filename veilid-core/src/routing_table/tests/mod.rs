use super::*;

pub mod test_serialize_routing_table;

pub(crate) fn mock_routing_table() -> routing_table::RoutingTable {
    let event_bus = EventBus::new();
    let veilid_config = VeilidConfig::new();
    #[cfg(feature = "unstable-blockstore")]
    let block_store = BlockStore::new(event_bus.clone(), veilid_config.clone());
    let protected_store = ProtectedStore::new(event_bus.clone(), veilid_config.clone());
    let table_store = TableStore::new(
        event_bus.clone(),
        veilid_config.clone(),
        protected_store.clone(),
    );
    let crypto = Crypto::new(
        event_bus.clone(),
        veilid_config.clone(),
        table_store.clone(),
    );
    let storage_manager = storage_manager::StorageManager::new(
        event_bus.clone(),
        veilid_config.clone(),
        crypto.clone(),
        table_store.clone(),
        #[cfg(feature = "unstable-blockstore")]
        block_store.clone(),
    );
    let network_manager = network_manager::NetworkManager::new(
        event_bus.clone(),
        veilid_config.clone(),
        storage_manager,
        table_store.clone(),
        #[cfg(feature = "unstable-blockstore")]
        block_store.clone(),
        crypto.clone(),
    );
    RoutingTable::new(network_manager)
}
