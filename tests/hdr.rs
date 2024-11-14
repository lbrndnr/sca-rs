mod common;

use scars::{
    TryFromUnchecked,
    hdr::IPv4
};
use common::{
    assert_bijective_serialization,
    raw
};

#[test]
fn it_deserializes_valid_ipv4() {
    let hdr = IPv4::try_from(raw::ipv4::VALID.as_slice()).unwrap();
    assert_eq!(hdr.version, 4);
    assert_eq!(hdr.ihl, 5);
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
    // assert!(hdr.options.is_none());
}

#[test]
fn it_deserializes_invalid_ipv4() {
    let hdr = IPv4::try_from(raw::ipv4::NO_OPTS.as_slice());
    assert_eq!(hdr.unwrap_err(), scars::Error::FieldDeserialization("options".to_string()));

    let hdr = IPv4::try_from_unchecked(raw::ipv4::NO_OPTS.as_slice()).unwrap();
    assert_eq!(hdr.version, 4);
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
    // assert!(hdr.options.is_none());

    let hdr = IPv4::try_from(raw::ipv4::OPTS.as_slice());
    assert_eq!(hdr.unwrap_err(), scars::Error::FieldDeserialization("options".to_string()));

    let hdr = IPv4::try_from_unchecked(raw::ipv4::OPTS.as_slice()).unwrap();
    assert_eq!(hdr.version, 4);
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
    // assert!(hdr.options.is_some());
}

#[test]
fn it_serializes_ipv4_bijectively() {
    assert_bijective_serialization::<IPv4>(&raw::ipv4::VALID);
}