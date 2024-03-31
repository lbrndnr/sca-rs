use scars::Header;
    
#[derive(Header)]
pub struct TestHeader {
    #[field(bit_len=10)]
    version: u8,
    #[field(bit_len=16)]
    src: u16,
    #[field(bit_len=16)]
    dst: u16,
}