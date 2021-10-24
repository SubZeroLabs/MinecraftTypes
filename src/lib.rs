pub type Result<T> = std::result::Result<T, String>;

pub trait Decodable<T> {
    /// Decodes the given bytes into type `T` and returns the decoded type `T` and the remaining bytes.
    fn decode(bytes: Vec<u8>) -> Result<(T, Vec<u8>)>;
}

pub trait Encodable<T> {
    /// Encodes `T` into a set of bytes to be sent to the client.
    fn encode(&self) -> Result<Vec<u8>>;
}

const UNEXPECTED_EOF: &str = "Unexpected EOF in decoder.";

macro_rules! prim_type {
        ($name:ident = $primitive:ty) => {
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

            impl std::ops::Deref for $name {
                type Target = $primitive;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        };
        ($name:ident = $primitive:ty, |$encode_self:ident| $encoder:expr) => {
            prim_type!($name = $primitive);

            impl super::Encodable<$name> for $name {
                fn encode($encode_self: &Self) -> super::Result<Vec<u8>> {
                    Ok($encoder)
                }
            }
        };
        ($name:ident = $primitive:ty, |$decode_iterator:ident| $decoder:expr, |$encode_self:ident| $encoder:expr) => {
            prim_type!($name = $primitive, |$encode_self| $encoder);

            impl super::Decodable<$name> for $name {
                fn decode(bytes: Vec<u8>) -> super::Result<($name, Vec<u8>)> {
                    let $decode_iterator = bytes.into_iter();
                    $decoder
                }
            }
        };
        ($name:ident = $primitive:ty, |$pre:tt $decode_iterator:ident| $decoder:expr, |$encode_self:ident| $encoder:expr) => {
            prim_type!($name = $primitive, |$encode_self| $encoder);

            impl super::Decodable<$name> for $name {
                fn decode(bytes: Vec<u8>) -> super::Result<($name, Vec<u8>)> {
                    let $pre $decode_iterator = bytes.into_iter();
                    $decoder
                }
            }
        };
    }

pub mod primitive {
    fn require_bytes(
        iterator: impl Iterator<Item = u8>,
        size: usize,
    ) -> super::Result<(Vec<u8>, Vec<u8>)> {
        let vec: Vec<u8> = iterator.collect();
        if vec.len() < size {
            Err(String::from(super::UNEXPECTED_EOF))
        } else if vec.len() == size {
            Ok((vec, vec![]))
        } else {
            Ok((Vec::from(&vec[0..size]), Vec::from(&vec[size..vec.len()])))
        }
    }

    prim_type!(
        McBoolean = bool,
        |mut iterator| {
            if let Some(next_byte) = iterator.next() {
                let bool_res = match next_byte {
                    0x00u8 => Ok(false),
                    0x01u8 => Ok(true),
                    _ => Err(String::from("")),
                }?;
                Ok((McBoolean(bool_res), iterator.collect()))
            } else {
                Err(String::from(super::UNEXPECTED_EOF))
            }
        },
        |self| if self.0 { vec![0x01u8] } else { vec![0x00u8] }
    );

    prim_type!(
        McByte = i8,
        |mut iterator| {
            if let Some(next_byte) = iterator.next() {
                Ok((McByte(next_byte as i8), iterator.collect()))
            } else {
                Err(String::from(super::UNEXPECTED_EOF))
            }
        },
        |self| vec![self.0 as u8]
    );

    prim_type!(
        McUnsignedByte = u8,
        |mut iterator| {
            if let Some(next_byte) = iterator.next() {
                Ok((McUnsignedByte(next_byte), iterator.collect()))
            } else {
                Err(String::from(super::UNEXPECTED_EOF))
            }
        },
        |self| vec![self.0]
    );

    prim_type!(
        McShort = i16,
        |iterator| {
            let (bytes, remaining) = require_bytes(iterator, 2)?;
            let be_bytes = [bytes[0], bytes[1]];
            let res = i16::from_be_bytes(be_bytes);
            Ok((McShort(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McUnsignedShort = u16,
        |iterator| {
            let (bytes, remaining) = require_bytes(iterator, 2)?;
            let be_bytes = [bytes[0], bytes[1]];
            let res = u16::from_be_bytes(be_bytes);
            Ok((McUnsignedShort(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McInteger = i32,
        |iterator| {
            let (bytes, remaining) = require_bytes(iterator, 4)?;
            let be_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
            let res = i32::from_be_bytes(be_bytes);
            Ok((McInteger(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McLong = i64,
        |iterator| {
            let (bytes, remaining) = require_bytes(iterator, 8)?;
            let be_bytes = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let res = i64::from_be_bytes(be_bytes);
            Ok((McLong(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McFloat = f32,
        |iterator| {
            let (bytes, remaining) = require_bytes(iterator, 4)?;
            let be_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
            let res = f32::from_be_bytes(be_bytes);
            Ok((McFloat(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McDouble = f64,
        |iterator| {
            let (bytes, remaining) = require_bytes(iterator, 8)?;
            let be_bytes = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let res = f64::from_be_bytes(be_bytes);
            Ok((McDouble(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );
}

prim_type!(VarInt = i32);
prim_type!(VarLong = i64);

impl Decodable<VarInt> for VarInt {
    fn decode(bytes: Vec<u8>) -> Result<(VarInt, Vec<u8>)> {
        let mut value = 0i32;
        let mut bit_offset = 0u32;
        let mut iter = bytes.into_iter();
        loop {
            if bit_offset == 35 {
                return Err(String::from("VarInt was too big."));
            }

            if let Some(next_byte) = iter.next() {
                value |= i32::from(next_byte & 0b01111111).overflowing_shl(bit_offset).0;
                bit_offset += 7;

                if next_byte & 0b10000000 == 0 {
                    break;
                }
            } else {
                return Err(String::from(UNEXPECTED_EOF));
            }
        };
        Ok((VarInt(value), iter.collect()))
    }
}

impl Encodable<VarInt> for VarInt {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        let mut temp = self.0.clone() as u32;
        loop {
            if temp & 0xFFFFFF80 == 0 {
                vec.push(temp as u8);
                return Ok(vec);
            }

            vec.push((temp & 0x7F | 0x80) as u8);
            temp = temp.overflowing_shr(7).0;
        };
    }
}

impl Decodable<VarLong> for VarLong {
    fn decode(bytes: Vec<u8>) -> Result<(VarLong, Vec<u8>)> {
        let mut value = 0i64;
        let mut bit_offset = 0u32;
        let mut iter = bytes.into_iter();
        loop {
            if bit_offset == 70 {
                return Err(String::from("VarInt was too big."));
            }

            if let Some(next_byte) = iter.next() {
                value |= i64::from(next_byte & 0b01111111).overflowing_shl(bit_offset).0;
                bit_offset += 7;

                if next_byte & 0b10000000 == 0 {
                    break;
                }
            } else {
                return Err(String::from(UNEXPECTED_EOF));
            }
        };
        Ok((VarLong(value), iter.collect()))
    }
}

impl Encodable<VarLong> for VarLong {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        let mut temp = self.0.clone() as u64;
        loop {
            if temp & 0xFFFFFFFFFFFFFF80 == 0 {
                vec.push(temp as u8);
                return Ok(vec);
            }

            vec.push((temp & 0x7F | 0x80) as u8);
            temp = temp.overflowing_shr(7).0;
        };
    }
}
