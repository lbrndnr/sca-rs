mod common;

use scars::ToBits;
use common::TestHeader;

#[test]
fn it_sums_bit_lens() {
    assert!(TestHeader::num_bits() == 24);
}