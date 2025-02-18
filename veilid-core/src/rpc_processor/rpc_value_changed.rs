use super::*;

impl RPCProcessor {
    // Sends a dht value change notification
    // Can be sent via all methods including relays and routes but never over a safety route
    #[instrument(level = "trace", target = "rpc", skip(self, value), err)]
    pub async fn rpc_call_value_changed(
        &self,
        dest: Destination,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        count: u32,
        watch_id: u64,
        value: Option<SignedValueData>,
    ) -> RPCNetworkResult<()> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .map_err(RPCError::map_try_again("not started up"))?;

        // Ensure destination is never using a safety route
        if matches!(dest.get_safety_selection(), SafetySelection::Safe(_)) {
            return Err(RPCError::internal(
                "Never send value changes over safety routes",
            ));
        }
        let value_changed = RPCOperationValueChanged::new(key, subkeys, count, watch_id, value)?;
        let statement =
            RPCStatement::new(RPCStatementDetail::ValueChanged(Box::new(value_changed)));

        // Send the value changed request
        self.statement(dest, statement).await
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    #[instrument(level = "trace", target = "rpc", skip_all)]
    pub(super) async fn process_value_changed(&self, msg: Message) -> RPCNetworkResult<()> {
        // Get the statement
        let (_, _, kind) = msg.operation.destructure();
        let (key, subkeys, count, watch_id, value) = match kind {
            RPCOperationKind::Statement(s) => match s.destructure() {
                RPCStatementDetail::ValueChanged(s) => s.destructure(),
                _ => panic!("not a value changed statement"),
            },
            _ => panic!("not a statement"),
        };

        // Get the inbound node if if this came in directly
        // If this was received over just a safety route, ignore it
        // It this was received over a private route, the inbound node id could be either the actual
        // node id, or a safety route (can't tell if a stub was used).
        // Try it as the node if, and the storage manager will reject the
        // value change if it doesn't match the active watch's node id
        let inbound_node_id = match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(d) => d.envelope.get_sender_typed_id(),
            RPCMessageHeaderDetail::SafetyRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "not processing value change over safety route",
                ));
            }
            RPCMessageHeaderDetail::PrivateRouted(p) => {
                TypedKey::new(p.direct.envelope.get_crypto_kind(), p.remote_safety_route)
            }
        };

        if debug_target_enabled!("dht") {
            let debug_string_value = if let Some(value) = &value {
                format!(
                    " len={} seq={} writer={}",
                    value.value_data().data().len(),
                    value.value_data().seq(),
                    value.value_data().writer(),
                )
            } else {
                "(no value)".to_owned()
            };

            let debug_string_stmt = format!(
                "IN <== ValueChanged(id={} {} #{:?}+{}{}) from {} <= {}",
                watch_id,
                key,
                subkeys,
                count,
                debug_string_value,
                inbound_node_id,
                msg.header.direct_sender_node_id(),
            );

            veilid_log!(self debug "{}", debug_string_stmt);
        }

        // Save the subkey, creating a new record if necessary
        let storage_manager = self.storage_manager();
        storage_manager
            .inbound_value_changed(
                key,
                subkeys,
                count,
                value.map(Arc::new),
                inbound_node_id,
                watch_id,
            )
            .await
            .map_err(RPCError::internal)
    }
}
