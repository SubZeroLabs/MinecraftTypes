use crate::{primitive::McLong, McString};

packet! { StatusResponse
    json_response = McString;encode;;decode 255;
}

packet! { Pong
    payload = McLong;encode;;decode;
}
