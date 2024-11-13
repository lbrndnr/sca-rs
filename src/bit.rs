use std::ops::Range;
use core::mem::size_of;

#[derive(Debug, Clone)]
pub enum BitError {
    Index,
}

/// A trait for getting subsections of bits from containers of bytes.
pub trait BitRange {
    fn get_bit_range(&self, range: Range<usize>) -> Result<Vec<u8>, BitError>;
    fn get_bit(&self, bit: usize) -> Result<bool, BitError>;
}

impl BitRange for [u8] {
    fn get_bit_range(&self, range: Range<usize>) -> Result<Vec<u8>, BitError> {
        let start_bit = range.start;
        let end_bit = range.end;
        let len_bit = end_bit - start_bit;

        if end_bit > 8*self.len() || start_bit > 8*self.len() || start_bit > end_bit {
            return Err(BitError::Index)
        }
        if len_bit == 0 { 
            return Ok(vec![]); 
        }
        
        let len = len_bit.div_ceil(8);

        println!("start_bit: {}, end_bit: {}, len: {}", start_bit, end_bit, len);

        let start_mask = 0xFF >> start_bit%8;

        let res = self.iter()
            .skip(start_bit/8)
            .take(len)
            .enumerate()
            .map(|(i, byte)| {
                let mut byte = *byte;
                if i == 0 { byte &= start_mask }
                if i == len-1 && end_bit%8 != 0 { 
                    byte >>= 8-(end_bit%8);
                }
                
                byte
            })
            .collect();
        Ok(res)
    }

    fn get_bit(&self, bit: usize) -> Result<bool, BitError> {
        if bit/8 >= self.len() {
            return Err(BitError::Index);
        }

        let byte = self[bit/8];
        Ok((byte >> 7-bit%8) & 1 == 1)
    }
}
