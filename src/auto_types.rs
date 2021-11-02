#[macro_export]
macro_rules! auto_enum {
    ($($enum_name:ident; $index_type:ty { $($byte_representation:literal => $option_name:ident $(:$option_type:ty)?,)* })*) => {
        auto_enum!($($enum_name; $index_type { $($byte_representation => $option_name $(:$option_type, pseudo)*,)* })*);
    };
    ($($enum_name:ident; $index_type:ty { $($byte_representation:literal => $option_name:ident $(:$option_type:ty, $pseudo:ident)?,)* })*) => {
        $(
            #[derive(Debug)]
            pub enum $enum_name {
                $(
                    $option_name$(($option_type))*,
                )*
            }

            impl $crate::encoder::Decodable for $enum_name {
                fn decode<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                    use $crate::IndexDecodable;
                    let index = <$index_type>::decode(reader)?;
                    <$enum_name>::decode_index(reader, &index)
                }
            }

            impl $crate::encoder::Encodable for $enum_name {
                fn encode<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                    match self {
                        $(
                            $enum_name::$option_name$(($pseudo))* => {
                                <$index_type>::encode(&<$index_type>::from($byte_representation), writer)?;
                                $(
                                    anyhow::Context::context(
                                        <$option_type>::encode($pseudo, writer),
                                        format!("Failed to encode enum type {}::{}.", stringify!($enum_name), stringify!($option_name))
                                    )?;
                                )*
                                Ok(())
                            }
                        )*
                    }
                }

                fn size(&self) -> anyhow::Result<$crate::nums::VarInt> {
                    match self {
                        $(
                            $enum_name::$option_name$(($pseudo))* => {
                                let size = <$index_type>::size(&<$index_type>::from($byte_representation))?;
                                $(
                                    let size = size + <$option_type>::size($pseudo)?;
                                )*
                                Ok(size)
                            }
                        )*
                    }
                }
            }

            #[async_trait::async_trait]
            impl $crate::encoder::AsyncEncodable for $enum_name {
                async fn async_encode<W: tokio::io::AsyncWrite + Send + Unpin>(
                    &self,
                    writer: &mut W,
                ) -> anyhow::Result<()> {
                    match self {
                        $(
                            $enum_name::$option_name$(($pseudo))* => {
                                <$index_type>::async_encode(&<$index_type>::from($byte_representation), writer).await?;
                                $(
                                    anyhow::Context::context(
                                        <$option_type>::async_encode($pseudo, writer).await,
                                        format!("Failed to encode enum type {}::{}.", stringify!($enum_name), stringify!($option_name))
                                    )?;
                                )*
                                Ok(())
                            }
                        )*
                    }
                }
            }
        )*
    };
}

macro_rules! field_context {
    ($field_name:ident, $field_type:ty, $context_type:literal) => {
        format!(
            "Failed to {} type {} for field {}.",
            $context_type,
            stringify!($field_type),
            stringify!($field_name)
        )
    };
}

macro_rules! struct_decode_if_def {
    ($reader:expr, $field_name:ident, $field_type:ty) => {
        anyhow::Context::context(
            <$field_type>::decode($reader),
            field_context!($field_name, $field_type, "decode"),
        )?
    };
    ($reader:expr, $field_name:ident, $field_type:ty, $predicate:expr, $alternate:expr) => {
        if $predicate {
            anyhow::Context::context(
                <$field_type>::decode($reader),
                field_context!($field_name, $field_type, "decode"),
            )?
        } else {
            $alternate
        }
    };
}

#[macro_export]
macro_rules! auto_struct {
    ($($struct_name:ident { $($field_name:ident: $field_type:ty $(|$predicate:expr => $alternate:expr)?,)* })*) => {
        $(
            #[derive(Debug)]
            pub struct $struct_name {
                $(pub $field_name: $field_type,)*
            }

            #[allow(unused_variables)]
            impl $crate::encoder::Decodable for $struct_name {
                fn decode<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                    $(let $field_name = struct_decode_if_def!(reader, $field_name, $field_type $(,$predicate, $alternate)*);)*
                    Ok($struct_name {
                        $($field_name,)*
                    })
                }
            }

            #[allow(unused_variables)]
            impl $crate::encoder::Encodable for $struct_name {
                fn encode<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                    $(anyhow::Context::context(self.$field_name.encode(writer), field_context!($field_name, $field_type, "encode"))?;)*
                    Ok(())
                }

                fn size(&self) -> anyhow::Result<$crate::VarInt> {
                    let size = $crate::VarInt::from(0);
                    $(
                        let size = size + <$field_type>::size(&self.$field_name)?;
                    )*
                    Ok(size)
                }
            }

            #[async_trait::async_trait]
            #[allow(unused_variables)]
            impl $crate::AsyncEncodable for $struct_name {
                async fn async_encode(
                    &self,
                    writer: &mut tokio::net::tcp::OwnedWriteHalf,
                ) -> anyhow::Result<()> {
                    $(anyhow::Context::context(self.$field_name.async_encode(writer).await, field_context!($field_name, $field_type, "encode"))?;)*
                    Ok(())
                }
            }
        )*
    };
}
