packet! { StatusResponse
    json_response = crate::McString;encode;;decode 255;
}

packet! { Pong
    payload = crate::primitive::McLong;encode;;decode;
}
