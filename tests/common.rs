#![allow(unused)]

// TODO: nbitvec shouldn't get imported
use scars::{Header, hdr::NBitVec};
use std::fmt::Debug;
    
#[derive(Header)]
pub struct TestHeader {
    #[field(bit_len(4))]
    pub version: u8,
    #[field(bit_len(10))]
    pub src: u16,
    #[field(bit_len(10), cond(version > 1))]
    pub dst: Option<u16>,
}

#[allow(dead_code)]
pub fn assert_bijective_serialization<'a, T: TryFrom<&'a [u8]>>(raw: &'a [u8]) 
    where Vec<u8>: From<T>,
          <T as TryFrom<&'a [u8]>>::Error: Debug {
    let hdr = T::try_from(raw);
    assert!(hdr.is_ok());
    let hdr = hdr.unwrap();
    let hdr_raw: Vec<u8> = hdr.into();

    assert_eq!(hdr_raw.as_slice(), raw);
}

#[allow(dead_code)]
pub mod raw {
    pub mod ipv4 {
        // IP(version=4, ihl=5, tos=66, len=12345, id=56789, flags=3, frag=7777, ttl=24, proto=212, chksum=34567, src="192.168.0.1", dst="192.168.0.2")
        pub static VALID: [u8; 20] = [0b1000101, 0b1000010, 0b110000, 0b111001, 0b11011101, 0b11010101, 0b1111110, 0b1100001, 0b11000, 0b11010100, 0b10000111, 0b111, 0b11000000, 0b10101000, 0b0, 0b1, 0b11000000, 0b10101000, 0b0, 0b10];

        // IP(version=4, ihl=12, tos=66, len=12345, id=56789, flags=3, frag=7777, ttl=24, proto=212, chksum=34567, src="192.168.0.1", dst="192.168.0.2")
        pub static NO_OPTS: [u8; 20] = [0b1001100, 0b1000010, 0b110000, 0b111001, 0b11011101, 0b11010101, 0b1111110, 0b1100001, 0b11000, 0b11010100, 0b10000111, 0b111, 0b11000000, 0b10101000, 0b0, 0b1, 0b11000000, 0b10101000, 0b0, 0b10];

        // IP(version=4, ihl=12, tos=66, len=12345, id=56789, flags=3, frag=7777, ttl=24, proto=212, chksum=34567, src="192.168.0.1", dst="192.168.0.2", options='\x010203')
        pub static OPTS: [u8; 28] = [0b1001100, 0b1000010, 0b110000, 0b111001, 0b11011101, 0b11010101, 0b1111110, 0b1100001, 0b11000, 0b11010100, 0b10000111, 0b111, 0b11000000, 0b10101000, 0b0, 0b1, 0b11000000, 0b10101000, 0b0, 0b10, 0b1, 0b110000, 0b110010, 0b110000, 0b110011, 0b0, 0b0, 0b0];
    }
}