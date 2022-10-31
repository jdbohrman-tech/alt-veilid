use super::*;

impl RPCProcessor {
    // Sends a unidirectional signal to a node
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_signal(
        self,
        dest: Destination,
        signal_info: SignalInfo,
    ) -> Result<NetworkResult<()>, RPCError> {
        let signal = RPCOperationSignal { signal_info };
        let statement = RPCStatement::new(RPCStatementDetail::Signal(signal));

        // Send the signal request
        network_result_try!(self.statement(dest, statement).await?);

        Ok(NetworkResult::value(()))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    pub(crate) async fn process_signal(&self, msg: RPCMessage) -> Result<(), RPCError> {
        // Get the statement
        let signal = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::Signal(s) => s,
                _ => panic!("not a signal"),
            },
            _ => panic!("not a statement"),
        };

        // Handle it
        let network_manager = self.network_manager();
        network_result_value_or_log!(debug network_manager
            .handle_signal(signal.signal_info)
            .await
            .map_err(RPCError::network)? => {
                return Ok(());
            }
        );

        Ok(())
    }
}
