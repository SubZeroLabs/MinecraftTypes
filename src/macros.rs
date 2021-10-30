macro_rules! declare_primitives {
    ($(|$prim:ty;$size:literal|)+) => {
        $(
            impl Decodable for $prim {
                fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
                    let mut into = [0u8; $size];
                    anyhow::Context::context(reader.read_exact(&mut into), format!("Unexpected EOF while reading {} from buffer.", stringify!($prim)))?;
                    Ok(<$prim>::from_be_bytes(into))
                }
            }

            impl Encodable for $prim {
                fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
                    anyhow::Context::context(writer.write_all(&self.to_be_bytes()), format!("Failed to write {} into buffer.", &self))
                }

                fn size(&self) -> anyhow::Result<VarInt> {
                    Ok($crate::VarInt::from($size))
                }
            }
        )*
    }
}

macro_rules! declare_variable_number {
    ($name:ident, $primitive_signed:ty, $bit_limit:literal, $primitive_unsigned:ty, $and_check:literal) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
        pub struct $name($primitive_signed);

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $name(self.0 + rhs.0)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Mul<$primitive_signed> for &$name {
            type Output = $name;

            fn mul(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 * rhs)
            }
        }

        impl std::ops::Mul<$primitive_unsigned> for &$name {
            type Output = $name;

            fn mul(self, rhs: $primitive_unsigned) -> Self::Output {
                $name(((self.0 as $primitive_unsigned) * rhs) as $primitive_signed)
            }
        }

        impl std::ops::Mul for &$name {
            type Output = $name;

            fn mul(self, rhs: Self) -> Self::Output {
                $name(self.0 * rhs.0)
            }
        }

        impl std::ops::Mul<$primitive_signed> for $name {
            type Output = $name;

            fn mul(self, rhs: $primitive_signed) -> Self::Output {
                $name(self.0 * rhs)
            }
        }

        impl std::ops::Mul<$primitive_unsigned> for $name {
            type Output = $name;

            fn mul(self, rhs: $primitive_unsigned) -> Self::Output {
                $name(((self.0 as $primitive_unsigned) * rhs) as $primitive_signed)
            }
        }

        impl std::ops::Mul for $name {
            type Output = $name;

            fn mul(self, rhs: Self) -> Self::Output {
                $name(self.0 * rhs.0)
            }
        }

        impl std::cmp::PartialEq<usize> for $name {
            fn eq(&self, other: &usize) -> bool {
                (self.0 as usize) == *other
            }
        }

        impl std::cmp::PartialOrd<usize> for $name {
            fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
                Some((self.0 as usize).cmp(other))
            }
        }

        impl std::cmp::PartialEq<$primitive_signed> for $name {
            fn eq(&self, other: &$primitive_signed) -> bool {
                self.0 == *other
            }
        }

        impl std::cmp::PartialOrd<$primitive_signed> for $name {
            fn partial_cmp(&self, other: &$primitive_signed) -> Option<std::cmp::Ordering> {
                Some(self.0.cmp(other))
            }
        }

        impl $name {
            pub fn decode_and_size(reader: &mut impl Read) -> anyhow::Result<(VarInt, Self)> {
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
                Ok((VarInt::from(running_size), $name(value)))
            }
        }

        impl Decodable for $name {
            fn decode(reader: &mut impl Read) -> anyhow::Result<Self> {
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

        impl Encodable for $name {
            fn encode(&self, writer: &mut impl Write) -> anyhow::Result<()> {
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

            fn size(&self) -> anyhow::Result<VarInt> {
                let mut running_size = 0;
                let mut temp = self.0.clone() as $primitive_unsigned;
                loop {
                    if temp & $and_check == 0 {
                        running_size += 1;
                        return Ok($crate::VarInt::from(running_size));
                    }
                    running_size += 1;
                    temp = temp.overflowing_shr(7).0;
                }
            }
        }

        impl From<$primitive_unsigned> for $name {
            fn from(internal: $primitive_unsigned) -> Self {
                $name(internal as $primitive_signed)
            }
        }

        impl From<$name> for $primitive_unsigned {
            fn from(internal: $name) -> Self {
                internal.0 as $primitive_unsigned
            }
        }

        impl From<$primitive_signed> for $name {
            fn from(internal: $primitive_signed) -> Self {
                $name(internal)
            }
        }

        impl From<$name> for $primitive_signed {
            fn from(internal: $name) -> Self {
                internal.0
            }
        }

        impl From<usize> for $name {
            fn from(internal: usize) -> Self {
                $name(internal as $primitive_signed)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $primitive_signed;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

macro_rules! auto_string {
    ($name:ident, $size:literal) => {
        #[derive(Debug)]
        pub struct $name(String);

        impl $crate::McString for $name {
            fn new(internal: String) -> Self {
                $name(internal)
            }
            fn string(&self) -> &String {
                &self.0
            }
            fn limit() -> $crate::VarInt {
                $crate::VarInt($size)
            }
        }

        impl From<String> for $name {
            fn from(internal: String) -> Self {
                $name(internal)
            }
        }

        impl From<&str> for $name {
            fn from(internal: &str) -> Self {
                $name(String::from(internal))
            }
        }
    };
}

#[macro_export]
macro_rules! simple_auto_enum {
    ($($enum_name:ident; $index_type:ty { $($byte_representation:literal => $option_name:ident,)* })*) => {
        $(
            #[derive(Debug)]
            pub enum $enum_name {
                $(
                    $option_name,
                )*
            }

            impl $crate::IndexDecodable for $enum_name {
                fn decode_index(__reader: &mut impl std::io::Read, index: &$crate::VarInt) -> anyhow::Result<Self> {
                    match (*index).into() {
                        $(
                            $byte_representation => Ok($enum_name::$option_name),
                        )*
                        _ => anyhow::bail!("Failed to decode enum, unknown index {}.", index),
                    }
                }
            }

            impl $crate::Decodable for $enum_name {
                fn decode(reader: &mut impl std::io::Read) -> anyhow::Result<Self> {
                    use $crate::IndexDecodable;

                    let index = <$index_type>::decode(reader)?;
                    <$enum_name>::decode_index(reader, &index)
                }
            }

            impl $crate::Encodable for $enum_name {
                fn encode(&self, writer: &mut impl std::io::Write) -> anyhow::Result<()> {
                    match self {
                        $(
                            $enum_name::$option_name => {
                                <$index_type>::encode(&<$index_type>::from($byte_representation), writer)?;
                                Ok(())
                            }
                        )*
                    }
                }
                fn size(&self) -> anyhow::Result<VarInt> {
                    match self {
                        $(
                            $enum_name::$option_name => {
                                <$index_type>::size(&<$index_type>::from($byte_representation))
                            }
                        )*
                    }
                }
            }
        )*
    };
}

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

            impl $crate::IndexDecodable for $enum_name {
                fn decode_index(__reader: &mut impl std::io::Read, index: &$crate::VarInt) -> anyhow::Result<Self> {
                    match (*index).into() {
                        $(
                            $byte_representation => Ok($enum_name::$option_name$(({
                                use $crate::Decodable;
                                <$option_type>::decode(__reader)?
                            }))*),
                        )*
                        _ => anyhow::bail!("Failed to decode enum, unknown index {}.", index),
                    }
                }
            }

            impl $crate::Decodable for $enum_name {
                fn decode(reader: &mut impl std::io::Read) -> anyhow::Result<Self> {
                    use $crate::IndexDecodable;
                    let index = <$index_type>::decode(reader)?;
                    <$enum_name>::decode_index(reader, &index)
                }
            }

            impl $crate::Encodable for $enum_name {
                fn encode(&self, writer: &mut impl std::io::Write) -> anyhow::Result<()> {
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

                fn size(&self) -> anyhow::Result<$crate::VarInt> {
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
            impl $crate::Decodable for $struct_name {
                fn decode(reader: &mut impl std::io::Read) -> anyhow::Result<Self> {
                    $(let $field_name = struct_decode_if_def!(reader, $field_name, $field_type $(,$predicate, $alternate)*);)*
                    Ok($struct_name {
                        $($field_name,)*
                    })
                }
            }

            #[allow(unused_variables)]
            impl $crate::Encodable for $struct_name {
                fn encode(&self, writer: &mut impl std::io::Write) -> anyhow::Result<()> {
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
        )*
    };
}
