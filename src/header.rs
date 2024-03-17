use std::{net::Ipv4Addr, ops::Deref};

#[derive(Debug, Clone)]
pub enum DecodingError {
    Length,
    Field,
    TypeTooShort
}

#[macro_export]
macro_rules! sum {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),*) => ($h + sum!($($t),*));
}

pub trait Header: for<'a> TryFrom<&'a [u8]> {

    fn num_bits() -> usize;
    
}

#[macro_export]
macro_rules! make_header {
    (
        $name: ident
        ( $($field: ident: $type: ty, $size: literal $(if $cond: expr)*),+ $(,)?) 
    ) => {
        pub struct $name {
            $(
                $field: $type,
            )*
        }

        impl Header for $name {

            fn num_bits() -> usize {
                sum![$($size),*]
            }
        
        }

        impl TryFrom<&[u8]> for $name {
            type Error = DecodingError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                use crate::bit::BitRange;
                let mut s = 0;

                $(
                    let val: Result<$type,_> = value.get_bit_range(s..s+$size);
                    let $field = val.map_err(|_| Self::Error::TypeTooShort)?;
                    s += $size;
                )*

                Ok(Self {
                    $(
                        $field,
                    )*
                })
            }
        }

        impl From<$name> for Vec<u8> {
            
            fn from(value: $name) -> Vec<u8> {
                use core::mem::size_of;
                use crate::bit::BitRange;

                let len = ($name::num_bits() as f32 / 8.0).ceil() as usize;
                let mut res = vec![0; len];
                let mut s = 0;

                $(
                    let bytes = value.$field.to_be_bytes();
                    for i in 0..$size {
                        let bit = bytes.get_bit(8*size_of::<$type>()-$size+i).unwrap();
                        let mask = (bit as u8) << (7 - ((s+i) % 8));
                        println!("{i} -> {bit} flags: {:#b}", mask);
                        res[((s+i) / 8) as usize] |= mask;
                    }
                    s += $size;
                )*

                res
            }

        }

    }
}

make_header!(
    IPv4 (
        version: u8,         04,
        hdr_len: u8,         04,
        dscp: u8,            06,
        ecn: u8,             02,
        len: u16,            16,
        id: u16,             16,
        flags: u8,           03,
        frag_offset: u16,    13,
        ttl: u8,             08,
        protocol: u8,        08,
        checksum: u16,       16,
        src: Ipv4Field,      32,
        dst: Ipv4Field,      32,
        options: u64,        32,
    )
);

trait ToBeBytes {

    type Bytes;

    fn to_be_bytes(&self) -> Self::Bytes;

}

struct Ipv4Field(Ipv4Addr);

impl Deref for Ipv4Field {

    type Target = Ipv4Addr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl ToBeBytes for Ipv4Field {

    type Bytes = [u8; 4];

    fn to_be_bytes(&self) -> Self::Bytes {
        let val: u32 = self.0.into();
        val.to_be_bytes()
    }

}

impl From<usize> for Ipv4Field {

    fn from(value: usize) -> Self {
        Ipv4Field(Ipv4Addr::from(value as u32))
    }

}
