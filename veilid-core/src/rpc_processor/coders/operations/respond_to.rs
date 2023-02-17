use super::*;

#[derive(Debug, Clone)]
pub enum RespondTo {
    Sender,
    PrivateRoute(PrivateRoute),
}

impl RespondTo {
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

    pub fn decode(
        reader: &veilid_capnp::question::respond_to::Reader,
        crypto: Crypto,
    ) -> Result<Self, RPCError> {
        let respond_to = match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::question::respond_to::Sender(()) => RespondTo::Sender,
            veilid_capnp::question::respond_to::PrivateRoute(pr_reader) => {
                let pr_reader = pr_reader.map_err(RPCError::protocol)?;
                let pr = decode_private_route(&pr_reader, crypto)?;
                RespondTo::PrivateRoute(pr)
            }
        };
        Ok(respond_to)
    }
}
