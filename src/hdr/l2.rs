use header_derive::CrateHeader;

#[derive(CrateHeader)]
pub struct Ethernet {
    #[field(bit_len(48))]
    pub dst: u64,

    #[field(bit_len(48))]
    pub src: u64,

    #[field(bit_len(16))]
    pub r#type: u16,
}
