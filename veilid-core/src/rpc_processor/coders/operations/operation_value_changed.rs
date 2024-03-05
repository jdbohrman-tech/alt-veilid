use super::*;
use crate::storage_manager::SignedValueData;

const MAX_VALUE_CHANGED_SUBKEYS_LEN: usize = 512;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationValueChanged {
    key: TypedKey,
    subkeys: ValueSubkeyRangeSet,
    count: u32,
    watch_id: u64,
    value: SignedValueData,
}

impl RPCOperationValueChanged {
    #[allow(dead_code)]
    pub fn new(
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        count: u32,
        watch_id: u64,
        value: SignedValueData,
    ) -> Result<Self, RPCError> {
        // Needed because RangeSetBlaze uses different types here all the time
        #[allow(clippy::unnecessary_cast)]
        let subkeys_len = subkeys.ranges_len() as usize;

        if subkeys_len > MAX_VALUE_CHANGED_SUBKEYS_LEN {
            return Err(RPCError::protocol("ValueChanged subkeys length too long"));
        }

        if watch_id == 0 {
            return Err(RPCError::protocol("ValueChanged needs a nonzero watch id"));
        }

        Ok(Self {
            key,
            subkeys,
            count,
            watch_id,
            value,
        })
    }

    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        if self.watch_id == 0 {
            return Err(RPCError::protocol("ValueChanged does not have a valid id"));
        }
        // further validation must be done by storage manager as this is more complicated
        Ok(())
    }

    #[allow(dead_code)]
    pub fn key(&self) -> &TypedKey {
        &self.key
    }

    #[allow(dead_code)]
    pub fn subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.subkeys
    }

    #[allow(dead_code)]
    pub fn count(&self) -> u32 {
        self.count
    }

    #[allow(dead_code)]
    pub fn watch_id(&self) -> u64 {
        self.watch_id
    }

    #[allow(dead_code)]
    pub fn value(&self) -> &SignedValueData {
        &self.value
    }

    #[allow(dead_code)]
    pub fn destructure(self) -> (TypedKey, ValueSubkeyRangeSet, u32, u64, SignedValueData) {
        (
            self.key,
            self.subkeys,
            self.count,
            self.watch_id,
            self.value,
        )
    }

    pub fn decode(
        reader: &veilid_capnp::operation_value_changed::Reader,
    ) -> Result<Self, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;

        let sk_reader = reader.get_subkeys().map_err(RPCError::protocol)?;
        if sk_reader.len() as usize > MAX_VALUE_CHANGED_SUBKEYS_LEN {
            return Err(RPCError::protocol("ValueChanged subkeys length too long"));
        }

        let mut subkeys = ValueSubkeyRangeSet::new();
        for skr in sk_reader.iter() {
            let vskr = (skr.get_start(), skr.get_end());
            if vskr.0 > vskr.1 {
                return Err(RPCError::protocol("invalid subkey range"));
            }
            if let Some(lvskr) = subkeys.last() {
                if lvskr >= vskr.0 {
                    return Err(RPCError::protocol(
                        "subkey range out of order or not merged",
                    ));
                }
            }
            subkeys.ranges_insert(vskr.0..=vskr.1);
        }
        let count = reader.get_count();
        let v_reader = reader.get_value().map_err(RPCError::protocol)?;
        let watch_id = reader.get_watch_id();
        let value = decode_signed_value_data(&v_reader)?;

        Ok(Self {
            key,
            subkeys,
            count,
            watch_id,
            value,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_value_changed::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_typed_key(&self.key, &mut k_builder);

        let mut sk_builder = builder.reborrow().init_subkeys(
            self.subkeys
                .ranges_len()
                .try_into()
                .map_err(RPCError::map_internal("invalid subkey range list length"))?,
        );
        for (i, skr) in self.subkeys.ranges().enumerate() {
            let mut skr_builder = sk_builder.reborrow().get(i as u32);
            skr_builder.set_start(*skr.start());
            skr_builder.set_end(*skr.end());
        }

        builder.set_count(self.count);
        builder.set_watch_id(self.watch_id);

        let mut v_builder = builder.reborrow().init_value();
        encode_signed_value_data(&self.value, &mut v_builder)?;
        Ok(())
    }
}
