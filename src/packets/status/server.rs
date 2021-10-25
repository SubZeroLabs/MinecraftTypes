use crate::primitive::McLong;

minecraft_struct!(StatusRequest);

minecraft_struct! { Ping
    payload = McLong;encode;;decode;
}
