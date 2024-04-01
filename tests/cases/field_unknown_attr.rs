use scars::Header;

#[derive(Header)]
struct TestHeader {
    #[field(bit_len=4, asdf_qwer=10)]
    pub version: u8
}

fn main() {
    let _hdr = TestHeader {
        version: 1
    };
}