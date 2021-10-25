packet! { Hanshake
    protocol_version = crate::VarInt;encode;;decode;
    server_address = crate::McString;encode;;decode 255;
    server_port = crate::primitive::McUnsignedShort;encode;;decode;
    next_state = crate::VarInt;encode;;decode;
}
