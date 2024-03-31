use std::ops::Range;
use core::mem::size_of;

#[derive(Debug, Clone)]
pub enum BitError {
    Index,
    Length
}

/// A trait for getting subsections of bits from containers of bytes.
pub trait BitRange {
    fn get_bit_range<T>(&self, range: Range<usize>) -> Result<T, BitError> where T: TryFrom<usize>;
    fn get_bit(&self, bit: usize) -> Result<bool, BitError>;
}

impl BitRange for [u8] {
    fn get_bit_range<T>(&self, range: Range<usize>) -> Result<T, BitError> where T: TryFrom<usize> {
        let start_bit = range.start;
        let end_bit = range.end;
        let length = end_bit - start_bit;

        if end_bit/8 > self.len() {
            return Err(BitError::Index)
        }
        if length > 8*size_of::<T>() {
            return Err(BitError::Length)
        }

        let mut res: usize = 0;
        for (i, off) in (start_bit..end_bit).zip(1..) {
            let bit = self.get_bit(i);
            if let Err(e) = bit { return Err(e) }

            res |= (bit.unwrap() as usize) << (length-off);
        }
        
        T::try_from(res)
            .map_err(|_| BitError::Length)
    }

    fn get_bit(&self, bit: usize) -> Result<bool, BitError> {
        if bit/8 >= self.len() {
            return Err(BitError::Index);
        }

        let byte = self[bit/8];
        Ok((byte >> 7-bit%8) & 1 == 1)
    }
}
