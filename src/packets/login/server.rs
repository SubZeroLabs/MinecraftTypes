use crate::{
    primitive::{McBoolean, McUnsignedByte},
    Decoder, McString, VarInt,
};

minecraft_struct! { LoginStart
    name = McString;;decode 16;
}

minecraft_struct! { EncryptionResponse
    shared_secret_length = VarInt;;decode;
    shared_secret = Vec<McUnsignedByte>;Decoder;decode_arr VarInt::from(*shared_secret_length);
    verify_token_length = VarInt;;decode;
    verify_token = Vec<McUnsignedByte>;Decoder;decode_arr VarInt::from(*verify_token_length);
}

minecraft_struct! { LoginPluginResponse
    message_id = VarInt;;decode;
    successful = McBoolean;;decode;
    data = Vec<McUnsignedByte>;Decoder;decode_to_end;
}
