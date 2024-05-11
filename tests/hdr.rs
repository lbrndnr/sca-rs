mod common;

use scars::hdr::IPv4;
use common::{
    assert_bijective_serialization,
    raw
};

#[test]
fn it_parses_ip4() {
    let hdr = IPv4::try_from(raw::IPV4.as_slice()).unwrap();
    assert_eq!(hdr.version, 6);
    assert_eq!(hdr.ihl, 12);
    assert_eq!(hdr.tos, 66);
    assert_eq!(hdr.len, 12345);
    assert_eq!(hdr.id, 56789);
    assert_eq!(hdr.flags, 3);
    assert_eq!(hdr.frag, 7777);
    assert_eq!(hdr.ttl, 24);
    assert_eq!(hdr.protocol, 212);
    assert_eq!(hdr.checksum, 34567);
    assert_eq!(hdr.src, 0xC0A80001);
    assert_eq!(hdr.dst, 0xC0A80002);
}

#[test]
fn it_serializes_ip4_bijectively() {
    assert_bijective_serialization::<IPv4>(&raw::IPV4);
}