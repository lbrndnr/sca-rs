pub use header_derive::Header;

pub mod hdr;

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldDeserialization(String)
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

pub trait TryIntoBytes<T>: Sized {

    fn try_into_bytes(self) -> Result<T, Error>;

}

impl TryIntoBytes<u64> for Vec<u8> {
    
    fn try_into_bytes(self) -> Result<u64, Error> {
        u64::try_from_bytes(self.as_slice())
    }

}

impl TryIntoBytes<u32> for Vec<u8> {
    
    fn try_into_bytes(self) -> Result<u32, Error> {
        u32::try_from_bytes(self.as_slice())
    }

}

impl TryIntoBytes<u16> for Vec<u8> {
    
    fn try_into_bytes(self) -> Result<u16, Error> {
        u16::try_from_bytes(self.as_slice())
    }

}

impl TryIntoBytes<u8> for Vec<u8> {
    
    fn try_into_bytes(self) -> Result<u8, Error> {
        u8::try_from_bytes(self.as_slice())
    }

}

impl TryIntoBytes<Vec<u8>> for Vec<u8> {
    
    fn try_into_bytes(self) -> Result<Vec<u8>, Error> {
        Ok(self)
    }

}

pub trait TryFromBytes: Sized {

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error>;

}

impl TryFromBytes for u64 {

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(u64::from_be_bytes(bytes.try_into().unwrap()))
    }

}

impl TryFromBytes for u32 {

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(u32::from_be_bytes(bytes.try_into().unwrap()))
    }

}

impl TryFromBytes for u16 {

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(u16::from_be_bytes(bytes.try_into().unwrap()))
    }

}

impl TryFromBytes for u8 {

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(u8::from_be_bytes(bytes.try_into().unwrap()))
    }

}

impl TryFromBytes for Vec<u8> {

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(bytes.to_vec())
    }
    
}