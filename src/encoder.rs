pub trait Decodable: Sized {
    fn decode<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self>;
}

pub trait SizeDecodable: Sized {
    fn decode_sized<R: std::io::Read>(
        reader: &mut R,
        size: &crate::nums::VarInt,
    ) -> anyhow::Result<Self>;
}

pub trait Encodable {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()>;

    fn size(&self) -> anyhow::Result<crate::nums::VarInt>;
}

#[async_trait::async_trait]
pub trait AsyncEncodable: Encodable + Send + Sync {
    async fn async_encode<W: tokio::io::AsyncWrite + Send + Unpin>(
        &self,
        writer: &mut W,
    ) -> anyhow::Result<()>;
}

pub trait SizeEncodable {
    fn encode_sized<W: std::io::Write>(
        &self,
        writer: &mut W,
        size: &crate::nums::VarInt,
    ) -> anyhow::Result<()>;

    fn predicted_size(&self) -> anyhow::Result<crate::nums::VarInt>;
}

#[async_trait::async_trait]
pub trait AsyncSizeEncodable: SizeEncodable + Send + Sync {
    async fn async_encode_sized<W: tokio::io::AsyncWrite + Send + Unpin>(
        &self,
        writer: &mut W,
        size: &crate::nums::VarInt,
    ) -> anyhow::Result<()>;
}
