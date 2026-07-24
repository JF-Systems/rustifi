#![cfg(feature = "unifi")]

use rustifi::api::clients::GetClients;
use rustifi::api::devices::GetDevices;
use rustifi::api::Endpoint;
use rustifi::pagination::DEFAULT_PAGE_SIZE;
use rustifi::response::SiteResponse;

#[test]
fn test_default_page_size() {
    assert_eq!(DEFAULT_PAGE_SIZE, 100);
}

#[test]
fn test_get_clients_without_pagination() {
    let endpoint = GetClients::new("site-123");

    assert_eq!(endpoint.site_id, "site-123");
    assert!(endpoint.offset.is_none());
    assert!(endpoint.limit.is_none());

    let params = endpoint.query_params();
    assert!(params.is_empty());
}

#[test]
fn test_get_clients_with_pagination() {
    let endpoint = GetClients::with_pagination("site-123", 50, 25);

    assert_eq!(endpoint.site_id, "site-123");
    assert_eq!(endpoint.offset, Some(50));
    assert_eq!(endpoint.limit, Some(25));

    let params = endpoint.query_params();
    assert_eq!(params.len(), 2);
    assert!(params.contains(&("offset", "50".to_string())));
    assert!(params.contains(&("limit", "25".to_string())));
}

#[test]
fn test_get_clients_builder_pattern() {
    let endpoint = GetClients::new("site-123").offset(100).limit(50);

    assert_eq!(endpoint.offset, Some(100));
    assert_eq!(endpoint.limit, Some(50));

    let params = endpoint.query_params();
    assert_eq!(params.len(), 2);
}

#[test]
fn test_get_devices_without_pagination() {
    let endpoint = GetDevices::new("site-456");

    assert_eq!(endpoint.site_id, "site-456");
    assert!(endpoint.offset.is_none());
    assert!(endpoint.limit.is_none());

    let params = endpoint.query_params();
    assert!(params.is_empty());
}

#[test]
fn test_get_devices_with_pagination() {
    let endpoint = GetDevices::with_pagination("site-456", 0, 100);

    assert_eq!(endpoint.site_id, "site-456");
    assert_eq!(endpoint.offset, Some(0));
    assert_eq!(endpoint.limit, Some(100));

    let params = endpoint.query_params();
    assert_eq!(params.len(), 2);
}

#[test]
fn test_get_devices_builder_pattern() {
    let endpoint = GetDevices::new("site-456").offset(200).limit(100);

    assert_eq!(endpoint.offset, Some(200));
    assert_eq!(endpoint.limit, Some(100));
}

#[test]
fn test_site_response_has_more_true() {
    let response: SiteResponse<String> = SiteResponse {
        data: vec!["item1".to_string(), "item2".to_string()],
        offset: 0,
        limit: 2,
        count: 2,
        total_count: 10,
    };

    assert!(response.has_more());
    assert_eq!(response.next_offset(), Some(2));
}

#[test]
fn test_site_response_has_more_false() {
    let response: SiteResponse<String> = SiteResponse {
        data: vec!["item1".to_string(), "item2".to_string()],
        offset: 8,
        limit: 2,
        count: 2,
        total_count: 10,
    };

    assert!(!response.has_more());
    // next_offset() returns None when there are no more pages
    assert_eq!(response.next_offset(), None);
}

#[test]
fn test_site_response_empty() {
    let response: SiteResponse<String> = SiteResponse {
        data: vec![],
        offset: 0,
        limit: 100,
        count: 0,
        total_count: 0,
    };

    assert!(!response.has_more());
    // next_offset() returns None when there are no more pages
    assert_eq!(response.next_offset(), None);
}

#[test]
fn test_site_response_exact_page() {
    // When we're at the last page exactly
    let response: SiteResponse<String> = SiteResponse {
        data: vec!["item".to_string(); 10],
        offset: 90,
        limit: 10,
        count: 10,
        total_count: 100,
    };

    assert!(!response.has_more()); // 90 + 10 = 100, not less than 100
                                   // next_offset() returns None when there are no more pages
    assert_eq!(response.next_offset(), None);
}

#[test]
fn test_site_response_partial_last_page() {
    // When the last page has fewer items than the limit
    let response: SiteResponse<String> = SiteResponse {
        data: vec!["item".to_string(); 5],
        offset: 95,
        limit: 10,
        count: 5,
        total_count: 100,
    };

    assert!(!response.has_more()); // 95 + 5 = 100, not less than 100
                                   // next_offset() returns None when there are no more pages
    assert_eq!(response.next_offset(), None);
}
