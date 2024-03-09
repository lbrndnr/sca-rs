use std::ops::Range;

#[derive(Debug, Clone)]
pub enum DecodingError {
    InvalidLength,
    InvalidField
}

#[macro_export]
macro_rules! sum {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),*) => ($h + sum!($($t),*));
}

/// A trait for getting subsections of bits from containers of bytes.
pub trait BitRange {

    fn get_bit_range(&self, range: Range<u32>) -> u32;
    fn get_bit(&self, bit: u32) -> bool;
}

// This automatically works with Vec<u8> thanks to ~deref magic~
impl<const N: usize> BitRange for [u8; N] {
    fn get_bit_range(&self, range: Range<u32>) -> u32 {
        let start_bit = range.start;
        let end_bit = range.end;
        let length = end_bit - start_bit;

        assert!(end_bit/8 <= self.len() as u32);
        assert!(length < 32);

        let mut result: u32 = 0;
        for (i, off) in (start_bit..end_bit).zip(1..) {
            result |= (self.get_bit(i) as u32) << (length-off);
        }
        result
    }

    fn get_bit(&self, bit: u32) -> bool {
        assert!(bit/8 < self.len() as u32);

        let byte = self[(bit/8) as usize] as u32;
        (byte >> 7-bit%8) & 1 == 1
    }
}

#[macro_export]
macro_rules! make_header {
    (
        $name: ident
        ( $($size: literal -> $field: ident: $type: ident),* $(,)?)
    )  => {
        pub struct $name<'a> {
            slice: &'a [u8; sum![$($size),*]],
            $(
                $field: $type,
            )*
        }


        impl<'a> TryFrom<&'a [u8; sum![$($size),*]]> for $name<'a> {
            type Error = header::DecodingError;

            fn try_from(value: &'a [u8; sum![$($size),*]]) -> Result<Self, Self::Error> {
                use crate::header::BitRange;
                let mut s = 0;
                $(
                    let $field = value.get_bit_range(s..s+$size) as $type;
                    s += $size;
                )*

                Ok(Self {
                    slice: value,
                    $(
                        $field,
                    )*
                })
            }
        }

        // impl<'a> TryFrom<&'a [u8]> for $name<'a> {
        //     type Error = header::DecodingError;

        //     fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        //         Self::try_from(value.try_into())
        //     }
        // }
    }
}