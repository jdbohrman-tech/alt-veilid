use super::*;

pub mod test_serialize_routing_table;

pub(crate) fn mock_routing_table() -> routing_table::RoutingTable {
    let veilid_config = VeilidConfig::new();
    #[cfg(feature = "unstable-blockstore")]
    let block_store = BlockStore::new(veilid_config.clone());
    let protected_store = ProtectedStore::new(veilid_config.clone());
    let table_store = TableStore::new(veilid_config.clone(), protected_store.clone());
    let crypto = Crypto::new(veilid_config.clone(), table_store.clone());
    let storage_manager = storage_manager::StorageManager::new(
        veilid_config.clone(),
        crypto.clone(),
        table_store.clone(),
        #[cfg(feature = "unstable-blockstore")]
        block_store.clone(),
    );
    let network_manager = network_manager::NetworkManager::new(
        veilid_config.clone(),
        storage_manager,
        table_store.clone(),
        #[cfg(feature = "unstable-blockstore")]
        block_store.clone(),
        crypto.clone(),
    );
    RoutingTable::new(network_manager)
}
