mod common;

use common::TestHeader;
use scars::BitLen;

#[test]
fn it_counts_bit_len() {
    let hdr = TestHeader {
        version: 1,
        src: 1,
        dst: Some(1),
    };

    assert_eq!(hdr.bit_len(), 14);
    assert_eq!(hdr.bit_len_unchecked(), 24);
}
