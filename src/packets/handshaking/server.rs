packet! { Hanshake
    protocol_version = crate::VarInt;encode;;
    server_address = crate::McString;encode;255;
    server_port = crate::primitive::McUnsignedShort;encode;;
    next_state = crate::VarInt;encode;;
}