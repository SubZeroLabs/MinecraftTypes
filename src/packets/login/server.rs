use crate::{
    primitive::{McBoolean, McUnsignedByte},
    Decoder, McString, VarInt,
};

minecraft_struct! { LoginStart
    name = McString;encode;;decode 16;
}

minecraft_struct! { EncryptionResponse
    shared_secret_length = VarInt;encode;;decode;
    shared_secret = Vec<McUnsignedByte>;encode_arr;Decoder;decode_arr VarInt::from(*shared_secret_length);
    verify_token_length = VarInt;encode;;decode;
    verify_token = Vec<McUnsignedByte>;encode_arr;Decoder;decode_arr VarInt::from(*verify_token_length);
}

minecraft_struct! { LoginPluginResponse
    message_id = VarInt;encode;;decode;
    successful = McBoolean;encode;;decode;
    data = Vec<McUnsignedByte>;encode_arr;Decoder;decode_to_end;
}
