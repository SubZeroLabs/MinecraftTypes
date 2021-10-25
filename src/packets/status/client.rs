use crate::{primitive::McLong, McString};

minecraft_struct! { StatusResponse
    json_response = McString;encode;;decode 255;
}

minecraft_struct! { Pong
    payload = McLong;encode;;decode;
}
