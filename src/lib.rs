pub type Result<T> = std::result::Result<T, String>;

pub trait Decodable<T> {
    /// Decodes the given bytes into type `T` and returns the decoded type `T` and the remaining bytes.
    fn decode(bytes: Vec<u8>) -> Result<(T, Vec<u8>)>;
}

pub trait Encodable<T> {
    /// Encodes `T` into a set of bytes to be sent to the client.
    fn encode(&self) -> Result<Vec<u8>>;
}

pub mod primitive {
    macro_rules! prim_type {
        ($name:ident = $primitive:ty, |$decode_iterator:ident| $decoder:expr, |$encode_self:ident| $encoder:expr) => {
            pub struct $name($primitive);

            impl $name {
                pub fn new(internal: $primitive) -> Self {
                    $name(internal)
                }
            }

            impl Into<$primitive> for $name {
                fn into(self) -> $primitive {
                    self.0
                }
            }

            impl super::Decodable<$name> for $name {
                fn decode(bytes: Vec<u8>) -> super::Result<($name, Vec<u8>)> {
                    let mut $decode_iterator = bytes.into_iter();
                    $decoder
                }
            }

            impl super::Encodable<$name> for $name {
                fn encode($encode_self: &Self) -> super::Result<Vec<u8>> {
                    $encoder
                }
            }
        };
    }

    prim_type!(
        McBoolean = bool,
        |iterator| {
            if let Some(next_byte) = iterator.next() {
                let bool_res = match next_byte {
                    0x00u8 => Ok(false),
                    0x01u8 => Ok(true),
                    _ => Err(String::from("")),
                }?;
                Ok((McBoolean(bool_res), iterator.collect::<Vec<u8>>()))
            } else {
                Err(String::from("Unexpected EOF in decoder."))
            }
        },
        |self| Ok(if self.0 { vec![0x01u8] } else { vec![0x00u8] })
    );
}
