mod common;

use common::{
    assert_bijective_serialization,
    TestHeader
};

#[test]
fn it_serializes_bijectively() {
    let raw = [0b01000110, 0b11001101, 0b11000000];
    assert_bijective_serialization::<TestHeader>(&raw);
}