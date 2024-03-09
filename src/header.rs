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
                use crate::bit::BitRange;
                let mut s = 0;
                $(
                    let val: Result<$type,_> = value.get_bit_range(s..s+$size);
                    let $field = val.map_err(|_| Self::Error::TypeTooShort)?;
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