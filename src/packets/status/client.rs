use crate::{primitive::McLong, McString};

minecraft_struct! { StatusResponse
    json_response = McString;;decode 255;
}

minecraft_struct! { Pong
    payload = McLong;;decode;
}
