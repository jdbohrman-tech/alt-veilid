use super::*;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum RespondTo {
    Sender,
    PrivateRoute(PrivateRoute),
}

impl RespondTo {
    pub fn validate(&mut self, crypto: Crypto) -> Result<(), RPCError> {
        match self {
            RespondTo::Sender => Ok(()),
            RespondTo::PrivateRoute(pr) => pr.validate(crypto).map_err(RPCError::protocol),
        }
    }

    pub fn encode(
        &self,
        builder: &mut veilid_capnp::question::respond_to::Builder,
    ) -> Result<(), RPCError> {
        match self {
            Self::Sender => {
                builder.reborrow().set_sender(());
            }
            Self::PrivateRoute(pr) => {
                let mut pr_builder = builder.reborrow().init_private_route();
                encode_private_route(pr, &mut pr_builder)?;
            }
        };
        Ok(())
    }

    pub fn decode(reader: &veilid_capnp::question::respond_to::Reader) -> Result<Self, RPCError> {
        let respond_to = match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::question::respond_to::Sender(()) => RespondTo::Sender,
            veilid_capnp::question::respond_to::PrivateRoute(pr_reader) => {
                let pr_reader = pr_reader.map_err(RPCError::protocol)?;
                let pr = decode_private_route(&pr_reader)?;
                RespondTo::PrivateRoute(pr)
            }
        };
        Ok(respond_to)
    }
}
