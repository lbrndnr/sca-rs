mod common;

use common::TestHeader;

#[test]
fn it_parses_bijectively() {
    let raw = [0b01000110, 0b11001101, 0b11000000];
    let hdr = TestHeader::try_from(raw.as_slice()).unwrap();
    let hdr_raw: Vec<u8> = hdr.into();

    assert_eq!(hdr_raw.as_slice(), raw);
}