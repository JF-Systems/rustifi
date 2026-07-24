#![cfg(feature = "unifi")]

use rustifi::models::{Device, DeviceType};
use serde_json::json;

#[test]
fn test_device_deserialization() {
    let json_data = json!({
        "id": "abc123",
        "mac": "00:11:22:33:44:55",
        "model": "U7MSH",
        "type": "uap",
        "name": "Test AP",
        "ip": "192.168.1.100",
        "version": "6.0.0.0000",
        "serial": "0000000000",
        "adopted": true,
        "disabled": false,
        "disconnected": false,
        "last_seen": 1613456789,
        "uptime": 86400
    });

    let device: Device = serde_json::from_value(json_data).unwrap();

    assert_eq!(device.id, "abc123");
    assert_eq!(device.mac.as_str(), "00:11:22:33:44:55");
    assert_eq!(device.model, "U7MSH");
    assert_eq!(device.type_field, DeviceType::AccessPoint);
    assert_eq!(device.name, "Test AP");
    assert_eq!(device.adopted, Some(true));
    assert_eq!(device.disabled, Some(false));
}

#[test]
fn test_device_type_aliases() {
    let uap: Device = serde_json::from_value(json!({
        "id": "1", "mac": "00:11:22:33:44:55", "model": "U7MSH", "type": "uap", "name": "AP"
    }))
    .unwrap();
    assert_eq!(uap.type_field, DeviceType::AccessPoint);

    let usw: Device = serde_json::from_value(json!({
        "id": "2", "mac": "00:11:22:33:44:56", "model": "USW-16-POE", "type": "usw", "name": "Switch"
    })).unwrap();
    assert_eq!(usw.type_field, DeviceType::Switch);

    let udm: Device = serde_json::from_value(json!({
        "id": "3", "mac": "00:11:22:33:44:57", "model": "UDM-PRO", "type": "udm", "name": "UDM"
    }))
    .unwrap();
    assert_eq!(udm.type_field, DeviceType::DreamMachine);

    let unknown: Device = serde_json::from_value(json!({
        "id": "4", "mac": "00:11:22:33:44:58", "model": "Unknown", "type": "xyz", "name": "Unknown"
    }))
    .unwrap();
    assert_eq!(unknown.type_field, DeviceType::Unknown);
}
