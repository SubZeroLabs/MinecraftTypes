use crate::{primitive::McUnsignedByte, ChatJson, Decoder, Identifier, McString, McUuid, VarInt};

minecraft_struct! { Disconnect
    reason = ChatJson;;decode;
}

minecraft_struct! { EncryptionRequest
    server_id = McString;;decode 20;
    public_key_length = VarInt;;decode;
    public_key = Vec<McUnsignedByte>;Decoder;decode_arr VarInt::from(*public_key_length);
    verify_token_length = VarInt;;decode;
    verify_token = Vec<McUnsignedByte>;Decoder;decode_arr VarInt::from(*verify_token_length);
}

minecraft_struct! { LoginSuccess
    uuid = McUuid;;decode;
    username = McString;;decode 16;
}

minecraft_struct! { SetCompression
    threshold = VarInt;;decode;
}

minecraft_struct! { LoginPluginrequest
    message_id = VarInt;;decode;
    channel = Identifier;;decode;
    data = Vec<McUnsignedByte>;Decoder;decode_to_end;
}
