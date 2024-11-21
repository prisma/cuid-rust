use wasm_bindgen_test::*;
use wasm_rs_dbg::dbg;

#[wasm_bindgen_test]
fn cuid1_length() {
    // e.g.: "cm3rpdbhn0001w10gi9au8cul"
    let id = cuid::cuid1();
    dbg!("cuid1: {}", &id);

    assert!(id.len() == 25);
}

#[wasm_bindgen_test]
fn cuid2_length() {
    // e.g.: "u8yjmh700jcegr5oi3onij1u"
    let id = cuid::cuid2();
    dbg!("cuid2: {}", &id);

    assert!(id.len() == 24);
}
