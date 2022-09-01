use super::*;

impl RPCProcessor {
    // Sends a our node info to another node
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_node_info_update(
        self,
        target: NodeRef,
        routing_domain: RoutingDomain,
    ) -> Result<NetworkResult<()>, RPCError> {
        let signed_node_info = self
            .routing_table()
            .get_own_signed_node_info(routing_domain);
        let node_info_update = RPCOperationNodeInfoUpdate { signed_node_info };
        let statement = RPCStatement::new(RPCStatementDetail::NodeInfoUpdate(node_info_update));

        // Send the node_info_update request
        network_result_try!(
            self.statement(
                Destination::direct(target).with_routing_domain(routing_domain),
                statement,
            )
            .await?
        );

        Ok(NetworkResult::value(()))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    pub(crate) async fn process_node_info_update(&self, msg: RPCMessage) -> Result<(), RPCError> {
        let sender_node_id = msg.header.envelope.get_sender_id();

        // Get the statement
        let node_info_update = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::NodeInfoUpdate(s) => s,
                _ => panic!("not a node info update"),
            },
            _ => panic!("not a statement"),
        };

        // Update our routing table with signed node info
        if !self.filter_peer_scope(&node_info_update.signed_node_info.node_info) {
            log_rpc!(debug
                "node_info_update has invalid peer scope from {}", sender_node_id
            );
            return Ok(());
        }

        self.routing_table().register_node_with_signed_node_info(
            sender_node_id,
            node_info_update.signed_node_info,
            false,
        );

        Ok(())
    }
}
