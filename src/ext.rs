use crate::encoder::*;
use crate::nums::VarInt;
use std::convert::TryInto;
use std::io::{Read, Write};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use nbt::Blob;
use uuid::Uuid;
use anyhow::Context;

impl<T> Decodable for Vec<T>
    where
        T: Decodable,
{
    fn decode<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
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
    fn encode<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        for item in self {
            item.encode(writer)?;
        }
        Ok(())
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        self.iter()
            .map(|item| item.size())
            .try_fold(0.into(), |bubble, item| Ok(bubble + item?))
    }
}

#[async_trait::async_trait]
impl<T> AsyncEncodable for Vec<T>
    where
        T: AsyncEncodable,
{
    async fn async_encode<W: AsyncWrite + Send + Unpin>(
        &self,
        writer: &mut W,
    ) -> anyhow::Result<()> {
        for item in self {
            item.async_encode(writer).await?;
        }
        Ok(())
    }
}

impl<T> SizeDecodable for Vec<T>
    where
        T: Decodable,
{
    fn decode_sized<R: Read>(reader: &mut R, size: &VarInt) -> anyhow::Result<Self> {
        let mut items = Vec::with_capacity(size.try_into()?);
        for _ in 0..size.into() {
            items.push(T::decode(reader)?);
        }
        Ok(items)
    }
}

impl<T> SizeEncodable for Vec<T>
    where
        T: Encodable,
{
    fn encode_sized<W: Write>(&self, writer: &mut W, size: &VarInt) -> anyhow::Result<()> {
        size.encode(writer)?;
        self.iter().try_for_each(|item| item.encode(writer))
    }

    fn predicted_size(&self) -> anyhow::Result<VarInt> {
        let mut size = self.size()?;
        size += size.size()?;
        Ok(size)
    }
}

#[async_trait::async_trait]
impl<T> AsyncSizeEncodable for Vec<T>
    where
        T: AsyncEncodable,
{
    async fn async_encode_sized<W: AsyncWrite + Send + Unpin>(
        &self,
        writer: &mut W,
        size: &VarInt,
    ) -> anyhow::Result<()> {
        size.async_encode(writer).await?;
        for item in self {
            item.async_encode(writer).await?;
        }
        Ok(())
    }
}

impl<T> Decodable for (VarInt, T) where T: SizeDecodable {
    fn decode<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        let size = VarInt::decode(reader)?;
        let item = T::decode_sized(reader, &size)?;
        Ok((size, item))
    }
}

impl<T> Encodable for (VarInt, T) where T: SizeEncodable {
    fn encode<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.1.encode_sized(writer, &self.0)
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        self.1.predicted_size()
    }
}

#[async_trait::async_trait]
impl<T> AsyncEncodable for (VarInt, T) where T: AsyncSizeEncodable + Send + Sync {
    async fn async_encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.1.async_encode_sized(writer, &self.0).await
    }
}

impl<T> Decodable for Option<T> where T: Decodable {
    fn decode<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(Some(T::decode(reader)?))
    }
}

impl<T> Encodable for Option<T> where T: Encodable {
    fn encode<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.as_ref().map_or(Ok(()), |item| item.encode(writer))
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        self.as_ref().map_or(Ok(VarInt::from(0)), |item| item.size())
    }
}

#[async_trait::async_trait]
impl<T> AsyncEncodable for Option<T> where T: AsyncEncodable {
    async fn async_encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> anyhow::Result<()> {
        if let Some(item) = self {
            item.async_encode(writer).await
        } else {
            Ok(())
        }
    }
}

impl<T> Decodable for (bool, Option<T>) where T: Decodable {
    fn decode<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        let present = bool::decode(reader)?;
        if present {
            Ok((true, Some(T::decode(reader)?)))
        } else {
            Ok((false, None))
        }
    }
}

impl<T> Encodable for (bool, Option<T>) where T: Encodable {
    fn encode<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
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
                None => Ok(size)
            }
        } else {
            Ok(size)
        }
    }
}

#[async_trait::async_trait]
impl<T> AsyncEncodable for (bool, Option<T>) where T: AsyncEncodable {
    async fn async_encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.0.async_encode(writer).await?;
        if self.0 {
            match &self.1 {
                Some(item) => item.async_encode(writer).await,
                None => anyhow::bail!("Expected some value but found None."),
            }
        } else {
            Ok(())
        }
    }
}

impl<X, Y, Z> Decodable for (X, Y, Z) where X: Decodable, Y: Decodable, Z: Decodable {
    fn decode<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        Ok((X::decode(reader)?, Y::decode(reader)?, Z::decode(reader)?))
    }
}

impl<X, Y, Z> Encodable for (X, Y, Z) where X: Encodable, Y: Encodable, Z: Encodable {
    fn encode<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.0.encode(writer)?;
        self.1.encode(writer)?;
        self.2.encode(writer)
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(self.0.size()? + self.1.size()? + self.2.size()?)
    }
}

#[async_trait::async_trait]
impl<X, Y, Z> AsyncEncodable for (X, Y, Z) where X: AsyncEncodable, Y: AsyncEncodable, Z: AsyncEncodable {
    async fn async_encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.0.async_encode(writer).await?;
        self.1.async_encode(writer).await?;
        self.2.async_encode(writer).await
    }
}

impl Decodable for Uuid {
    fn decode<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        let mut bytes: [u8; 16] = [0u8; 16];
        reader.read_exact(&mut bytes)?;
        Ok(Uuid::from_bytes(bytes))
    }
}

impl Encodable for Uuid {
    fn encode<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        writer.write_all(self.as_bytes()).context(format!("Failed to write {:?} into bytes.", self))
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(VarInt::from(16))
    }
}

#[async_trait::async_trait]
impl AsyncEncodable for Uuid {
    async fn async_encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> anyhow::Result<()> {
        writer.write_all(self.as_bytes()).await.context(format!("Failed to write {:?} into bytes.", self))
    }
}

impl Decodable for Blob {
    fn decode<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        Blob::from_reader(reader).context("Failed to read bytes into nbt string.")
    }
}

impl Encodable for Blob {
    fn encode<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.to_writer(writer).context("Failed to write nbt string into bytes.")
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        self.len_bytes().try_into().context(format!("Failed to turn {} into a VarInt.", self.len_bytes()))
    }
}

#[async_trait::async_trait]
impl AsyncEncodable for Blob {
    async fn async_encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> anyhow::Result<()> {
        let mut vec = Vec::with_capacity(self.len_bytes());
        self.to_writer(&mut vec)?;
        writer.write_all(&vec).await.context("Failed to write nbt string into bytes.")
    }
}
