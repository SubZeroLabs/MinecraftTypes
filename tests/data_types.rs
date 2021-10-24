use minecraft_data_types::*;

#[test]
fn mc_bool_decode_takes_1_byte() -> Result<()> {
    let (mc_bool, remaining) = primitive::McBoolean::decode(vec![0x00u8, 0x01u8])?;
    assert_eq!(false, mc_bool.into());
    assert_eq!(vec![0x01u8], remaining);
    Ok(())
}

#[test]
fn mc_bool_encodes_true_to_1() -> Result<()> {
    assert_eq!(vec![0x01u8], primitive::McBoolean::new(true).encode()?);
    Ok(())
}

#[test]
fn mc_bool_encodes_false_to_0() -> Result<()> {
    assert_eq!(vec![0x00u8], primitive::McBoolean::new(false).encode()?);
    Ok(())
}

#[test]
fn mc_bool_decode_empty_arr_fails() {
    if let Ok((_, _)) = primitive::McBoolean::decode(vec![]) {
        panic!("Invalid array proceeded as valid.")
    }
}
