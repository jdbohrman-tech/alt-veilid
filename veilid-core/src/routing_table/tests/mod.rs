pub mod test_serialize_routing_table;

pub(crate) mod mock_registry {
    use super::super::*;
    use crate::tests::test_veilid_config::setup_veilid_core_with_namespace;
    use crate::{network_manager::NetworkManagerStartupContext, storage_manager::StorageManager};

    pub(crate) async fn init<S: AsRef<str>>(namespace: S) -> VeilidComponentRegistry {
        let (update_callback, config_callback) = setup_veilid_core_with_namespace(namespace);
        let veilid_config =
            VeilidConfig::new_from_callback(config_callback, update_callback).unwrap();
        let registry = VeilidComponentRegistry::new(veilid_config);
        registry.enable_mock();
        registry.register(ProtectedStore::new);
        registry.register(TableStore::new);
        registry.register(Crypto::new);
        registry.register(StorageManager::new);
        registry.register(RoutingTable::new);
        let startup_context = NetworkManagerStartupContext::default();
        registry.register_with_context(NetworkManager::new, startup_context);

        registry.init().await.expect("should init");
        registry.post_init().await.expect("should post init");

        registry
    }

    pub(crate) async fn terminate(registry: VeilidComponentRegistry) {
        registry.pre_terminate().await;
        registry.terminate().await;
    }
}
