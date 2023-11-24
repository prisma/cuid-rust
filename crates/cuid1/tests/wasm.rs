use cuid::cuid;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn cuid_length() {
    let id = cuid().unwrap();
    assert!(id.len() == 25);
}
