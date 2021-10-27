auto_string!(StatusResponseJson, 255);

auto_struct! {
    StatusResponse {
        json_response: StatusResponseJson,
    }

    Pong {
        payload: i64,
    }
}
