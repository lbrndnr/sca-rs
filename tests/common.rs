use scars::Header;
    
#[derive(Header)]
pub struct TestHeader {
    #[field(bit_len=4)]
    pub version: u8,
    #[field(bit_len=10)]
    pub src: u16,
    #[field(bit_len=10)]
    pub dst: u16,
}