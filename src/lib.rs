pub use header_derive::Header;

pub mod bit;
pub mod hdr;

#[derive(Debug)]
pub enum Error {
    Decoding
}

pub trait BitLen {

    fn bit_len(&self) -> usize;
    
    fn bit_len_unchecked(&self) -> usize;

}