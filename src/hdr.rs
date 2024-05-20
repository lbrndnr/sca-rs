#![allow(unused)]

use header_derive::CrateHeader;

#[derive(CrateHeader)]
pub struct Ethernet {
    #[field(bit_len(48))]
    dst: u64,

    #[field(bit_len(48))]
    src: u64,

    #[field(bit_len(16))]
    r#type: u16,
}

#[derive(CrateHeader)]
pub struct IPv4 {
    #[field(bit_len(4))]
    pub version: u8,

    #[field(bit_len(4))]
    pub ihl: u8,

    #[field(bit_len(8))]
    pub tos: u8,

    #[field(bit_len(16))]
    pub len: u16,

    #[field(bit_len(16))]
    pub id: u16,

    #[field(bit_len(3))]
    pub flags: u8,

    #[field(bit_len(13))]
    pub frag: u16,

    #[field(bit_len(8))]
    pub ttl: u8,

    #[field(bit_len(8))]
    pub protocol: u8,

    #[field(bit_len(16))]
    pub checksum: u16,

    #[field(bit_len(32))]
    pub src: u32,

    #[field(bit_len(32))]
    pub dst: u32,

    #[field(bit_len(16), cond(ihl > 5))]
    pub options: Option<u64>,
}