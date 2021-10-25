use crate::{primitive::McUnsignedShort, McString, VarInt};

minecraft_struct! { Handshake
    protocol_version = VarInt;;decode;
    server_address = McString;;decode 255;
    server_port = McUnsignedShort;;decode;
    next_state = VarInt;;decode;
}
