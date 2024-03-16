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
        ( $($size: literal -> $field: ident: $type: ident),* $(,)?)
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
        04 -> version: u8,
        04 -> hdr_len: u8,
        06 -> dscp: u8,
        02 -> ecn: u8,
        16 -> len: u16,
        16 -> id: u16,
        03 -> flags: u8,
        13 -> frag_offset: u16,
        08 -> ttl: u8,
        08 -> protocol: u8,
        16 -> checksum: u16,
        32 -> src: u32,
        32 -> dst: u32,
    )
);