packet! { StatusResponse
    json_response = crate::McString;encode;255;
}

packet! { Pong
    payload = crate::primitive::McLong;encode;;
}
