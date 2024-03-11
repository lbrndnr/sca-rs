mod header;
mod bit;

use header::{
    DecodingError,
    Header
};

pub trait PacketParser {
    fn split_header<'a, H: Header<'a>>(&'a self) -> Result<(H, &[u8]), DecodingError>;
}

impl<const N: usize> PacketParser for [u8; N] {
    fn split_header<'a, H: Header<'a>>(&'a self) -> Result<(H, &[u8]), DecodingError> {
        let hlen = H::len();
        if self.len() < hlen {
            return Err(DecodingError::Length)
        }

        // let hdr = &self[..hlen];
        // why is this infallible?
        let hdr = H::try_from(self)
            .map_err(|_| DecodingError::Length)?;
        
        Ok((hdr, &self[hlen..]))
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;

    use super::*;

    make_header!(
        TestHeader (
            1  -> field_1: u128,
            7  -> field_2: u8,
            10 -> field_3: u32,
        )
    );

    #[test]
    fn it_works() {
        let raw = b"E\x00\x00\x14\x00\x01\x00\x00\n\x00\xc0\xa8\x01x\xc0\xa8\x01\x01";
        let hdr = TestHeader::try_from(raw.as_slice()).unwrap();

        assert_eq!(TestHeader::len(), 18);
        assert_eq!(hdr.field_1, 0);
    }
}
