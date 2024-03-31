pub use header_derive::Header;

pub mod bit;

#[derive(Debug)]
pub enum Error {
    Decoding
}

pub trait ToBits {

    fn num_bits() -> usize;

}

// #[derive(Header)]
// struct IPv4 {
//     version: u8,
//     hdr_len: u8,
//     dscp: u8,
//     ecn: u8,
//     len: u16,
//     id: u16,
//     flags: u8,
//     frag_offset: u16,
//     ttl: u8,
//     protocol: u8,
//     checksum: u16,
//     src: u32,
//     dst: u32,
//     options: u64,
// }