use minecraft_data_types::{Decodable, Encodable, VarInt};
use std::io::Cursor;

type MinecraftCursor = Cursor<Vec<u8>>;

#[test]
fn test_byte_prim() -> anyhow::Result<()> {
    let mut cursor: MinecraftCursor = Cursor::new(vec![2]);

    let result = u8::decode(&mut cursor)?;
    assert_eq!(result, 2);
    Ok(())
}

#[test]
fn test_byte_prim_2() -> anyhow::Result<()> {
    let mut cursor: MinecraftCursor = Cursor::new(vec![]);
    let x = 5u8;
    x.encode(&mut cursor)?;
    assert_eq!(vec![5], cursor.into_inner());
    Ok(())
}

#[test]
fn test_byte_prim_3() -> anyhow::Result<()> {
    let mut cursor: MinecraftCursor = Cursor::new(vec![0b1111_1110]);

    let result = i8::decode(&mut cursor)?;
    assert_eq!(result, -2);
    Ok(())
}

#[test]
fn test_byte_prim_4() -> anyhow::Result<()> {
    let mut cursor: MinecraftCursor = Cursor::new(vec![]);
    let x = -2i8;
    x.encode(&mut cursor)?;
    assert_eq!(vec![0b1111_1110], cursor.into_inner());
    Ok(())
}

#[test]
fn mul_var_test() {
    let x = VarInt::from(10);
    assert_eq!(VarInt::from(10), x);
    assert_eq!(VarInt::from(20), x * 2);
    assert_eq!(VarInt::from(30), &x * 3);
}

#[test]
fn mc_varint_encode_examples() -> anyhow::Result<()> {
    let vec_pairs = vec![(1, vec![0x1u8])];

    for pair in vec_pairs {
        // test read
        let mut cursor: MinecraftCursor = Cursor::new(pair.1.clone());
        let result = VarInt::decode(&mut cursor)?;
        assert_eq!(pair.0, *result);

        // test write
        let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
        result.encode(&mut cursor)?;
        assert_eq!(pair.1, cursor.into_inner())
    }

    // let mut cursor: MinecraftCursor = Cursor::new(vec![0x00]);
    // let result = VarInt::decode(&mut cursor);
    // assert_eq!(0, *result);
    // VarInt::encode(result, cursor);
    //
    //
    //
    // let res1 = VarInt::decode(&mut Cursor::new(vec![0x00]))?;
    // assert_eq!(0, *res1);
    // let res2 = VarInt::decode(&mut Cursor::new(vec![0x01]))?;
    // assert_eq!(1, *res2);
    // let res3 = VarInt::decode(&mut Cursor::new(vec![0x02]))?;
    // assert_eq!(2, *res3);
    // let res4 = VarInt::decode(&mut Cursor::new(vec![0x7f]))?;
    // assert_eq!(127, *res4);
    // let res5 = VarInt::decode(&mut Cursor::new(vec![0x80, 0x01]))?;
    // assert_eq!(128, *res5);
    // let res6 = VarInt::decode(&mut Cursor::new(vec![0xff, 0x01]))?;
    // assert_eq!(255, *res6);
    // let res7 = VarInt::decode(&mut Cursor::new(vec![0xdd, 0xc7, 0x01]))?;
    // assert_eq!(25565, *res7);
    // let res8 = VarInt::decode(&mut Cursor::new(vec![0xff, 0xff, 0x7f]))?;
    // assert_eq!(2097151, *res8);
    // let res9 = VarInt::decode(&mut Cursor::new(vec![0xff, 0xff, 0xff, 0xff, 0x07]))?;
    // assert_eq!(2147483647, *res9);
    // let res10 = VarInt::decode(&mut Cursor::new(vec![0xff, 0xff, 0xff, 0xff, 0x0f]))?;
    // assert_eq!(-1, *res10);
    // let res11 = VarInt::decode(&mut Cursor::new(vec![0x80, 0x80, 0x80, 0x80, 0x08]))?;
    // assert_eq!(-2147483648, *res11);
    //
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0x00], res1.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0x01], res2.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0x02], res3.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0x7f], res4.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0x80, 0x01], res5.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0xff, 0x01], res6.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0xdd, 0xc7, 0x01], res7.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0xff, 0xff, 0x7f], res8.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0xff, 0xff, 0xff, 0xff, 0x07], res9.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // assert_eq!(vec![0xff, 0xff, 0xff, 0xff, 0x0f], res10.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    // res11.encode(&mut cursor);
    // assert_eq!(vec![0x80, 0x80, 0x80, 0x80, 0x08], res11.encode()?);
    // let mut cursor: MinecraftCursor = Cursor::new(Vec::new());
    Ok(())
}

// #[test]
// fn mc_varlong_encode_examples() -> Result<()> {
//     let (res1, _) = VarLong::decode(vec![0x00])?;
//     assert_eq!(0, *res1);
//     let (res2, _) = VarLong::decode(vec![0x01])?;
//     assert_eq!(1, *res2);
//     let (res3, _) = VarLong::decode(vec![0x02])?;
//     assert_eq!(2, *res3);
//     let (res4, _) = VarLong::decode(vec![0x7f])?;
//     assert_eq!(127, *res4);
//     let (res5, _) = VarLong::decode(vec![0x80, 0x01])?;
//     assert_eq!(128, *res5);
//     let (res6, _) = VarLong::decode(vec![0xff, 0x01])?;
//     assert_eq!(255, *res6);
//     let (res7, _) = VarLong::decode(vec![0xff, 0xff, 0xff, 0xff, 0x07])?;
//     assert_eq!(2147483647, *res7);
//     let (res8, _) = VarLong::decode(vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f])?;
//     assert_eq!(9223372036854775807, *res8);
//     let (res9, _) = VarLong::decode(vec![
//         0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
//     ])?;
//     assert_eq!(-1, *res9);
//     let (res10, _) = VarLong::decode(vec![
//         0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01,
//     ])?;
//     assert_eq!(-2147483648, *res10);
//     let (res11, _) = VarLong::decode(vec![
//         0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01,
//     ])?;
//     assert_eq!(-9223372036854775808, *res11);
//
//     assert_eq!(vec![0x00], res1.encode()?);
//     assert_eq!(vec![0x01], res2.encode()?);
//     assert_eq!(vec![0x02], res3.encode()?);
//     assert_eq!(vec![0x7f], res4.encode()?);
//     assert_eq!(vec![0x80, 0x01], res5.encode()?);
//     assert_eq!(vec![0xff, 0x01], res6.encode()?);
//     assert_eq!(vec![0xff, 0xff, 0xff, 0xff, 0x07], res7.encode()?);
//     assert_eq!(
//         vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
//         res8.encode()?
//     );
//     assert_eq!(
//         vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
//         res9.encode()?
//     );
//     assert_eq!(
//         vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
//         res10.encode()?
//     );
//     assert_eq!(
//         vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
//         res11.encode()?
//     );
//     Ok(())
// }

// use minecraft_data_types::*;
// use uuid::Uuid;
//
// #[test]
// fn mc_bool_decode_takes_1_byte() -> Result<()> {
//     let (mc_bool, remaining) = primitive::McBoolean::decode(vec![0x00u8, 0x01u8])?;
//     assert_eq!(false, mc_bool.into());
//     assert_eq!(vec![0x01u8], remaining);
//     Ok(())
// }
//
// #[test]
// fn mc_bool_encodes_true_to_1() -> Result<()> {
//     assert_eq!(vec![0x01u8], primitive::McBoolean::new(true).encode()?);
//     Ok(())
// }
//
// #[test]
// fn mc_bool_encodes_false_to_0() -> Result<()> {
//     assert_eq!(vec![0x00u8], primitive::McBoolean::new(false).encode()?);
//     Ok(())
// }
//
// #[test]
// fn mc_bool_decode_empty_arr_fails() {
//     if let Ok((_, _)) = primitive::McBoolean::decode(vec![]) {
//         panic!("Invalid array proceeded as valid.")
//     }
// }
//
// #[test]
// fn mc_byte_should_translate_byte() -> Result<()> {
//     let vec = vec![0x1u8];
//     let (parse, _) = primitive::McByte::decode(vec)?;
//     assert_eq!(0x1i8, *parse);
//     let encoded = parse.encode()?;
//     assert_eq!(vec![0x1u8], encoded);
//     Ok(())
// }
//
// #[test]
// fn mc_varint_encode_examples() -> Result<()> {
//     let (res1, _) = VarInt::decode(vec![0x00])?;
//     assert_eq!(0, *res1);
//     let (res2, _) = VarInt::decode(vec![0x01])?;
//     assert_eq!(1, *res2);
//     let (res3, _) = VarInt::decode(vec![0x02])?;
//     assert_eq!(2, *res3);
//     let (res4, _) = VarInt::decode(vec![0x7f])?;
//     assert_eq!(127, *res4);
//     let (res5, _) = VarInt::decode(vec![0x80, 0x01])?;
//     assert_eq!(128, *res5);
//     let (res6, _) = VarInt::decode(vec![0xff, 0x01])?;
//     assert_eq!(255, *res6);
//     let (res7, _) = VarInt::decode(vec![0xdd, 0xc7, 0x01])?;
//     assert_eq!(25565, *res7);
//     let (res8, _) = VarInt::decode(vec![0xff, 0xff, 0x7f])?;
//     assert_eq!(2097151, *res8);
//     let (res9, _) = VarInt::decode(vec![0xff, 0xff, 0xff, 0xff, 0x07])?;
//     assert_eq!(2147483647, *res9);
//     let (res10, _) = VarInt::decode(vec![0xff, 0xff, 0xff, 0xff, 0x0f])?;
//     assert_eq!(-1, *res10);
//     let (res11, _) = VarInt::decode(vec![0x80, 0x80, 0x80, 0x80, 0x08])?;
//     assert_eq!(-2147483648, *res11);
//
//     assert_eq!(vec![0x00], res1.encode()?);
//     assert_eq!(vec![0x01], res2.encode()?);
//     assert_eq!(vec![0x02], res3.encode()?);
//     assert_eq!(vec![0x7f], res4.encode()?);
//     assert_eq!(vec![0x80, 0x01], res5.encode()?);
//     assert_eq!(vec![0xff, 0x01], res6.encode()?);
//     assert_eq!(vec![0xdd, 0xc7, 0x01], res7.encode()?);
//     assert_eq!(vec![0xff, 0xff, 0x7f], res8.encode()?);
//     assert_eq!(vec![0xff, 0xff, 0xff, 0xff, 0x07], res9.encode()?);
//     assert_eq!(vec![0xff, 0xff, 0xff, 0xff, 0x0f], res10.encode()?);
//     assert_eq!(vec![0x80, 0x80, 0x80, 0x80, 0x08], res11.encode()?);
//     Ok(())
// }
//
// #[test]
// fn mc_varlong_encode_examples() -> Result<()> {
//     let (res1, _) = VarLong::decode(vec![0x00])?;
//     assert_eq!(0, *res1);
//     let (res2, _) = VarLong::decode(vec![0x01])?;
//     assert_eq!(1, *res2);
//     let (res3, _) = VarLong::decode(vec![0x02])?;
//     assert_eq!(2, *res3);
//     let (res4, _) = VarLong::decode(vec![0x7f])?;
//     assert_eq!(127, *res4);
//     let (res5, _) = VarLong::decode(vec![0x80, 0x01])?;
//     assert_eq!(128, *res5);
//     let (res6, _) = VarLong::decode(vec![0xff, 0x01])?;
//     assert_eq!(255, *res6);
//     let (res7, _) = VarLong::decode(vec![0xff, 0xff, 0xff, 0xff, 0x07])?;
//     assert_eq!(2147483647, *res7);
//     let (res8, _) = VarLong::decode(vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f])?;
//     assert_eq!(9223372036854775807, *res8);
//     let (res9, _) = VarLong::decode(vec![
//         0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
//     ])?;
//     assert_eq!(-1, *res9);
//     let (res10, _) = VarLong::decode(vec![
//         0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01,
//     ])?;
//     assert_eq!(-2147483648, *res10);
//     let (res11, _) = VarLong::decode(vec![
//         0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01,
//     ])?;
//     assert_eq!(-9223372036854775808, *res11);
//
//     assert_eq!(vec![0x00], res1.encode()?);
//     assert_eq!(vec![0x01], res2.encode()?);
//     assert_eq!(vec![0x02], res3.encode()?);
//     assert_eq!(vec![0x7f], res4.encode()?);
//     assert_eq!(vec![0x80, 0x01], res5.encode()?);
//     assert_eq!(vec![0xff, 0x01], res6.encode()?);
//     assert_eq!(vec![0xff, 0xff, 0xff, 0xff, 0x07], res7.encode()?);
//     assert_eq!(
//         vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
//         res8.encode()?
//     );
//     assert_eq!(
//         vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
//         res9.encode()?
//     );
//     assert_eq!(
//         vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
//         res10.encode()?
//     );
//     assert_eq!(
//         vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
//         res11.encode()?
//     );
//     Ok(())
// }
//
// #[test]
// fn string_encode_and_decode() -> Result<()> {
//     let mc_string = McString::from("Some String");
//     let encoded = mc_string.encode()?;
//     let (decoded, _) = McString::decode(encoded, 32767)?;
//
//     assert_eq!("Some String", *decoded);
//
//     Ok(())
// }
//
// #[test]
// fn string_type_forwards() -> Result<()> {
//     let chat_json = ChatJson::from("Some String");
//     let encoded = chat_json.encode()?;
//     let (decoded, _) = ChatJson::decode(encoded)?;
//
//     assert_eq!("Some String", *decoded);
//
//     let identifier = Identifier::from("Some String");
//     let encoded = identifier.encode()?;
//     let (decoded, _) = Identifier::decode(encoded)?;
//
//     assert_eq!("Some String", *decoded);
//
//     Ok(())
// }
//
// #[test]
// fn uuid_type_forwards() -> Result<()> {
//     let uuid = Uuid::new_v4();
//     let mc_uuid = McUuid::new(uuid.clone());
//     let encoded = mc_uuid.encode()?;
//     let (decoded, _) = McUuid::decode(encoded)?;
//
//     assert_eq!(uuid, *decoded);
//     Ok(())
// }
