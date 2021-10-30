auto_string!(StatusResponseJson, 32767);

auto_struct! {
    StatusResponse {
        json_response: StatusResponseJson,
    }

    Pong {
        payload: i64,
    }
}
