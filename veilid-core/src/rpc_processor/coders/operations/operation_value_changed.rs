use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationValueChanged {
    pub key: TypedKey,
    pub subkeys: Vec<ValueSubkeyRange>,
    pub count: u32,
    pub value: ValueData,
}

impl RPCOperationValueChanged {
    pub fn decode(
        reader: &veilid_capnp::operation_value_changed::Reader,
    ) -> Result<RPCOperationValueChanged, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;

        let sk_reader = reader.get_subkeys().map_err(RPCError::protocol)?;
        let mut subkeys = Vec::<ValueSubkeyRange>::with_capacity(
            sk_reader
                .len()
                .try_into()
                .map_err(RPCError::map_protocol("too many subkey ranges"))?,
        );
        for skr in sk_reader.iter() {
            let vskr = (skr.get_start(), skr.get_end());
            if vskr.0 > vskr.1 {
                return Err(RPCError::protocol("invalid subkey range"));
            }
            if let Some(lvskr) = subkeys.last() {
                if lvskr.1 >= vskr.0 {
                    return Err(RPCError::protocol(
                        "subkey range out of order or not merged",
                    ));
                }
            }
            subkeys.push(vskr);
        }
        let count = reader.get_count();
        let v_reader = reader.get_value().map_err(RPCError::protocol)?;
        let value = decode_value_data(&v_reader)?;
        Ok(RPCOperationValueChanged {
            key,
            subkeys,
            count,
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
                .len()
                .try_into()
                .map_err(RPCError::map_internal("invalid subkey range list length"))?,
        );
        for (i, skr) in self.subkeys.iter().enumerate() {
            let mut skr_builder = sk_builder.reborrow().get(i as u32);
            skr_builder.set_start(skr.0);
            skr_builder.set_end(skr.1);
        }

        builder.set_count(self.count);

        let mut v_builder = builder.reborrow().init_value();
        encode_value_data(&self.value, &mut v_builder)?;
        Ok(())
    }
}
