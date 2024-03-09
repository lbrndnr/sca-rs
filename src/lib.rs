mod header;

#[cfg(test)]
mod tests {
    use super::*;

    make_header!(
        TestHeader (
            1  -> field_1: u128,
            7  -> field_2: u8,
            10 -> field_3: u32,
        )
    );
    

    #[test]
    fn it_works() {
        let raw = b"E\x00\x00\x14\x00\x01\x00\x00\n\x00\xc0\xa8\x01x\xc0\xa8\x01\x01";
        let hdr = TestHeader::try_from(raw).unwrap();

        assert_eq!(hdr.field_1, 0);
    }
}
