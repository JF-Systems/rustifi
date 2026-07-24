#![cfg(feature = "unifi")]

use rustifi::response::{ApiResponse, PaginatedResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize, PartialEq)]
pub struct MockSite {
    pub id: String,
    pub name: String,
    pub desc: String,
}

#[test]
fn test_api_response_deserialization() {
    let json_data = json!({
        "meta": {
            "rc": "ok"
        },
        "data": [
            {
                "id": "site1",
                "name": "default",
                "desc": "My Site"
            }
        ]
    });

    let response: ApiResponse<Vec<MockSite>> = serde_json::from_value(json_data).unwrap();

    let meta = response.meta.expect("meta should be present");
    assert_eq!(meta.rc, "ok");
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, "site1");
}

#[test]
fn test_paginated_response_deserialization() {
    let json_data = json!({
        "data": [
            {"id": "1", "name": "device1", "desc": "Device 1"},
            {"id": "2", "name": "device2", "desc": "Device 2"}
        ],
        "meta": {
            "total_count": 10,
            "first": 1,
            "last": 2
        }
    });

    let response: PaginatedResponse<MockSite> = serde_json::from_value(json_data).unwrap();

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.meta.total_count, 10);
    assert_eq!(response.meta.first, 1);
    assert_eq!(response.meta.last, 2);
}

#[test]
fn test_response_without_meta() {
    let json_data = json!({
        "data": [
            {"id": "1", "name": "test", "desc": "Test"}
        ]
    });

    let response: ApiResponse<Vec<MockSite>> = serde_json::from_value(json_data).unwrap();

    assert!(response.meta.is_none());
    assert_eq!(response.data.len(), 1);
}

#[test]
fn test_response_with_errors() {
    let json_data = json!({
        "meta": {"rc": "error"},
        "data": [],
        "errors": [
            {"code": "AUTH_FAIL", "message": "Authentication failed"}
        ]
    });

    let response: ApiResponse<Vec<MockSite>> = serde_json::from_value(json_data).unwrap();

    assert!(response.errors.is_some());
    let errors = response.errors.unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].code, "AUTH_FAIL");
}
