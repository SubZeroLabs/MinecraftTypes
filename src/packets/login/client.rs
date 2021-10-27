use crate::{ChatJson, Identifier, McUuid, VarInt};

auto_string!(EncryptionRequestServerId, 20);
auto_string!(LoginSuccessName, 16);

auto_struct! {
    Disconnect {
        reason: ChatJson,
    }

    EncryptionRequest {
        server_id: EncryptionRequestServerId,
        public_key: (VarInt, Vec<u8>),
        verify_token: (VarInt, Vec<u8>),
    }

    LoginSuccess {
        uuid: McUuid,
        username: LoginSuccessName,
    }

    SetCompression {
        threshold: VarInt,
    }

    LoginPluginRequest {
        message_id: VarInt,
        channel: Identifier,
        data: Vec<u8>,
    }
}
