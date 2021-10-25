macro_rules! packet {
    ($packet_name:ident, $self:ident, $encoder:ident, $($field:ident = $field_type:ty; $encoder_func:expr; !$decode_predicate:expr, $alternate:expr,)*) => {
        pub struct $packet_name {
            $(pub $field: $field_type,)*
        }

        #[allow(unreachable_code)]
        impl crate::Encodable for $packet_name {
            fn encode($self: &Self) -> crate::Result<Vec<u8>> {
                let mut $encoder = crate::Encoder::new();
                $(
                    $encoder_func;
                )*
                Ok($encoder.into())
            }
        }

        #[allow(unreachable_code)]
        impl crate::Decodable<$packet_name> for $packet_name {
            fn decode(remaining: Vec<u8>) -> Result<($packet_name, Vec<u8>)> {
                $(
                    let ($field, remaining) = if $decode_predicate {
                        <$field_type>::decode(remaining)?
                    } else {
                        ($alternate, remaining)
                    };
                )*

                Ok(($packet_name {
                    $($field,)*
                }, remaining))
            }
        }
    }
}

pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;