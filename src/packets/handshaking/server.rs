use crate::VarInt;

auto_string!(HandshakeServerAddress, 255);

simple_auto_enum! {
    NextState; VarInt {
        1 => Status,
        2 => Login,
    }
}

auto_struct! {
    Handshake {
        protocol_version: VarInt,
        server_address: HandshakeServerAddress,
        server_port: u16,
        next_state: NextState,
    }
}
