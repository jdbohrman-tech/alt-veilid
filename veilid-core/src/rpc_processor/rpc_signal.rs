use super::*;

impl RPCProcessor {
    // Sends a unidirectional signal to a node
    // Can be sent via relays but not routes. For routed 'signal' like capabilities, use AppMessage.
    #[instrument(level = "trace", target = "rpc", skip(self), ret, err)]
    pub async fn rpc_call_signal(
        self,
        dest: Destination,
        signal_info: SignalInfo,
    ) -> RPCNetworkResult<()> {
        let _guard = self
            .unlocked_inner
            .startup_lock
            .enter()
            .map_err(RPCError::map_try_again("not started up"))?;

        // Ensure destination never has a private route
        if matches!(
            dest,
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _
            }
        ) {
            return Err(RPCError::internal(
                "Never send signal requests over private routes",
            ));
        }

        let signal = RPCOperationSignal::new(signal_info);
        let statement = RPCStatement::new(RPCStatementDetail::Signal(Box::new(signal)));

        // Send the signal request
        self.statement(dest, statement).await
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    #[instrument(level = "trace", target = "rpc", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(crate) async fn process_signal(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        // Ignore if disabled
        let routing_table = self.routing_table();

        let has_capability_signal = routing_table
            .get_published_peer_info(msg.header.routing_domain())
            .map(|ppi| {
                ppi.signed_node_info()
                    .node_info()
                    .has_capability(CAP_SIGNAL)
            })
            .unwrap_or(false);
        if !has_capability_signal {
            return Ok(NetworkResult::service_unavailable(
                "signal is not available",
            ));
        }

        // Can't allow anything other than direct packets here, as handling reverse connections
        // or anything like via signals over private routes would deanonymize the route
        let flow = match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(d) => d.flow,
            RPCMessageHeaderDetail::SafetyRouted(_) | RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message("signal must be direct"));
            }
        };

        // Get the statement
        let (_, _, kind) = msg.operation.destructure();
        let signal = match kind {
            RPCOperationKind::Statement(s) => match s.destructure() {
                RPCStatementDetail::Signal(s) => s,
                _ => panic!("not a signal"),
            },
            _ => panic!("not a statement"),
        };

        // Handle it
        let network_manager = self.network_manager();
        let signal_info = signal.destructure();
        network_manager
            .handle_signal(flow, signal_info)
            .await
            .map_err(RPCError::network)
    }
}
