#![cfg(feature = "unifi")]

use rustifi::models::{IpAddress, MacAddress};
use serde_json::json;
use std::net::Ipv4Addr;

#[test]
fn test_ip_address_deserialization() {
    let json_data = json!("192.168.1.100");
    let ip: IpAddress = serde_json::from_value(json_data).unwrap();
    assert_eq!(ip.0, Ipv4Addr::new(192, 168, 1, 100));
}

#[test]
fn test_ip_address_default() {
    let ip = IpAddress::default();
    assert_eq!(ip.0, Ipv4Addr::new(0, 0, 0, 0));
}

#[test]
fn test_ip_address_to_string() {
    let ip = IpAddress::new(Ipv4Addr::new(10, 0, 0, 1));
    assert_eq!(ip.to_string(), "10.0.0.1");
}

#[test]
fn test_mac_address_deserialization() {
    let json_data = json!("00:11:22:33:44:55");
    let mac: MacAddress = serde_json::from_value(json_data).unwrap();
    assert_eq!(mac.as_str(), "00:11:22:33:44:55");
}

#[test]
fn test_mac_address_default() {
    let mac = MacAddress::default();
    assert!(mac.as_str().is_empty());
}

#[test]
fn test_mac_address_new() {
    let mac = MacAddress::new("aa:bb:cc:dd:ee:ff");
    assert_eq!(mac.as_str(), "aa:bb:cc:dd:ee:ff");
}
