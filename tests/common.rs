use scars::Header;
    
#[derive(Default, Header)]
pub struct TestHeader {
    #[field(bit_len=10)]
    pub version: u8,
    #[field(bit_len=16)]
    pub src: u16,
    #[field(bit_len=16)]
    pub dst: u16,
}