use crate::VarInt;

auto_string!(LoginNameString, 16);

auto_struct! {
    LoginStart {
        name: LoginNameString,
    }

    EncryptionResponse {
        shared_secret: (VarInt, Vec<u8>),
        verify_token: (VarInt, Vec<u8>),
    }

    LoginPluginResponse {
        message_id: VarInt,
        successful: bool,
        data: Vec<u8>,
    }
}
