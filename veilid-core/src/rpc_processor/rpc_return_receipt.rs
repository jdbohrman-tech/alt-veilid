use super::*;

impl RPCProcessor {
    // Sends a unidirectional in-band return receipt
    // Can be sent via all methods including relays and routes
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, receipt), ret, err)
    )]
    pub async fn rpc_call_return_receipt<D: AsRef<[u8]>>(
        self,
        dest: Destination,
        receipt: D,
    ) -> Result<NetworkResult<()>, RPCError> {
        let receipt = receipt.as_ref().to_vec();

        let return_receipt = RPCOperationReturnReceipt::new(receipt)?;
        let statement =
            RPCStatement::new(RPCStatementDetail::ReturnReceipt(Box::new(return_receipt)));

        // Send the return_receipt request
        network_result_try!(self.statement(dest, statement).await?);

        Ok(NetworkResult::value(()))
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_return_receipt(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get the statement
        let (_, _, _, kind) = msg.operation.destructure();
        let receipt = match kind {
            RPCOperationKind::Statement(s) => match s.destructure() {
                RPCStatementDetail::ReturnReceipt(s) => s.destructure(),
                _ => panic!("not a return receipt"),
            },
            _ => panic!("not a statement"),
        };

        // Handle it
        let network_manager = self.network_manager();

        let res = match msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => {
                network_manager
                    .handle_in_band_receipt(receipt, detail.peer_noderef)
                    .await
            }
            RPCMessageHeaderDetail::SafetyRouted(_) => {
                network_manager.handle_safety_receipt(receipt).await
            }
            RPCMessageHeaderDetail::PrivateRouted(detail) => {
                network_manager
                    .handle_private_receipt(receipt, detail.private_route)
                    .await
            }
        };

        Ok(res)
    }
}
