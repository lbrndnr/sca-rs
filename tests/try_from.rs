mod common;

use common::TestHeader;

#[test]
fn it_implemenets_try_from() {
    let raw = [0b01000110, 0b11001101, 0b11000000];
    let hdr = TestHeader::try_from(raw.as_slice()).unwrap();

    assert_eq!(hdr.version, 0b0100);
    assert_eq!(hdr.src, 0b0110110011);
    assert_eq!(hdr.dst, 0b0111000000);
}