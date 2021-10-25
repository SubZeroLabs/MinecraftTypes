use crate::{primitive::McUnsignedByte, ChatJson, Decoder, Identifier, McString, McUuid, VarInt};

minecraft_struct! { Disconnect
    reason = ChatJson;encode;;decode;
}

minecraft_struct! { EncryptionRequest
    server_id = McString;encode;;decode 20;
    public_key_length = VarInt;encode;;decode;
    public_key = Vec<McUnsignedByte>;encode_arr;Decoder;decode_arr VarInt::from(*public_key_length);
    verify_token_length = VarInt;encode;;decode;
    verify_token = Vec<McUnsignedByte>;encode_arr;Decoder;decode_arr VarInt::from(*verify_token_length);
}

minecraft_struct! { LoginSuccess
    uuid = McUuid;encode;;decode;
    username = McString;encode;;decode 16;
}

minecraft_struct! { SetCompression
    threshold = VarInt;encode;;decode;
}

minecraft_struct! { LoginPluginrequest
    message_id = VarInt;encode;;decode;
    channel = Identifier;encode;;decode;
    data = Vec<McUnsignedByte>;encode_arr;Decoder;decode_to_end;
}
