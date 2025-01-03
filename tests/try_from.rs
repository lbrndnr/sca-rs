use common::TestHeader;

mod common;

#[test]
fn it_deserializes_fields() {
    let raw = [0b01000110, 0b11001101, 0b11000000];
    let hdr = TestHeader::try_from(raw.as_slice()).unwrap();

    assert_eq!(hdr.version, 0b0100);
    assert_eq!(hdr.src, 0b0110110011);
    assert_eq!(hdr.dst, Some(0b0111000000));
}

#[test]
fn it_deserializes_cond_fields() {
    let raw = [0b00010110, 0b11001101, 0b11000000];
    let hdr = TestHeader::try_from(raw.as_slice()).unwrap();

    assert_eq!(hdr.version, 0b0001);
    assert_eq!(hdr.src, 0b0110110011);
    assert_eq!(hdr.dst, None);
}

#[test]
fn it_compiles_bit_len() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/field_bit_len.rs");
}

#[test]
fn it_compiles_cond() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/field_cond.rs");
}

#[test]
fn it_reports_unknown_attrs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/cases/field_unknown_attr.rs")
}