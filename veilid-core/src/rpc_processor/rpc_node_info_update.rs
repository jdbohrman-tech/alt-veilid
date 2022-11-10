use super::*;

impl RPCProcessor {
    // Sends a our node info to another node
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_node_info_update(
        self,
        target: NodeRef,
        routing_domain: RoutingDomain,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get the signed node info for the desired routing domain to send update with
        let signed_node_info = self
            .routing_table()
            .get_own_peer_info(routing_domain)
            .signed_node_info;
        let node_info_update = RPCOperationNodeInfoUpdate { signed_node_info };
        let statement = RPCStatement::new(RPCStatementDetail::NodeInfoUpdate(node_info_update));

        // Send the node_info_update request to the specific routing domain requested
        network_result_try!(
            self.statement(
                Destination::direct(
                    target.filtered_clone(NodeRefFilter::new().with_routing_domain(routing_domain))
                ),
                statement,
            )
            .await?
        );

        Ok(NetworkResult::value(()))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    pub(crate) async fn process_node_info_update(&self, msg: RPCMessage) -> Result<(), RPCError> {
        let detail = match msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => detail,
            RPCMessageHeaderDetail::SafetyRouted(_) | RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Err(RPCError::protocol("node_info_update must be direct"));
            }
        };
        let sender_node_id = detail.envelope.get_sender_id();
        let routing_domain = detail.routing_domain;

        // Get the statement
        let node_info_update = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::NodeInfoUpdate(s) => s,
                _ => panic!("not a node info update"),
            },
            _ => panic!("not a statement"),
        };

        // Update our routing table with signed node info
        if !self.filter_node_info(routing_domain, &node_info_update.signed_node_info) {
            log_rpc!(debug "node info doesn't belong in {:?} routing domain: {}", routing_domain, sender_node_id);
            return Ok(());
        }

        self.routing_table().register_node_with_signed_node_info(
            routing_domain,
            sender_node_id,
            node_info_update.signed_node_info,
            false,
        );

        Ok(())
    }
}
