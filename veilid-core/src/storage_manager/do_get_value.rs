use super::*;

pub struct DoGetValueResult {
    pub value: Option<SignedValueData>,
    pub descriptor: Option<SignedValueDescriptor>,
}

impl StorageManager {

    pub async fn do_get_value(
        &self,
        mut inner: AsyncMutexGuardArc<StorageManagerInner>,
        key: TypedKey,
        subkey: ValueSubkey,
        min_seq: ValueSeqNum,
        last_descriptor: Option<SignedValueDescriptor>,
        safety_selection: SafetySelection,
    ) -> Result<Option<DoGetValueResult>, VeilidAPIError> {
        let Some(rpc_processor) = inner.rpc_processor.clone() else {
            apibail_not_initialized!();
        };

        let routing_table = rpc_processor.routing_table();

        // Get the DHT parameters for 'GetValue'
        let (count, fanout, timeout) = {
            let c = self.unlocked_inner.config.get();
            (
                c.network.dht.get_value_count as usize,
                c.network.dht.get_value_fanout as usize,
                TimestampDuration::from(ms_to_us(c.network.dht.get_value_timeout_ms)),
            )
        };

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            async move {
                match rpc_processor
                    .clone()
                    .rpc_call_get_value(
                        Destination::direct(next_node).with_safety(safety_selection),
                        key, subkey, last_descriptor
                    )
                    .await
                {
                    Ok(v) => {
                        let v = network_result_value_or_log!(v => {
                            // Any other failures, just try the next node
                            return Ok(None);
                        });
                        
                        // Keep the value if we got one and it is newer and it passes schema validation
                        if let Some(value) = v.answer.value {
                            // See if this is even a candidate
                            if value.value_data().    xxx apply min_seq and also to OperationGetValueQ
                            // Validate with scheam
                        }

                        // Return peers if we have some
                        Ok(Some(v.answer.peers))
                    }
                    Err(e) => Err(e),
                }
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |closest_nodes: &[NodeRef]| {
            // If the node we want to locate is one of the closest nodes, return it immediately
            if let Some(out) = closest_nodes
                .iter()
                .find(|x| x.node_ids().contains(&node_id))
            {
                return Some(out.clone());
            }
            None
        };

        // Call the fanout
        let fanout_call = FanoutCall::new(
            routing_table.clone(),
            node_id,
            count,
            fanout,
            timeout_us,
            call_routine,
            check_done,
        );

        fanout_call.run().await

        // Search in preferred cryptosystem order
        let nr = this
            .search_dht_single_key(node_id, count, fanout, timeout, safety_selection)
            .await?;

        if let Some(nr) = &nr {
            if nr.node_ids().contains(&node_id) {
                // found a close node, but not exact within our configured resolve_node timeout
                return Ok(None);
            }
        }

        Ok(nr)
    }
}
