use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationValueChanged {
    pub key: ValueKey,
    pub value: ValueData,
}

impl RPCOperationValueChanged {
    pub fn decode(
        reader: &veilid_capnp::operation_value_changed::Reader,
    ) -> Result<RPCOperationValueChanged, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_value_key(&k_reader)?;
        let v_reader = reader.get_value().map_err(RPCError::protocol)?;
        let value = decode_value_data(&v_reader)?;
        Ok(RPCOperationValueChanged { key, value })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_value_changed::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_value_key(&self.key, &mut k_builder)?;
        let mut v_builder = builder.reborrow().init_value();
        encode_value_data(&self.value, &mut v_builder)?;
        Ok(())
    }
}
