use crate::primitive::McLong;

packet!(StatusRequest);

packet! { Ping
    payload = McLong;encode;;decode;
}
