packet! { StatusRequest
}

packet! { Ping
    payload = crate::primitive::McLong;encode;;decode;
}
