use crate::{primitive::McUnsignedByte, ChatJson, Decoder, Identifier, McString, McUuid, VarInt};

packet! { Disconnect
    reason = ChatJson;encode;;decode;
}

packet! { EncryptionRequest
    server_id = McString;encode;;decode 20;
    public_key_length = VarInt;encode;;decode;
    public_key = Vec<McUnsignedByte>;encode_arr;Decoder;decode_arr VarInt::from(*public_key_length);
    verify_token_length = VarInt;encode;;decode;
    verify_token = Vec<McUnsignedByte>;encode_arr;Decoder;decode_arr VarInt::from(*verify_token_length);
}

packet! { LoginSuccess
    uuid = McUuid;encode;;decode;
    username = McString;encode;;decode 16;
}

packet! { SetCompression
    threshold = VarInt;encode;;decode;
}

packet! { LoginPluginrequest
    message_id = VarInt;encode;;decode;
    channel = Identifier;encode;;decode;
    data = Vec<McUnsignedByte>;encode_arr;Decoder;decode_to_end;
}
