use scars::bit::BitRange;

#[test]
fn it_gets_single_byte_valid_range() {
    let xs = [0b10110111];
    let bit = xs.get_bit_range(0..2).unwrap();
    assert_eq!(bit, [0b10]);

    let bit = xs.get_bit_range(5..8).unwrap();
    assert_eq!(bit, [0b111]);

    let bit = xs.get_bit_range(3..7).unwrap();
    assert_eq!(bit, [0b1011]);
}

#[test]
fn it_gets_single_byte_invalid_range() {
    let xs = [0b10110111];
    let bit = xs.get_bit_range(0..10);
    assert!(bit.is_err());

    let bit = xs.get_bit_range(6..9);
    assert!(bit.is_err());
}

#[test]
fn it_gets_multi_byte_valid_range() {
    let xs = [0b10110111, 0b01111011];
    let bit = xs.get_bit_range(0..16).unwrap();
    assert_eq!(bit, xs);

    let bit = xs.get_bit_range(5..13).unwrap();
    assert_eq!(bit, [0b111, 0b0111]);
}

#[test]
fn it_gets_multi_byte_invalid_range() {
    let xs = [0b10110111, 0b01111011];

    let bit = xs.get_bit_range(5..17);
    assert!(bit.is_err());
}