use super::*;
use core::convert::TryInto;

pub(crate) fn decode_dial_info(
    reader: &veilid_capnp::dial_info::Reader,
) -> Result<DialInfo, RPCError> {
    match reader
        .reborrow()
        .which()
        .map_err(RPCError::map_protocol("Missing dial info type"))?
    {
        veilid_capnp::dial_info::Which::Udp(udp) => {
            let socket_address_reader = udp
                .map_err(RPCError::protocol)?
                .get_socket_address()
                .map_err(RPCError::map_protocol("missing UDP socketAddress"))?;
            let socket_address = decode_socket_address(&socket_address_reader)?;
            Ok(DialInfo::udp(socket_address))
        }
        veilid_capnp::dial_info::Which::Tcp(tcp) => {
            let socket_address_reader = tcp
                .map_err(RPCError::protocol)?
                .get_socket_address()
                .map_err(RPCError::map_protocol("missing TCP socketAddress"))?;
            let socket_address = decode_socket_address(&socket_address_reader)?;
            Ok(DialInfo::tcp(socket_address))
        }
        veilid_capnp::dial_info::Which::Ws(ws) => {
            let ws = ws.map_err(RPCError::protocol)?;
            let socket_address_reader = ws
                .get_socket_address()
                .map_err(RPCError::map_protocol("missing WS socketAddress"))?;
            let socket_address = decode_socket_address(&socket_address_reader)?;
            let request = ws
                .get_request()
                .map_err(RPCError::map_protocol("missing WS request"))?;
            DialInfo::try_ws(
                socket_address,
                request
                    .to_string()
                    .map_err(RPCError::map_protocol("invalid WS request string"))?,
            )
            .map_err(RPCError::map_protocol("invalid WS dial info"))
        }
        veilid_capnp::dial_info::Which::Wss(wss) => {
            let wss = wss.map_err(RPCError::protocol)?;
            let socket_address_reader = wss
                .get_socket_address()
                .map_err(RPCError::map_protocol("missing WSS socketAddress"))?;
            let socket_address = decode_socket_address(&socket_address_reader)?;
            let request = wss
                .get_request()
                .map_err(RPCError::map_protocol("missing WSS request"))?;
            DialInfo::try_wss(
                socket_address,
                request
                    .to_string()
                    .map_err(RPCError::map_protocol("invalid WSS request string"))?,
            )
            .map_err(RPCError::map_protocol("invalid WSS dial info"))
        }
    }
}

pub(crate) fn encode_dial_info(
    dial_info: &DialInfo,
    builder: &mut veilid_capnp::dial_info::Builder,
) -> Result<(), RPCError> {
    match dial_info {
        DialInfo::UDP(udp) => {
            let mut di_udp_builder = builder.reborrow().init_udp();
            encode_socket_address(
                &udp.socket_address,
                &mut di_udp_builder.reborrow().init_socket_address(),
            )?;
        }
        DialInfo::TCP(tcp) => {
            let mut di_tcp_builder = builder.reborrow().init_tcp();
            encode_socket_address(
                &tcp.socket_address,
                &mut di_tcp_builder.reborrow().init_socket_address(),
            )?;
        }
        DialInfo::WS(ws) => {
            let mut di_ws_builder = builder.reborrow().init_ws();
            encode_socket_address(
                &ws.socket_address,
                &mut di_ws_builder.reborrow().init_socket_address(),
            )?;
            let request = dial_info
                .request()
                .ok_or_else(RPCError::else_internal("no request for WS dialinfo"))?;

            let mut requestb = di_ws_builder.init_request(
                request
                    .len()
                    .try_into()
                    .map_err(RPCError::map_protocol("request too long"))?,
            );
            requestb.push_str(request.as_str());
        }
        DialInfo::WSS(wss) => {
            let mut di_wss_builder = builder.reborrow().init_wss();
            encode_socket_address(
                &wss.socket_address,
                &mut di_wss_builder.reborrow().init_socket_address(),
            )?;
            let request = dial_info
                .request()
                .ok_or_else(RPCError::else_internal("no request for WSS dialinfo"))?;

            let mut requestb = di_wss_builder.init_request(
                request
                    .len()
                    .try_into()
                    .map_err(RPCError::map_protocol("request too long"))?,
            );
            requestb.push_str(request.as_str());
        }
    };
    Ok(())
}
