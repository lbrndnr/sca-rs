pub use header_derive::Header;

pub mod bit;
pub mod hdr;

#[derive(Debug)]
pub enum Error {
    Decoding
}

pub trait ToBits {

    fn num_bits(&self) -> usize;

}