use std::io::Write;
use std::ops::Deref;
use uuid::{Builder, Uuid};

#[macro_use]
pub mod packets;

pub type Result<T> = std::result::Result<T, String>;

pub trait Decodable<T> {
    /// Decodes the given bytes into type `T` and returns the decoded type `T` and the remaining bytes.
    fn decode(bytes: Vec<u8>) -> Result<(T, Vec<u8>)>;
}

pub trait Encodable {
    /// Encodes struct into a set of bytes to be sent to the client.
    fn encode(&self) -> Result<Vec<u8>>;
}

pub struct Decoder;

impl Decoder {
    pub fn decode<T>(bytes: Vec<u8>) -> Result<(T, Vec<u8>)>
        where T: Decodable<T>
    {
        T::decode(bytes)
    }

    pub fn decode_arr<T>(bytes: Vec<u8>, size: VarInt) -> Result<(Vec<T>, Vec<u8>)>
        where T: Decodable<T>
    {
        let mut result = Vec::new();
        let mut remaining = bytes;
        for _ in 0..*size {
            let (output, leftover) = Decoder::decode::<T>(remaining)?;
            result.push(output);
            remaining = leftover;
        }
        Ok((result, remaining))
    }
}

pub struct Encoder {
    internal_vec: std::io::Cursor<Vec<u8>>,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            internal_vec: std::io::Cursor::new(Vec::new()),
        }
    }

    pub fn encode(&mut self, encodable: &impl Encodable) -> Result<()> {
        let mut encoded = encodable.encode()?;
        if let Ok(_) = self.internal_vec.write_all(&mut encoded) {
            Ok(())
        } else {
            Err(String::from("Error writing to encoder."))
        }
    }

    pub fn encode_if(&mut self, encodable: &Option<impl Encodable>) -> Result<()> {
        if let Some(encodable) = encodable {
            self.encode(encodable)
        } else {
            Ok(())
        }
    }

    pub fn encode_arr(&mut self, encodable: &Vec<impl Encodable>) -> Result<()> {
        for item in encodable {
            self.encode(item)?
        }
        Ok(())
    }
}

impl Into<Vec<u8>> for Encoder {
    fn into(self) -> Vec<u8> {
        self.internal_vec.into_inner()
    }
}

const UNEXPECTED_EOF: &str = "Unexpected EOF in decoder.";

impl Encodable for u8 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![*self])
    }
}

impl Encodable for [u8] {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(Vec::from(self))
    }
}

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

            impl From<$primitive> for $name {
                fn from(internal: $primitive) -> Self {
                    $name(internal)
                }
            }
        };
        ($name:ident = $primitive:ty, |$encode_self:ident| $encoder:expr) => {
            prim_type!($name = $primitive);

            impl super::Encodable for $name {
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

fn require_bytes(iterator: impl Iterator<Item = u8>, size: usize) -> Result<(Vec<u8>, Vec<u8>)> {
    let vec: Vec<u8> = iterator.collect();
    if vec.len() < size {
        Err(String::from(UNEXPECTED_EOF))
    } else if vec.len() == size {
        Ok((vec, vec![]))
    } else {
        Ok((Vec::from(&vec[0..size]), Vec::from(&vec[size..vec.len()])))
    }
}

pub mod primitive {
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
            let (bytes, remaining) = super::require_bytes(iterator, 2)?;
            let be_bytes = [bytes[0], bytes[1]];
            let res = i16::from_be_bytes(be_bytes);
            Ok((McShort(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McUnsignedShort = u16,
        |iterator| {
            let (bytes, remaining) = super::require_bytes(iterator, 2)?;
            let be_bytes = [bytes[0], bytes[1]];
            let res = u16::from_be_bytes(be_bytes);
            Ok((McUnsignedShort(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McInteger = i32,
        |iterator| {
            let (bytes, remaining) = super::require_bytes(iterator, 4)?;
            let be_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
            let res = i32::from_be_bytes(be_bytes);
            Ok((McInteger(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McLong = i64,
        |iterator| {
            let (bytes, remaining) = super::require_bytes(iterator, 8)?;
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
            let (bytes, remaining) = super::require_bytes(iterator, 4)?;
            let be_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
            let res = f32::from_be_bytes(be_bytes);
            Ok((McFloat(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );

    prim_type!(
        McDouble = f64,
        |iterator| {
            let (bytes, remaining) = super::require_bytes(iterator, 8)?;
            let be_bytes = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let res = f64::from_be_bytes(be_bytes);
            Ok((McDouble(res), remaining))
        },
        |self| Vec::from(self.0.to_be_bytes())
    );
}

macro_rules! var_num {
    ($name:ident, $primitive_signed:ty, $bit_limit:literal, $primitive_unsigned:ty, $and_check:literal) => {
        prim_type!($name = $primitive_signed);

        impl Decodable<$name> for $name {
            fn decode(bytes: Vec<u8>) -> Result<($name, Vec<u8>)> {
                let mut value: $primitive_signed = 0;
                let mut bit_offset = 0u32;
                let mut iter = bytes.into_iter();
                loop {
                    if bit_offset == $bit_limit {
                        return Err(format!(
                            "Variable number was too big, expected {}.",
                            $bit_limit
                        ));
                    }

                    if let Some(next_byte) = iter.next() {
                        value |= <$primitive_signed>::from(next_byte & 0b01111111)
                            .overflowing_shl(bit_offset)
                            .0;
                        bit_offset += 7;

                        if next_byte & 0b10000000 == 0 {
                            break;
                        }
                    } else {
                        return Err(String::from(UNEXPECTED_EOF));
                    }
                }
                Ok(($name(value), iter.collect()))
            }
        }
        impl Encodable for $name {
            fn encode(&self) -> Result<Vec<u8>> {
                let mut vec = Vec::new();
                let mut temp = self.0.clone() as $primitive_unsigned;
                loop {
                    if temp & $and_check == 0 {
                        vec.push(temp as u8);
                        return Ok(vec);
                    }

                    vec.push((temp & 0x7F | 0x80) as u8);
                    temp = temp.overflowing_shr(7).0;
                }
            }
        }

        impl From<$primitive_unsigned> for $name {
            fn from(internal: $primitive_unsigned) -> Self {
                $name(internal as $primitive_signed)
            }
        }
    };
}

var_num!(VarInt, i32, 35, u32, 0xFFFFFF80);
var_num!(VarLong, i64, 70, u64, 0xFFFFFFFFFFFFFF80);

pub struct McString(String);

impl McString {
    pub fn new(internal: String) -> Self {
        McString(internal)
    }

    pub fn decode(bytes: Vec<u8>, max_length: u32) -> Result<(McString, Vec<u8>)> {
        let (size, remaining) = VarInt::decode(bytes)?;
        let true_size = *size as u32;
        if true_size > max_length * 4 {
            return Err(format!(
                "Failed to validate expected string length {} with decoder.",
                max_length
            ));
        }
        let (string_bytes, remaining) = require_bytes(remaining.into_iter(), true_size as usize)?;
        if let Ok(internal) = String::from_utf8(string_bytes) {
            Ok((McString(internal), remaining))
        } else {
            Err(String::from(
                "Failed to create UTF-8 string from decoded data.",
            ))
        }
    }
}

impl Encodable for McString {
    fn encode(&self) -> Result<Vec<u8>> {
        let var_int_bytes = VarInt(self.0.len() as i32).encode()?;
        let string_bytes = self.0.as_bytes();
        Ok(Vec::from([&var_int_bytes, string_bytes].concat()))
    }
}

impl Deref for McString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String> for McString {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for McString {
    fn from(internal: String) -> Self {
        McString(internal)
    }
}

impl From<&str> for McString {
    fn from(internal: &str) -> Self {
        McString(String::from(internal))
    }
}

macro_rules! string_type {
    ($name:ident, $limit:literal) => {
        pub struct $name(McString);

        impl $name {
            pub fn new(internal: McString) -> Self {
                $name(internal)
            }
        }

        impl Encodable for $name {
            fn encode(&self) -> Result<Vec<u8>> {
                self.0.encode()
            }
        }

        impl Decodable<$name> for $name {
            fn decode(bytes: Vec<u8>) -> Result<($name, Vec<u8>)> {
                match McString::decode(bytes, $limit) {
                    Ok((mc_string, remaining)) => Ok(($name(mc_string), remaining)),
                    Err(e) => Err(e),
                }
            }
        }

        impl Deref for $name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Into<String> for $name {
            fn into(self) -> String {
                self.0.into()
            }
        }

        impl From<String> for $name {
            fn from(internal: String) -> Self {
                $name(McString(internal))
            }
        }

        impl From<&str> for $name {
            fn from(internal: &str) -> Self {
                $name(McString(String::from(internal)))
            }
        }
    };
}

string_type!(ChatJson, 262144);
string_type!(Identifier, 32767);

pub struct McUuid(Uuid);

impl McUuid {
    pub fn new(uuid: Uuid) -> Self {
        McUuid(uuid)
    }
}

impl Encodable for McUuid {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(Vec::from(*self.0.as_bytes()))
    }
}

impl Decodable<McUuid> for McUuid {
    fn decode(bytes: Vec<u8>) -> Result<(McUuid, Vec<u8>)> {
        let (uuid_bytes, remaining) = require_bytes(bytes.into_iter(), 16)?;
        let uuid: [u8; 16] = [
            uuid_bytes[0],
            uuid_bytes[1],
            uuid_bytes[2],
            uuid_bytes[3],
            uuid_bytes[4],
            uuid_bytes[5],
            uuid_bytes[6],
            uuid_bytes[7],
            uuid_bytes[8],
            uuid_bytes[9],
            uuid_bytes[10],
            uuid_bytes[11],
            uuid_bytes[12],
            uuid_bytes[13],
            uuid_bytes[14],
            uuid_bytes[15],
        ];
        return Ok((McUuid(Builder::from_bytes(uuid).build()), remaining));
    }
}

impl Deref for McUuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Angle(primitive::McUnsignedByte);

impl Angle {
    pub fn new(byte: primitive::McUnsignedByte) -> Self {
        Angle(byte)
    }
}

impl Encodable for Angle {
    fn encode(&self) -> Result<Vec<u8>> {
        self.0.encode()
    }
}

impl Decodable<Angle> for Angle {
    fn decode(bytes: Vec<u8>) -> Result<(Angle, Vec<u8>)> {
        let (mc_unsigned_byte, remaining) = primitive::McUnsignedByte::decode(bytes)?;
        Ok((Angle(mc_unsigned_byte), remaining))
    }
}

impl Deref for Angle {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Position(i64, i64, i64);

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Position(x, y, z)
    }
}

impl Encodable for Position {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut long: i64 = 0;
        long = long | (i64::from(self.0) & 0x3FFFFFF).overflowing_shl(38).0;
        long = long | ((i64::from(self.1) & 0x3FFFFFF).overflowing_shl(12).0);
        long = long | (i64::from(self.2) & 0xFFF);
        primitive::McLong::encode(&primitive::McLong::from(long))
    }
}

impl Decodable<Position> for Position {
    fn decode(bytes: Vec<u8>) -> Result<(Position, Vec<u8>)> {
        let (long, remaining) = primitive::McLong::decode(bytes)?;
        let long = *long;
        let x = long.overflowing_shr(38).0;
        let y = long & 0xFFF;
        let z = long.overflowing_shl(26).0.overflowing_shr(38).0;
        Ok((Position(x, y, z), remaining))
    }
}

pub struct NbtTag(nbt::Blob);

impl NbtTag {
    pub fn new(blob: nbt::Blob) -> Self {
        NbtTag(blob)
    }
}

impl Encodable for NbtTag {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut dst = Vec::new();
        match self.0.to_writer(&mut dst) {
            Ok(()) => Ok(dst),
            Err(_) => Err(String::from("Failed to write nbt data to bytes.")),
        }
    }
}

impl Decodable<NbtTag> for NbtTag {
    fn decode(bytes: Vec<u8>) -> Result<(NbtTag, Vec<u8>)> {
        let mut src = std::io::Cursor::new(bytes);
        match nbt::Blob::from_reader(&mut src) {
            Ok(blob) => Ok((NbtTag(blob), src.into_inner())),
            Err(_) => Err(String::from("Error reading blob nbt.")),
        }
    }
}

pub struct SlotData {
    present: bool,
    item_id: Option<VarInt>,
    count: Option<primitive::McUnsignedByte>,
    tag: Option<NbtTag>,
}

impl SlotData {
    pub fn new() -> Self {
        SlotData {
            present: false,
            item_id: None,
            count: None,
            tag: None,
        }
    }

    pub fn item(item_id: VarInt, count: primitive::McUnsignedByte, tag: NbtTag) -> Self {
        SlotData {
            present: true,
            item_id: Some(item_id),
            count: Some(count),
            tag: Some(tag),
        }
    }
}

impl Encodable for SlotData {
    fn encode(&self) -> Result<Vec<u8>> {
        if self.present {
            let mut encoder = Encoder::new();
            encoder.encode(&primitive::McBoolean::new(true))?;
            encoder.encode_if(&self.item_id)?;
            encoder.encode_if(&self.count)?;
            encoder.encode_if(&self.tag)?;
            Ok(encoder.into())
        } else {
            primitive::McBoolean::new(false).encode()
        }
    }
}

impl Decodable<SlotData> for SlotData {
    fn decode(bytes: Vec<u8>) -> Result<(SlotData, Vec<u8>)> {
        let (mc_bool, remaining) = primitive::McBoolean::decode(bytes)?;
        if *mc_bool {
            let (item_id, remaining) = VarInt::decode(remaining)?;
            let (count, remaining) = primitive::McUnsignedByte::decode(remaining)?;
            let (tag, remaining) = NbtTag::decode(remaining)?;
            Ok((SlotData::item(item_id, count, tag), remaining))
        } else {
            Ok((SlotData::new(), remaining))
        }
    }
}
