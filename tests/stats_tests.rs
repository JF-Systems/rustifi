#![cfg(feature = "unifi")]

use rustifi::models::{AccessType, Client, ClientAccess, ClientType};
use rustifi::stats::{aggregate_clients_by_device, get_device_client_stats, DeviceClientStats};

fn make_client(
    id: &str,
    client_type: ClientType,
    device_id: Option<&str>,
    access_type: Option<AccessType>,
) -> Client {
    Client {
        id: id.to_string(),
        client_type,
        name: Some(format!("Client {}", id)),
        connected_at: Some("2024-01-01T12:00:00Z".to_string()),
        ip_address: None,
        access: access_type.map(|t| ClientAccess { access_type: t }),
        uplink_device_id: device_id.map(String::from),
    }
}

#[test]
fn test_device_client_stats_new() {
    let stats = DeviceClientStats::new("device-abc");

    assert_eq!(stats.device_id, "device-abc");
    assert_eq!(stats.total_clients, 0);
    assert_eq!(stats.wired_clients, 0);
    assert_eq!(stats.wireless_clients, 0);
    assert_eq!(stats.guest_clients, 0);
    assert!(!stats.has_clients());
}

#[test]
fn test_device_client_stats_add_wireless_client() {
    let mut stats = DeviceClientStats::new("device-1");
    let client = make_client("c1", ClientType::Wireless, Some("device-1"), None);

    stats.add_client(&client);

    assert_eq!(stats.total_clients, 1);
    assert_eq!(stats.wireless_clients, 1);
    assert_eq!(stats.wired_clients, 0);
    assert_eq!(stats.guest_clients, 0);
    assert!(stats.has_clients());
}

#[test]
fn test_device_client_stats_add_wired_client() {
    let mut stats = DeviceClientStats::new("device-1");
    let client = make_client("c1", ClientType::Wired, Some("device-1"), None);

    stats.add_client(&client);

    assert_eq!(stats.total_clients, 1);
    assert_eq!(stats.wireless_clients, 0);
    assert_eq!(stats.wired_clients, 1);
    assert_eq!(stats.guest_clients, 0);
}

#[test]
fn test_device_client_stats_add_guest_client() {
    let mut stats = DeviceClientStats::new("device-1");
    let client = make_client(
        "c1",
        ClientType::Wireless,
        Some("device-1"),
        Some(AccessType::Guest),
    );

    stats.add_client(&client);

    assert_eq!(stats.total_clients, 1);
    assert_eq!(stats.wireless_clients, 1);
    assert_eq!(stats.guest_clients, 1);
}

#[test]
fn test_device_client_stats_non_guest_clients() {
    let mut stats = DeviceClientStats::new("device-1");
    stats.total_clients = 10;
    stats.guest_clients = 3;

    assert_eq!(stats.non_guest_clients(), 7);
}

#[test]
fn test_device_client_stats_non_guest_clients_all_guests() {
    let mut stats = DeviceClientStats::new("device-1");
    stats.total_clients = 5;
    stats.guest_clients = 5;

    assert_eq!(stats.non_guest_clients(), 0);
}

#[test]
fn test_aggregate_clients_by_device_single_device() {
    let clients = vec![
        make_client("c1", ClientType::Wireless, Some("device-1"), None),
        make_client("c2", ClientType::Wired, Some("device-1"), None),
        make_client(
            "c3",
            ClientType::Wireless,
            Some("device-1"),
            Some(AccessType::Guest),
        ),
    ];

    let stats = aggregate_clients_by_device(&clients);

    assert_eq!(stats.len(), 1);

    let device1 = stats.get("device-1").unwrap();
    assert_eq!(device1.total_clients, 3);
    assert_eq!(device1.wireless_clients, 2);
    assert_eq!(device1.wired_clients, 1);
    assert_eq!(device1.guest_clients, 1);
}

#[test]
fn test_aggregate_clients_by_device_multiple_devices() {
    let clients = vec![
        make_client("c1", ClientType::Wireless, Some("ap-1"), None),
        make_client("c2", ClientType::Wireless, Some("ap-1"), None),
        make_client("c3", ClientType::Wireless, Some("ap-2"), None),
        make_client("c4", ClientType::Wired, Some("switch-1"), None),
        make_client(
            "c5",
            ClientType::Wireless,
            Some("ap-2"),
            Some(AccessType::Guest),
        ),
    ];

    let stats = aggregate_clients_by_device(&clients);

    assert_eq!(stats.len(), 3);

    assert_eq!(stats.get("ap-1").unwrap().total_clients, 2);
    assert_eq!(stats.get("ap-2").unwrap().total_clients, 2);
    assert_eq!(stats.get("ap-2").unwrap().guest_clients, 1);
    assert_eq!(stats.get("switch-1").unwrap().total_clients, 1);
    assert_eq!(stats.get("switch-1").unwrap().wired_clients, 1);
}

#[test]
fn test_aggregate_clients_by_device_ignores_clients_without_device() {
    let clients = vec![
        make_client("c1", ClientType::Wireless, Some("device-1"), None),
        make_client("c2", ClientType::Wireless, None, None), // No device ID
        make_client("c3", ClientType::Wired, None, None),    // No device ID
    ];

    let stats = aggregate_clients_by_device(&clients);

    assert_eq!(stats.len(), 1);
    assert_eq!(stats.get("device-1").unwrap().total_clients, 1);
}

#[test]
fn test_aggregate_clients_by_device_empty_list() {
    let clients: Vec<Client> = vec![];
    let stats = aggregate_clients_by_device(&clients);

    assert!(stats.is_empty());
}

#[test]
fn test_aggregate_clients_by_device_all_without_device_id() {
    let clients = vec![
        make_client("c1", ClientType::Wireless, None, None),
        make_client("c2", ClientType::Wired, None, None),
    ];

    let stats = aggregate_clients_by_device(&clients);

    assert!(stats.is_empty());
}

#[test]
fn test_get_device_client_stats_with_matches() {
    let clients = vec![
        make_client("c1", ClientType::Wireless, Some("device-1"), None),
        make_client("c2", ClientType::Wired, Some("device-1"), None),
        make_client("c3", ClientType::Wireless, Some("device-2"), None),
    ];

    let stats = get_device_client_stats(&clients, "device-1");

    assert_eq!(stats.device_id, "device-1");
    assert_eq!(stats.total_clients, 2);
    assert_eq!(stats.wireless_clients, 1);
    assert_eq!(stats.wired_clients, 1);
}

#[test]
fn test_get_device_client_stats_no_matches() {
    let clients = vec![
        make_client("c1", ClientType::Wireless, Some("device-1"), None),
        make_client("c2", ClientType::Wireless, Some("device-2"), None),
    ];

    let stats = get_device_client_stats(&clients, "device-999");

    assert_eq!(stats.device_id, "device-999");
    assert_eq!(stats.total_clients, 0);
    assert!(!stats.has_clients());
}

#[test]
fn test_get_device_client_stats_empty_list() {
    let clients: Vec<Client> = vec![];
    let stats = get_device_client_stats(&clients, "any-device");

    assert_eq!(stats.device_id, "any-device");
    assert_eq!(stats.total_clients, 0);
}

#[test]
fn test_client_type_counts() {
    let clients = vec![
        make_client("c1", ClientType::Wireless, Some("device-1"), None),
        make_client("c2", ClientType::Wireless, Some("device-1"), None),
        make_client("c3", ClientType::Wireless, Some("device-1"), None),
        make_client("c4", ClientType::Wired, Some("device-1"), None),
        make_client("c5", ClientType::Wired, Some("device-1"), None),
        make_client("c6", ClientType::Unknown, Some("device-1"), None),
    ];

    let stats = aggregate_clients_by_device(&clients);
    let device1 = stats.get("device-1").unwrap();

    assert_eq!(device1.total_clients, 6);
    assert_eq!(device1.wireless_clients, 3);
    assert_eq!(device1.wired_clients, 2);
    // Unknown clients are counted in total but not in wired/wireless
    assert_eq!(device1.wireless_clients + device1.wired_clients, 5);
}

#[test]
fn test_access_types_guest_counting() {
    let clients = vec![
        make_client(
            "c1",
            ClientType::Wireless,
            Some("device-1"),
            Some(AccessType::Default),
        ),
        make_client(
            "c2",
            ClientType::Wireless,
            Some("device-1"),
            Some(AccessType::Guest),
        ),
        make_client(
            "c3",
            ClientType::Wireless,
            Some("device-1"),
            Some(AccessType::Guest),
        ),
        make_client(
            "c4",
            ClientType::Wireless,
            Some("device-1"),
            Some(AccessType::Blocked),
        ),
        make_client(
            "c5",
            ClientType::Wireless,
            Some("device-1"),
            Some(AccessType::Allowed),
        ),
        make_client("c6", ClientType::Wireless, Some("device-1"), None), // No access info
    ];

    let stats = aggregate_clients_by_device(&clients);
    let device1 = stats.get("device-1").unwrap();

    assert_eq!(device1.total_clients, 6);
    assert_eq!(device1.guest_clients, 2); // Only GUEST access type counts
    assert_eq!(device1.non_guest_clients(), 4);
}
