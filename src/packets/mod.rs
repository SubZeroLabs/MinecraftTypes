macro_rules! packet {
    {$packet_name:ident $($field:ident = $field_type:ty; $encoder_func:ident; $($param:expr)*;)*} => {
        pub struct $packet_name {
            $(pub $field: $field_type,)*
        }

        #[allow(unused_mut)] // this is purely for empty packets
        impl crate::Encodable for $packet_name {
            fn encode(&self) -> crate::Result<Vec<u8>> {
                let mut encoder = crate::Encoder::new();
                $(encoder.$encoder_func(&self.$field)?;)*
                Ok(encoder.into())
            }
        }

        impl crate::Decodable<$packet_name> for $packet_name {
            fn decode(remaining: Vec<u8>) -> crate::Result<($packet_name, Vec<u8>)> {
                $(let ($field, remaining) = <$field_type>::decode(remaining, $($param,)*)?;)*
                Ok(($packet_name { $($field,)* }, remaining))
            }
        }
    }
}

pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;
