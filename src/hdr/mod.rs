// TODO: is there a better way to remove the unused variable warnings?
#![allow(unused)]
pub mod l2;
pub mod l3;
pub mod l7;

pub use bitvec::{order::Msb0, vec::BitVec};
pub type NBitVec = BitVec<u8, Msb0>;
