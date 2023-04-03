mod record_store;
mod value_record;
mod record_store_limits;
use record_store::*;
use record_store_limits::*;
use value_record::*;

use super::*;
use crate::rpc_processor::*;

struct StorageManagerInner {
    record_store: RecordStore,
}

struct StorageManagerUnlockedInner {
    config: VeilidConfig,
    crypto: Crypto,
    protected_store: ProtectedStore,
    table_store: TableStore,
    block_store: BlockStore,
    rpc_processor: RPCProcessor,
}

#[derive(Clone)]
pub struct StorageManager {
    unlocked_inner: Arc<StorageManagerUnlockedInner>,
    inner: Arc<Mutex<StorageManagerInner>>,
}

impl StorageManager {
    fn new_unlocked_inner(
        config: VeilidConfig,
        crypto: Crypto,
        protected_store: ProtectedStore,
        table_store: TableStore,
        block_store: BlockStore,
        rpc_processor: RPCProcessor,
    ) -> StorageManagerUnlockedInner {
        StorageManagerUnlockedInner {
            config,
            crypto,
            protected_store,
            table_store,
            block_store,
            rpc_processor,
        }
    }
    fn new_inner() -> StorageManagerInner {
        StorageManagerInner {
            record_store: RecordStore::new(table_store),
        }
    }

    pub fn new(
        config: VeilidConfig,
        crypto: Crypto,
        protected_store: ProtectedStore,
        table_store: TableStore,
        block_store: BlockStore,
        rpc_processor: RPCProcessor,
    ) -> StorageManager {
        StorageManager {
            unlocked_inner: Arc::new(Self::new_unlocked_inner(
                config,
                crypto,
                protected_store,
                table_store,
                block_store,
                rpc_processor,
            )),
            inner: Arc::new(Mutex::new(Self::new_inner())),
        }
    }

    #[instrument(level = "debug", skip_all, err)]
    pub async fn init(&self) -> EyreResult<()> {
        debug!("startup storage manager");
        let mut inner = self.inner.lock();
        // xxx
        Ok(())
    }

    pub fn terminate(&self) {
        debug!("starting storage manager shutdown");

        // Release the storage manager
        *self.inner.lock() = Self::new_inner();

        debug!("finished storage manager shutdown");
    }

    async fn add_value_record(&self, key: TypedKey, record: ValueRecord) -> EyreResult<()> {
        // add value record to record store
        let mut inner = self.inner.lock();
        inner.record_store.
    }

    /// Creates a new DHT value with a specified crypto kind and schema
    /// Returns the newly allocated DHT Key if successful.
    pub async fn create_value(
        &self,
        kind: CryptoKind,
        schema: &DHTSchema,
        safety_selection: SafetySelection,
    ) -> Result<TypedKey, VeilidAPIError> {
        // Get cryptosystem
        let Some(vcrypto) = self.unlocked_inner.crypto.get(kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        // New values require a new owner key
        let keypair = vcrypto.generate_keypair();
        let key = TypedKey::new(kind, keypair.key);
        let secret = keypair.secret;

        // Add value record
        let record = ValueRecord::new(Some(secret), schema, safety_selection);
        self.add_value_record(key, record)
            .await
            .map_err(VeilidAPIError::internal)?;

        Ok(key)
    }

    /// Opens a DHT value at a specific key. Associates an owner secret if one is provided.
    /// Returns the DHT key descriptor for the opened key if successful
    /// Value may only be opened or created once. To re-open with a different routing context, first close the value.
    pub async fn open_value(
        key: TypedKey,
        secret: Option<SecretKey>,
        safety_selection: SafetySelection,
    ) -> Result<DHTDescriptor, VeilidAPIError> {
        unimplemented!();
    }

    /// Closes a DHT value at a specific key that was opened with create_value or open_value.
    /// Closing a value allows you to re-open it with a different routing context
    pub async fn close_value(key: TypedKey) -> Result<(), VeilidAPIError> {
        unimplemented!();
    }

    /// Gets the latest value of a subkey from the network
    /// Returns the possibly-updated value data of the subkey
    pub async fn get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    ) -> Result<ValueData, VeilidAPIError> {
        unimplemented!();
    }

    /// Pushes a changed subkey value to the network
    /// Returns None if the value was successfully put
    /// Returns Some(newer_value) if the value put was older than the one available on the network
    pub async fn set_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        value_data: ValueData,
    ) -> Result<Option<ValueData>, VeilidAPIError> {
        unimplemented!();
    }

    /// Watches changes to an opened or created value
    /// Changes to subkeys within the subkey range are returned via a ValueChanged callback
    /// If the subkey range is empty, all subkey changes are considered
    /// Expiration can be infinite to keep the watch for the maximum amount of time
    /// Return value upon success is the amount of time allowed for the watch
    pub async fn watch_value(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
        expiration: Timestamp,
        count: u32,
    ) -> Result<Timestamp, VeilidAPIError> {
        unimplemented!();
    }

    /// Cancels a watch early
    /// This is a convenience function that cancels watching all subkeys in a range
    pub async fn cancel_watch_value(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
    ) -> Result<bool, VeilidAPIError> {
        unimplemented!();
    }
}
