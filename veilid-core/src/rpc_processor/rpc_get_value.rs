use super::*;
use crate::storage_manager::{SignedValueData, SignedValueDescriptor};

#[derive(Clone, Debug)]
pub struct GetValueAnswer {
    pub value: Option<SignedValueData>,
    pub peers: Vec<PeerInfo>,
    pub descriptor: Option<SignedValueDescriptor>,
}

impl RPCProcessor {
    // Sends a get value request and wait for response
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_get_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkey: ValueSubkey,
        last_descriptor: Option<SignedValueDescriptor>,
    ) -> Result<NetworkResult<Answer<GetValueAnswer>>, RPCError> {
        let get_value_q = RPCOperationGetValueQ::new(key, subkey, last_descriptor.is_none());
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::GetValueQ(get_value_q),
        );
        let Some(vcrypto) = self.crypto.get(key.kind) else {
            return Err(RPCError::internal("unsupported cryptosystem"));
        };

        // Send the app call question
        let question_context = QuestionContext::GetValue(ValidateGetValueContext {
            last_descriptor,
            subkey,
            vcrypto,
        });

        let waitable_reply = network_result_try!(
            self.question(dest, question, Some(question_context))
                .await?
        );

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let (_, _, _, kind) = msg.operation.destructure();
        let get_value_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::GetValueA(a) => a,
                _ => return Err(RPCError::invalid_format("not a getvalue answer")),
            },
            _ => return Err(RPCError::invalid_format("not an answer")),
        };

        let (value, peers, descriptor) = get_value_a.destructure();

        Ok(NetworkResult::value(Answer::new(
            latency,
            GetValueAnswer {
                value,
                peers,
                descriptor,
            },
        )))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(crate) async fn process_get_value_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        Err(RPCError::unimplemented("process_get_value_q"))
    }
}
