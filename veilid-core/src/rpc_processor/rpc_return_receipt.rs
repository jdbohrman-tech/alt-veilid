use super::*;

impl RPCProcessor {
    // Sends a unidirectional in-band return receipt
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", skip(self, receipt), ret, err)]
    pub async fn rpc_call_return_receipt<D: AsRef<[u8]>>(
        self,
        dest: Destination,
        receipt: D,
    ) -> Result<NetworkResult<()>, RPCError> {
        let receipt = receipt.as_ref().to_vec();

        let return_receipt = RPCOperationReturnReceipt { receipt };
        let statement = RPCStatement::new(RPCStatementDetail::ReturnReceipt(return_receipt));

        // Send the return_receipt request
        network_result_try!(self.statement(dest, statement).await?);

        Ok(NetworkResult::value(()))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    pub(crate) async fn process_return_receipt(&self, msg: RPCMessage) -> Result<(), RPCError> {
        // Get the statement
        let RPCOperationReturnReceipt { receipt } = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::ReturnReceipt(s) => s,
                _ => panic!("not a return receipt"),
            },
            _ => panic!("not a statement"),
        };

        // Handle it
        let network_manager = self.network_manager();

        match msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => {
                network_result_value_or_log!(debug
                    network_manager
                        .handle_in_band_receipt(receipt, detail.peer_noderef)
                        .await => {}
                );
            }
            RPCMessageHeaderDetail::PrivateRouted(detail) => {
                network_result_value_or_log!(debug
                    network_manager
                        .handle_private_receipt(receipt, detail.private_route)
                        .await => {}
                );
            }
        }

        Ok(())
    }
}
