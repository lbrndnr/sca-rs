use header_derive::CrateHeader;

#[derive(CrateHeader)]
pub struct IPv4 {
    #[field(bit_len(4))]
    version: u8,

    #[field(bit_len(4))]
    ihl: u8,

    #[field(bit_len(6))]
    dscp: u8,

    #[field(bit_len(2))]
    ecn: u8,

    #[field(bit_len(16))]
    len: u16,

    #[field(bit_len(16))]
    id: u16,

    #[field(bit_len(3))]
    flags: u8,

    #[field(bit_len(13))]
    frag_offset: u16,

    #[field(bit_len(8))]
    ttl: u8,

    #[field(bit_len(8))]
    protocol: u8,

    #[field(bit_len(16))]
    checksum: u16,

    #[field(bit_len(32))]
    src: u32,

    #[field(bit_len(32))]
    dst: u32,

    #[field(bit_len(16), cond(ihl > 5))]
    options: u64,
}