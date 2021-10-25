macro_rules! nest_decode {
    ($call_type:path,; $decoder:ident) => {
        use $call_type as $decoder;
    };
    ($call_type:path, $decoder_type:path; $decoder:ident) => {
        use $decoder_type as $decoder;
    };
}

macro_rules! minecraft_struct {
    ($packet_name:ident) => {
        pub struct $packet_name;
        impl crate::Encodable for $packet_name {
            fn encode(&self) -> crate::Result<Vec<u8>> {
                Ok(vec![])
            }
        }
        impl crate::Decodable<$packet_name> for $packet_name {
            fn decode(remaining: Vec<u8>) -> crate::Result<($packet_name, Vec<u8>)> {
                Ok(($packet_name {}, remaining))
            }
        }
    };
    {$packet_name:ident $($field:ident = $field_type:path; $($decoder_path:path)?; $decoder_func:ident $($param:expr)*;)*} => {
        pub struct $packet_name {
            $(pub $field: $field_type,)*
        }

        impl crate::Encodable for $packet_name {
            fn encode(&self) -> crate::Result<Vec<u8>> {
                let mut encoder = crate::Encoder::new();
                $(encoder.encode(&self.$field)?;)*
                Ok(encoder.into())
            }
        }

        impl crate::Decodable<$packet_name> for $packet_name {
            fn decode(remaining: Vec<u8>) -> crate::Result<($packet_name, Vec<u8>)> {
                $(let ($field, remaining): ($field_type, Vec<u8>) = {
                    nest_decode!($field_type, $($decoder_path)*; decoder);
                    decoder::$decoder_func(remaining, $($param,)*)?
                    // <$field_type>::decode(remaining, $($param,)*)?;
                };)*
                Ok(($packet_name { $($field,)* }, remaining))
            }
        }
    };
}
