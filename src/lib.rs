pub use header_derive::Header;

pub mod bit;
pub mod hdr;

#[derive(Debug)]
pub enum Error {
    FieldDeserialization(String)
}

impl PartialEq for Error {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::FieldDeserialization(a), Error::FieldDeserialization(b)) => a == b,
        }
    }

}

/// Computes the overall bit length of a header.
pub trait BitLen {

    /// Sums the bit lengths of all fields in the header.
    /// Conditional fields whose condition is false are not
    /// taken into account.
    fn bit_len(&self) -> usize;
    
    /// Sums the bit lengths of all fields in the header.
    /// Compared to `bit_len`, this method ignores the
    /// conditional.
    fn bit_len_unchecked(&self) -> usize;

}

/// Similar to the [`From`] trait, but ignores conditional
/// fields. This is useful when you need to craft an invalid
/// packet for testing purposes. 
/// It is the reciprocal of [`IntoUnchecked`].
pub trait FromUnchecked<T>: Sized {
    /// Performs the conversion.
    #[must_use]
    fn from_unchecked(value: T) -> Self;
}

/// An attempted conversion that consumes `self`, which may or may not be
/// expensive.
pub trait IntoUnchecked<T>: Sized {
    /// Performs the conversion.
    #[must_use]
    fn into_unchecked(self) -> T;
}

/// Similar to the [`TryFrom`] trait, but ignores conditional
/// fields. This is useful when you need to craft an invalid
/// packet for testing purposes. 
/// It is the reciprocal of [`TryIntoUnchecked`].
pub trait TryFromUnchecked<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from_unchecked(value: T) -> Result<Self, Self::Error>;
}

/// An attempted conversion that consumes `self`, which may or may not be
/// expensive.
pub trait TryIntoUnchecked<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_into_unchecked(self) -> Result<T, Self::Error>;
}