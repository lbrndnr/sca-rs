#![allow(unused)]

use scars::{Header, hdr::NBitVec};

#[derive(Header)]
struct TestHeader {
    #[field(bit_len(4))]
    pub version: u8,
    #[field(bit_len(32), cond(version > 1))]
    pub name: Option<u32>
}

fn main() {
    let _hdr = TestHeader {
        version: 1,
        name: Some(0xdead)
    };
}