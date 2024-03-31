mod common;

use scars::ToBits;
use common::TestHeader;

#[test]
fn it_implemenets_to_bits() {
    assert!(TestHeader::num_bits() == 24);
}