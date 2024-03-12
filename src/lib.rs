mod header;
mod bit;

use header::{
    DecodingError,
    Header
};

// pub trait PacketParser {
//     fn split_header<H: Header>(&self) -> Result<(H, &[u8]), DecodingError>;
// }

// impl PacketParser for [u8] {
//     fn split_header<H: Header>(&self) -> Result<(H, &[u8]), DecodingError> {
//         let hlen = H::num_bits();
//         if self.len() < hlen {
//             return Err(DecodingError::Length)
//         }

//         // let hdr = &self[..hlen];
//         // why is this infallible?
//         let hdr = H::try_from(self)
//             .map_err(|_| DecodingError::Length)?;
        
//         Ok((hdr, &self[hlen..]))
//     }
// }

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
        let raw = [0b01000110, 0b11001101, 0b11000000];
        let hdr = TestHeader::try_from(raw.as_slice()).unwrap();

        assert_eq!(TestHeader::num_bits(), 18);
        assert_eq!(hdr.field_1, 0b0);
        assert_eq!(hdr.field_2, 0b1000110);
        assert_eq!(hdr.field_3, 0b1100110111);

        let hdr_raw: Vec<u8> = hdr.into();
        assert_eq!(hdr_raw.as_slice(), raw);

    }
}
