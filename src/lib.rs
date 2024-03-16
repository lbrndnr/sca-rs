pub mod header;
pub mod bit;

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
