use crate::VarInt;

auto_string!(LoginNameString, 16);

pub type SharedSecret = (VarInt, Vec<u8>);

auto_struct! {
    LoginStart {
        name: LoginNameString,
    }

    EncryptionResponse {
        shared_secret: SharedSecret,
        verify_token: (VarInt, Vec<u8>),
    }

    LoginPluginResponse {
        message_id: VarInt,
        successful: bool,
        data: Vec<u8>,
    }
}
