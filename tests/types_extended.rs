#![cfg(feature = "unifi")]

use rustifi::models::{AccessType, Client, ClientType};
use serde_json::json;

#[test]
fn test_client_deserialization() {
    let json_data = json!({
        "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
        "type": "WIRELESS",
        "name": "My Laptop",
        "connectedAt": "2019-08-24T14:15:22Z",
        "ipAddress": "192.168.1.50",
        "access": {
            "type": "DEFAULT"
        }
    });

    let client: Client = serde_json::from_value(json_data).unwrap();

    assert_eq!(client.id, "497f6eca-6276-4993-bfeb-53cbbbba6f08");
    assert_eq!(client.name, Some("My Laptop".to_string()));
    assert_eq!(
        client.ip_address.as_ref().unwrap().to_string(),
        "192.168.1.50"
    );
    assert_eq!(client.client_type, ClientType::Wireless);
    assert_eq!(
        client.connected_at,
        Some("2019-08-24T14:15:22Z".to_string())
    );
    assert!(client.is_wireless());
    assert!(client.is_connected());
    assert!(!client.is_blocked());
}

#[test]
fn test_client_type_variants() {
    let wireless: Client = serde_json::from_value(json!({
        "id": "1", "type": "WIRELESS"
    }))
    .unwrap();
    assert_eq!(wireless.client_type, ClientType::Wireless);
    assert!(wireless.is_wireless());
    assert!(!wireless.is_wired());

    let wired: Client = serde_json::from_value(json!({
        "id": "2", "type": "WIRED"
    }))
    .unwrap();
    assert_eq!(wired.client_type, ClientType::Wired);
    assert!(wired.is_wired());
    assert!(!wired.is_wireless());

    let unknown: Client = serde_json::from_value(json!({
        "id": "3", "type": "SOMETHING_NEW"
    }))
    .unwrap();
    assert_eq!(unknown.client_type, ClientType::Unknown);
}

#[test]
fn test_client_is_connected() {
    let connected: Client = serde_json::from_value(json!({
        "id": "1",
        "connectedAt": "2019-08-24T14:15:22Z"
    }))
    .unwrap();
    assert!(connected.is_connected());

    let disconnected: Client = serde_json::from_value(json!({
        "id": "2"
    }))
    .unwrap();
    assert!(!disconnected.is_connected());
}

#[test]
fn test_client_access_types() {
    let default_access: Client = serde_json::from_value(json!({
        "id": "1",
        "access": { "type": "DEFAULT" }
    }))
    .unwrap();
    assert_eq!(
        default_access.access.as_ref().unwrap().access_type,
        AccessType::Default
    );
    assert!(!default_access.is_blocked());

    let blocked: Client = serde_json::from_value(json!({
        "id": "2",
        "access": { "type": "BLOCKED" }
    }))
    .unwrap();
    assert_eq!(
        blocked.access.as_ref().unwrap().access_type,
        AccessType::Blocked
    );
    assert!(blocked.is_blocked());

    let allowed: Client = serde_json::from_value(json!({
        "id": "3",
        "access": { "type": "ALLOWED" }
    }))
    .unwrap();
    assert_eq!(
        allowed.access.as_ref().unwrap().access_type,
        AccessType::Allowed
    );
    assert!(!allowed.is_blocked());

    let no_access: Client = serde_json::from_value(json!({
        "id": "4"
    }))
    .unwrap();
    assert!(no_access.access.is_none());
    assert!(!no_access.is_blocked());
}
