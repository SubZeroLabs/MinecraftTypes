use anyhow::Context;
use std::io::{Read, Write};

#[macro_use]
pub mod macros;
pub mod base_types;
pub mod packets;

pub trait Decodable {
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait IndexDecodable {
    fn decode_index(reader: &mut impl Read, index: &VarInt) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait SizeDecodable {
    fn decode_sized(reader: &mut impl Read, size: &VarInt) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait Encodable {
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()>;

    fn size(&self) -> anyhow::Result<VarInt>;
}

pub trait SizeEncodable {
    fn encode_sized(&self, writer: &mut impl Write, size: &VarInt) -> anyhow::Result<()>;

    fn predicted_size(&self) -> anyhow::Result<VarInt>;
}

// primitives

impl Decodable for bool {
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let byte = u8::decode(reader)?;
        if byte == 0x0u8 {
            Ok(false)
        } else if byte == 0x1u8 {
            Ok(true)
        } else {
            anyhow::bail!("Malformed boolean found. Byte {}", byte);
        }
    }
}

impl Encodable for bool {
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        writer
            .write_all(&[*self as u8])
            .context(format!("Failed to write {} into buffer.", &self))
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(VarInt::from(1))
    }
}

declare_primitives!(
    |i8;1|
    |u8;1|
    |i16;2|
    |u16;2|
    |i32;4|
    |i64;8|
    |f32;4|
    |f64;8|
);

use std::fmt::{Display, Formatter, Result};
use uuid::{Builder, Uuid};
declare_variable_number!(VarInt, i32, 35, u32, 0xFFFFFF80);
declare_variable_number!(VarLong, i64, 70, u64, 0xFFFFFFFFFFFFFF80);

// exported functions

impl<T> Decodable for Vec<T>
where
    T: Decodable,
{
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let mut items: Vec<T> = Vec::new();
        let mut remaining_bytes = Vec::new();
        let length = reader.read_to_end(&mut remaining_bytes)? as u64;
        let mut cursor = std::io::Cursor::new(remaining_bytes);

        while cursor.position() < length {
            items.push(T::decode(&mut cursor)?);
        }
        Ok(items)
    }
}

impl<T> Encodable for Vec<T>
where
    T: Encodable,
{
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        for item in self {
            item.encode(writer)?;
        }
        Ok(())
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        let mut size = VarInt::from(0);
        for item in self {
            size = size + item.size()?;
        }
        Ok(size)
    }
}

impl<T> SizeDecodable for Vec<T>
where
    T: Decodable,
{
    fn decode_sized(reader: &mut impl Read, size: &VarInt) -> anyhow::Result<Self> {
        let mut items = Vec::with_capacity(**size as usize);
        for _ in 0..**size {
            items.push(T::decode(reader)?);
        }
        Ok(items)
    }
}

impl<T> SizeEncodable for Vec<T>
where
    T: Encodable,
{
    fn encode_sized(&self, writer: &mut impl Write, size: &VarInt) -> anyhow::Result<()> {
        size.encode(writer)?;
        for item in self {
            item.encode(writer)?;
        }
        Ok(())
    }

    fn predicted_size(&self) -> anyhow::Result<VarInt> {
        let mut size = VarInt::from(0);
        for item in self {
            size = size + item.size()?;
        }
        Ok(size)
    }
}

impl<T> Decodable for Option<T>
where
    T: Decodable,
{
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let result = T::decode(reader)?;
        Ok(Some(result))
    }
}

impl<T> Encodable for Option<T>
where
    T: Encodable,
{
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        if let Some(item) = self {
            item.encode(writer)?;
        }
        Ok(())
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        if let Some(item) = self {
            item.size()
        } else {
            Ok(VarInt::from(0))
        }
    }
}

impl<T> Decodable for (VarInt, T)
where
    T: SizeDecodable,
{
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let size = VarInt::decode(reader)?;
        let item = T::decode_sized(reader, &size)?;
        Ok((size, item))
    }
}

impl<T> Encodable for (VarInt, T)
where
    T: SizeEncodable,
{
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        self.1.encode_sized(writer, &self.0)
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(self.0.size()? + self.1.predicted_size()?)
    }
}

// This section is used for entity metadata

impl<T> Encodable for (bool, Option<T>)
where
    T: Encodable,
{
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        self.0.encode(writer)?;
        if self.0 {
            match &self.1 {
                Some(item) => item.encode(writer),
                None => anyhow::bail!("Expected some value but found None."),
            }
        } else {
            Ok(())
        }
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        let size = self.0.size()?;
        if self.0 {
            match &self.1 {
                Some(item) => Ok(size + item.size()?),
                None => Ok(size),
            }
        } else {
            Ok(size)
        }
    }
}

impl<T> Decodable for (bool, Option<T>)
where
    T: Decodable,
{
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let present = bool::decode(reader)?;
        if present {
            let item = T::decode(reader)?;
            Ok((true, Some(item)))
        } else {
            Ok((false, None))
        }
    }
}

impl<X, Y, Z> Encodable for (X, Y, Z)
where
    X: Encodable,
    Y: Encodable,
    Z: Encodable,
{
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        self.0.encode(writer)?;
        self.1.encode(writer)?;
        self.2.encode(writer)?;
        Ok(())
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(self.0.size()? + self.1.size()? + self.2.size()?)
    }
}

impl<X, Y, Z> Decodable for (X, Y, Z)
where
    X: Decodable,
    Y: Decodable,
    Z: Decodable,
{
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok((X::decode(reader)?, Y::decode(reader)?, Z::decode(reader)?))
    }
}

// Strings

pub trait McString: Sized {
    fn new(internal: String) -> Self;

    fn string(&self) -> &String;

    fn limit() -> VarInt;
}

impl<T> Decodable for T
where
    T: McString,
{
    fn decode(reader: &mut impl Read) -> anyhow::Result<T> {
        let true_size = VarInt::decode(reader)?;

        if true_size > T::limit() * 4 {
            anyhow::bail!(
                "Failed to construct string with limit {} with given size {}.",
                T::limit(),
                true_size
            );
        }

        let mut bytes = vec![0u8; *true_size as usize];
        reader.read_exact(&mut bytes).context(format!(
            "Unexpected EOF while decoding string with size {}.",
            true_size
        ))?;
        let internal = String::from_utf8(bytes).context("Failed to build UTF-8 encoded string.")?;

        Ok(T::new(internal))
    }
}

impl<T> Encodable for T
where
    T: McString,
{
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        let bytes = self.string().as_bytes();
        let length = VarInt::from(bytes.len() as i32);
        if length > T::limit() {
            anyhow::bail!(
                "Failed to encode string with limit {} with given size {}.",
                T::limit(),
                bytes.len()
            );
        }

        length.encode(writer)?;
        writer.write_all(bytes)?;

        Ok(())
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(VarInt::from(self.string().len()).size()? + VarInt::from(self.string().len()))
    }
}

auto_string!(ChatJson, 262144);
auto_string!(Identifier, 32767);
auto_string!(BigString, 32767);

#[derive(Debug)]
pub struct McUuid(Uuid);

impl McUuid {
    pub fn new(uuid: Uuid) -> Self {
        McUuid(uuid)
    }
}

impl Encodable for McUuid {
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        writer
            .write_all(&*self.0.as_bytes())
            .context("Failed to encode uuid bytes.")
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(VarInt::from(16))
    }
}

impl Decodable for McUuid {
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let mut uuid: [u8; 16] = [0u8; 16];
        reader
            .read_exact(&mut uuid)
            .context("Unexpected EOF while decoding 16 bytes for UUID.")?;
        Ok(McUuid(Builder::from_bytes(uuid).build()))
    }
}

impl std::ops::Deref for McUuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Angle(u8);

impl Angle {
    pub fn new(byte: u8) -> Self {
        Angle(byte)
    }
}

impl Encodable for Angle {
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        self.0.encode(writer)
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(VarInt::from(1))
    }
}

impl Decodable for Angle {
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let result = u8::decode(reader)?;
        Ok(Angle(result))
    }
}

impl std::ops::Deref for Angle {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Position(i64, i64, i64);

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Position(x, y, z)
    }
}

impl Encodable for Position {
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        let mut long: i64 = 0;
        long = long | (i64::from(self.0) & 0x3FFFFFF).overflowing_shl(38).0;
        long = long | ((i64::from(self.1) & 0x3FFFFFF).overflowing_shl(12).0);
        long = long | (i64::from(self.2) & 0xFFF);
        long.encode(writer)
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        i64::size(&0i64)
    }
}

impl Decodable for Position {
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let long = i64::decode(reader)?;
        let x = long.overflowing_shr(38).0;
        let y = long & 0xFFF;
        let z = long.overflowing_shl(26).0.overflowing_shr(38).0;
        Ok(Position(x, y, z))
    }
}

#[derive(Debug)]
pub struct NbtTag(nbt::Blob);

impl NbtTag {
    pub fn new(blob: nbt::Blob) -> Self {
        NbtTag(blob)
    }
}

impl Encodable for NbtTag {
    fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        self.0
            .to_writer(writer)
            .context("Failed to encode NBT tag to buffer.")
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(VarInt::from(self.0.len_bytes()))
    }
}

impl Decodable for NbtTag {
    fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
        let blob =
            nbt::Blob::from_reader(reader).context("Failed to decode NBT tag from buffer.")?;
        Ok(NbtTag(blob))
    }
}
