use scars::{
    header::Header,
    make_header,
    bit::BitRange, sum, header::DecodingError, bit,
};

#[derive(Clone, Debug, Default, PartialEq)]
enum TestEnum {
    #[default]
    Opt1,
    Opt2
}

impl From<usize> for TestEnum {

    fn from(value: usize) -> Self {
        if value > 1 {
            Self::Opt1
        }
        else {
            Self::Opt2
        }
    }

}

impl TestEnum {

    fn to_be_bytes(&self) -> [u8; 1] {
        match self {
            Self::Opt1 => [1],
            Self::Opt2 => [0]
        }
    }

}

make_header!(
    TestHeader (
        field_1: u128,     1,
        field_2: u8,       7,
        field_3: u32,     10,
        field_4: TestEnum, 3,
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
    assert_eq!(hdr.field_4, TestEnum::Opt2);

    let hdr_raw: Vec<u8> = hdr.into();
    assert_eq!(hdr_raw.as_slice(), raw);

}