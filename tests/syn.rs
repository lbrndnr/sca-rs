#[test]
fn it_compiles_bit_len() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/field_bit_len.rs");
}

#[test]
fn it_reports_unknown_attrs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/cases/field_unknown_attr.rs")
}