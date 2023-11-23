use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err))]
    // Sends a high level app message
    // Can be sent via all methods including relays and routes
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, message), fields(message.len = message.len()), err)
    )]
    pub async fn rpc_call_value_changed(
        self,
        dest: Destination,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        count: u32,
        value: SignedValueData,
    ) -> RPCNetworkResult<()> {
        let value_changed = RPCOperationValueChanged::new(key, subkeys, count, value);
        let statement =
            RPCStatement::new(RPCStatementDetail::ValueChanged(Box::new(value_changed)));

        // Send the value changed request
        self.statement(dest, statement).await
    }

    pub(crate) async fn process_value_changed(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        // Get the statement
        let (_, _, _, kind) = msg.operation.destructure();
        let (key, subkeys, count, value) = match kind {
            RPCOperationKind::Statement(s) => match s.destructure() {
                RPCStatementDetail::ValueChanged(s) => s.destructure(),
                _ => panic!("not a value changed statement"),
            },
            _ => panic!("not a statement"),
        };

        #[cfg(feature = "debug-dht")]
        {
            let debug_string_value = format!(
                " len={} seq={} writer={}",
                value.value_data().data().len(),
                value.value_data().seq(),
                value.value_data().writer(),
            );

            let debug_string_stmt = format!(
                "IN <== ValueChanged({} #{:?}+{}{}) <= {}",
                key,
                subkeys,
                count,
                debug_string_value,
                msg.header.direct_sender_node_id()
            );

            log_rpc!(debug "{}", debug_string_stmt);
        }

        // Save the subkey, creating a new record if necessary
        let storage_manager = self.storage_manager();
        storage_manager
            .inbound_value_changed(key, subkeys, count, value)
            .await
            .map_err(RPCError::internal)?;

        Ok(NetworkResult::value(()))
    }
}
