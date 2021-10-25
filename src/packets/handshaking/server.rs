use crate::{primitive::McUnsignedShort, McString, VarInt};

minecraft_struct! { Handshake
    protocol_version = VarInt;encode;;decode;
    server_address = McString;encode;;decode 255;
    server_port = McUnsignedShort;encode;;decode;
    next_state = VarInt;encode;;decode;
}
