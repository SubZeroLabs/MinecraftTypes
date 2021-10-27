use crate::VarInt;

auto_string!(HandshakeServerAddress, 255);

auto_struct! {
    Handshake {
        protocol_version: VarInt,
        server_address: HandshakeServerAddress,
        server_port: u8,
        next_state: VarInt,
    }
}
