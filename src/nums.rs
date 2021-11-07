use anyhow::Context;
use std::convert::TryFrom;

macro_rules! declare_primitives {
    ($(|$prim:ty;$size:literal|)+) => {
        $(
            impl $crate::encoder::Decodable for $prim {
                fn decode<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                    let mut into = [0u8; $size];
                    reader.read_exact(&mut into).context(format!("Unexpected EOF while reading {} from buffer.", stringify!($prim)))?;
                    Ok(<$prim>::from_be_bytes(into))
                }
            }

            impl $crate::encoder::Encodable for $prim {
                fn encode<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                   writer.write_all(&self.to_be_bytes()).context(format!("Failed to write {} into buffer.", &self))
                }

                fn size(&self) -> anyhow::Result<$crate::nums::VarInt> {
                    Ok($crate::nums::VarInt::from($size))
                }
            }

            #[async_trait::async_trait]
            impl $crate::encoder::AsyncEncodable for $prim {
                async fn async_encode<W: tokio::io::AsyncWrite + Send + Unpin>(
                    &self,
                    writer: &mut W,
                ) -> anyhow::Result<()> {
                    use tokio::io::AsyncWriteExt;
                    writer
                        .write_all(&self.to_be_bytes())
                        .await
                        .context(format!("Failed to write {} into buffer.", &self))
                }
            }
        )*
    }
}

macro_rules! impl_into_num_bind {
    ($name:ident, $sim:ty, $prim:ty, $relationship:ident) => {
        impl From<$prim> for $name {
            fn from(prim: $prim) -> $name {
                $name(<$sim>::$relationship(prim))
            }
        }
    };
    ($name:ident, $sim:ty, $prim:ty, $relationship:ident | $rel_err:ty) => {
        impl TryFrom<$prim> for $name {
            type Error = $rel_err;
            fn try_from(prim: $prim) -> std::result::Result<$name, Self::Error> {
                Ok($name(<$sim>::$relationship(prim)?))
            }
        }
    };
}

macro_rules! impl_into_prim_bind {
    ($name:ident, $prim:ty, $alt_relationship:ident) => {
        impl From<$name> for $prim {
            fn from(item: $name) -> $prim {
                <$prim>::$alt_relationship(item.0)
            }
        }

        impl From<&$name> for $prim {
            fn from(item: &$name) -> $prim {
                <$prim>::$alt_relationship(item.0)
            }
        }
    };
    ($name:ident, $prim:ty, $alt_relationship:ident | $alt_err:ty) => {
        impl TryFrom<$name> for $prim {
            type Error = $alt_err;
            fn try_from(item: $name) -> std::result::Result<$prim, Self::Error> {
                <$prim>::$alt_relationship(item.0)
            }
        }

        impl TryFrom<&$name> for $prim {
            type Error = $alt_err;
            fn try_from(item: &$name) -> std::result::Result<$prim, Self::Error> {
                <$prim>::$alt_relationship(item.0)
            }
        }
    };
}

macro_rules! impl_ord_bind {
    ($name:ident, $sim:ty, $prim:ty, $relationship:ident) => {
        impl std::cmp::PartialEq<$prim> for $name {
            fn eq(&self, other: &$prim) -> bool {
                Self::eq(self, &<$sim>::$relationship(*other))
            }
        }

        impl std::cmp::PartialOrd<$prim> for $name {
            fn partial_cmp(&self, other: &$prim) -> Option<std::cmp::Ordering> {
                Self::partial_cmp(self, &<$sim>::$relationship(*other))
            }
        }
    };
}

macro_rules! impl_try_operators {
    ($name:ident, $sim:ty, $(($($to_impl:tt)*), $fn_name:ident;)*) => {
        $(
            impl $($to_impl)*<std::result::Result<$sim, std::num::TryFromIntError>> for $name {
                type Output = anyhow::Result<$name>;
                fn $fn_name(self, rhs: std::result::Result<$sim, std::num::TryFromIntError>) -> Self::Output {
                    Ok($name(rhs?.$fn_name(self.0)))
                }
            }
            impl $($to_impl)*<std::result::Result<$sim, std::num::TryFromIntError>> for &$name {
                type Output = anyhow::Result<$name>;
                fn $fn_name(self, rhs: std::result::Result<$sim, std::num::TryFromIntError>) -> Self::Output {
                    Ok($name(rhs?.$fn_name(self.0)))
                }
            }
        )*
    }
}

macro_rules! impl_primitive_operators {
    ($name:ident, $self:ty, $sim:ty, $prim:ty, $relationship:ident, $(($($to_impl:tt)*), $fn_name:ident,)*) => {
        $(
            impl $($to_impl)*<$self> for $prim {
                type Output = $name;
                fn $fn_name(self, rhs: $self) -> Self::Output {
                    <$sim>::$relationship(self).$fn_name(rhs)
                }
            }

            impl $($to_impl)*<$prim> for $self {
                type Output = $name;
                fn $fn_name(self, rhs: $prim) -> Self::Output {
                    self.$fn_name(<$sim>::$relationship(rhs))
                }
            }
        )*
    };
    ($name:ident, $self:ty, $sim:ty, $prim:ty, $relationship:ident, $rel_err:ty, $(($($to_impl:tt)*), $fn_name:ident,)*) => {
        $(
            impl $($to_impl)*<$self> for $prim {
                type Output = std::result::Result<$name, $rel_err>;
                fn $fn_name(self, rhs: $self) -> Self::Output {
                    Ok(<$sim>::$relationship(self)?.$fn_name(rhs))
                }
            }

            impl $($to_impl)*<$prim> for $self {
                type Output = std::result::Result<$name, $rel_err>;
                fn $fn_name(self, rhs: $prim) -> Self::Output {
                    Ok(self.$fn_name(<$sim>::$relationship(rhs)?))
                }
            }
        )*
    }
}

macro_rules! impl_variable_number_bind {
    ($name:ident, $self:ty, $sim:ty, $prim:ty, $relationship:ident $(|$rel_err:ty)?) => {
        impl_primitive_operators!($name, $self, $sim, $prim, $relationship, $($rel_err,)*
            (std::ops::Mul), mul,
            (std::ops::Add), add,
            (std::ops::Sub), sub,
        );
    };
    ($name:ident, $sim:ty, $($prim:ty: ($relationship:ident $(|$rel_err:ty)?, $alt_relationship:ident $(|$alt_err:ty)?),)*) => {
        impl_try_operators!($name, $sim,
            (std::ops::Mul), mul;
            (std::ops::Add), add;
            (std::ops::Sub), sub;
        );
        impl std::cmp::PartialEq<std::result::Result<$sim, std::num::TryFromIntError>> for $name {
            fn eq(&self, other: &std::result::Result<$sim, std::num::TryFromIntError>) -> bool {
                if let Ok(internal) = other {
                    internal == &self.0
                } else {
                    false
                }
            }
        }
        impl std::cmp::PartialOrd<std::result::Result<$sim, std::num::TryFromIntError>> for $name {
            fn partial_cmp(&self, other: &std::result::Result<$sim, std::num::TryFromIntError>) -> Option<std::cmp::Ordering> {
                if let Ok(internal) = other {
                    self.0.partial_cmp(&internal)
                } else {
                    None
                }
            }
        }
        $(
            impl_variable_number_bind!($name, &$name, $sim, $prim, $relationship $(|$rel_err)*);
            impl_variable_number_bind!($name, $name, $sim, $prim, $relationship $(|$rel_err)*);
            impl_ord_bind!($name, $sim, $prim, $relationship);
            impl_into_prim_bind!($name, $prim, $alt_relationship $(|$alt_err)*);
            impl_into_num_bind!($name, $sim, $prim, $relationship $(|$rel_err)*);
        )*
    }
}

macro_rules! declare_variable_number {
    ($name:ident, $primitive_signed:ty, $bit_limit:literal, $primitive_unsigned:ty, $and_check:literal $(,
        $prim:ty: ($relationship:ident $(|$rel_err:ty)?, $alt_relationship:ident $(|$alt_err:ty)?)
    )*) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
        pub struct $name($primitive_signed);

        impl_variable_number_bind!($name, $primitive_signed, $(
            $prim: ($relationship $(|$rel_err)*, $alt_relationship $(|$alt_err)*),
        )*);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl $name {
            pub fn decode_and_size(reader: &mut impl std::io::Read) -> anyhow::Result<(VarInt, Self)> {
                let mut running_size = 0;
                let mut value: $primitive_signed = 0;
                let mut bit_offset = 0u32;
                loop {
                    if bit_offset == $bit_limit {
                        anyhow::bail!(
                            "Failed to decode {}, too many bytes.",
                            stringify!($crate::VarInt)
                        );
                    }

                    let mut buf = [0; 1];
                    reader.read_exact(&mut buf)?;
                    running_size += 1;
                    let byte = buf[0];
                    value |= <$primitive_signed>::from(byte & 0b01111111)
                        .overflowing_shl(bit_offset)
                        .0;
                    bit_offset += 7;

                    if byte & 0b10000000 == 0 {
                        break;
                    }
                }
                Ok((VarInt::try_from(running_size)?, $name(value)))
            }
        }

        impl $crate::encoder::Decodable for $name {
            fn decode<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                let mut value: $primitive_signed = 0;
                let mut bit_offset = 0u32;
                loop {
                    if bit_offset == $bit_limit {
                        anyhow::bail!(
                            "Failed to decode {}, too many bytes.",
                            stringify!($crate::VarInt)
                        );
                    }

                    let mut buf = [0; 1];
                    reader.read_exact(&mut buf)?;
                    let byte = buf[0];
                    value |= <$primitive_signed>::from(byte & 0b01111111)
                        .overflowing_shl(bit_offset)
                        .0;
                    bit_offset += 7;

                    if byte & 0b10000000 == 0 {
                        break;
                    }
                }
                Ok($name(value))
            }
        }

        impl $crate::encoder::Encodable for $name {
            fn encode<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                let mut temp = self.0.clone() as $primitive_unsigned;
                loop {
                    if temp & $and_check == 0 {
                        writer.write_all(&[temp as u8])?;
                        return Ok(());
                    }
                    writer.write_all(&[(temp & 0x7F | 0x80) as u8])?;
                    temp = temp.overflowing_shr(7).0;
                }
            }

            fn size(&self) -> anyhow::Result<$crate::nums::VarInt> {
                let mut running_size: i32 = 0;
                let mut temp = self.0.clone() as $primitive_unsigned;
                loop {
                    if temp & $and_check == 0 {
                        running_size += 1;
                        return Ok(running_size.into());
                    }
                    running_size += 1;
                    temp = temp.overflowing_shr(7).0;
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::encoder::AsyncEncodable for $name {
            async fn async_encode<W: tokio::io::AsyncWrite + Send + Unpin>(
                &self,
                writer: &mut W,
            ) -> anyhow::Result<()> {
                use tokio::io::AsyncWriteExt;

                let mut temp = self.0.clone() as $primitive_unsigned;
                loop {
                    if temp & $and_check == 0 {
                        writer.write_u8(temp as u8).await?;
                        return Ok(());
                    }
                    writer.write_u8((temp & 0x7F | 0x80) as u8).await?;
                    temp = temp.overflowing_shr(7).0;
                }
            }
        }

        // extensions

        impl std::ops::Mul for &$name {
            type Output = $name;

            fn mul(self, rhs: Self) -> Self::Output {
                $name(self.0 * rhs.0)
            }
        }

        impl std::ops::Mul for $name {
            type Output = $name;

            fn mul(self, rhs: Self) -> Self::Output {
                $name(self.0 * rhs.0)
            }
        }

        impl std::ops::Add for &$name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                $name(self.0 + rhs.0)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $name(self.0 + rhs.0)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $primitive_signed;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // prim sign functions
        impl std::ops::Mul<&$name> for $primitive_signed {
            type Output = $name;
            fn mul(self, rhs: &$name) -> Self::Output {
                $name(self * rhs.0)
            }
        }
        impl std::ops::Mul<$primitive_signed> for &$name {
            type Output = $name;
            fn mul(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 * rhs)
            }
        }
        impl std::ops::Add<&$name> for $primitive_signed {
            type Output = $name;
            fn add(self, rhs: &$name) -> Self::Output {
                $name(self + rhs.0)
            }
        }
        impl std::ops::Add<$primitive_signed> for &$name {
            type Output = $name;
            fn add(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 + rhs)
            }
        }
        impl std::ops::Sub<&$name> for $primitive_signed {
            type Output = $name;
            fn sub(self, rhs: &$name) -> Self::Output {
                $name(self - rhs.0)
            }
        }
        impl std::ops::Sub<$primitive_signed> for &$name {
            type Output = $name;
            fn sub(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 - rhs)
            }
        }
        impl std::ops::Mul<$name> for $primitive_signed {
            type Output = $name;
            fn mul(self, rhs: $name) -> Self::Output {
                $name(self * rhs.0)
            }
        }
        impl std::ops::Mul<$primitive_signed> for $name {
            type Output = $name;
            fn mul(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 * rhs)
            }
        }
        impl std::ops::Add<$name> for $primitive_signed {
            type Output = $name;
            fn add(self, rhs: $name) -> Self::Output {
                $name(self + rhs.0)
            }
        }
        impl std::ops::Add<$primitive_signed> for $name {
            type Output = $name;
            fn add(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 + rhs)
            }
        }
        impl std::ops::Sub<$name> for $primitive_signed {
            type Output = $name;
            fn sub(self, rhs: $name) -> Self::Output {
                $name(self - rhs.0)
            }
        }
        impl std::ops::Sub<$primitive_signed> for $name {
            type Output = $name;
            fn sub(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 - rhs)
            }
        }
        impl std::cmp::PartialEq<$primitive_signed> for $name {
            fn eq(&self, other: &$primitive_signed) -> bool {
                self.0 == *other
            }
        }
        impl std::cmp::PartialOrd<$primitive_signed> for $name {
            fn partial_cmp(&self, other: &$primitive_signed) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }
        impl From<$primitive_signed> for $name {
            fn from(prim: $primitive_signed) -> Self {
                Self(prim)
            }
        }
        impl From<$name> for $primitive_signed {
            fn from(t: $name) -> $primitive_signed {
                t.0
            }
        }

        impl From<&$name> for $primitive_signed {
            fn from(t: &$name) -> $primitive_signed {
                t.0
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 = self.0 + rhs.0;
            }
        }
    };
}

impl crate::encoder::Decodable for bool {
    fn decode<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
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

impl crate::encoder::Encodable for bool {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        writer
            .write_all(&[*self as u8])
            .context(format!("Failed to write {} into buffer.", &self))
    }

    fn size(&self) -> anyhow::Result<VarInt> {
        Ok(VarInt::from(1))
    }
}

#[async_trait::async_trait]
impl crate::encoder::AsyncEncodable for bool {
    async fn async_encode<W: tokio::io::AsyncWrite + Send + Unpin>(
        &self,
        writer: &mut W,
    ) -> anyhow::Result<()> {
        use tokio::io::AsyncWriteExt;
        writer
            .write_u8(*self as u8)
            .await
            .context(format!("Failed to write {} into buffer.", &self))
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

declare_variable_number!(VarInt, i32, 35, u32, 0xFFFFFF80,
    u8: (from, try_from | std::num::TryFromIntError),
    i8: (from, try_from | std::num::TryFromIntError),
    u16: (from, try_from | std::num::TryFromIntError),
    i16: (from, try_from | std::num::TryFromIntError),
    u32: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError),
    u64: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError),
    i64: (try_from | std::num::TryFromIntError, from),
    u128: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError),
    i128: (try_from | std::num::TryFromIntError, from),
    usize: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError),
    isize: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError)
);

declare_variable_number!(VarLong, i64, 70, u64, 0xFFFFFFFFFFFFFF80,
    u8: (from, try_from | std::num::TryFromIntError),
    i8: (from, try_from | std::num::TryFromIntError),
    u16: (from, try_from | std::num::TryFromIntError),
    i16: (from, try_from | std::num::TryFromIntError),
    u32: (from, try_from | std::num::TryFromIntError),
    i32: (from, try_from | std::num::TryFromIntError),
    u64: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError),
    u128: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError),
    i128: (try_from | std::num::TryFromIntError, from),
    usize: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError),
    isize: (try_from | std::num::TryFromIntError, try_from | std::num::TryFromIntError)
);
