#![cfg(feature = "protect")]

use rustifi::protect::models::{CameraPatch, RtspsQuality};
use rustifi::protect::ProtectClient;
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn client_for(server: &MockServer) -> ProtectClient {
    ProtectClient::new(server.uri(), "test-api-key").unwrap()
}

#[tokio::test]
async fn test_cameras_bare_array_and_api_key_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/proxy/protect/integration/v1/cameras"))
        .and(header("X-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": "cam1", "modelKey": "camera", "state": "CONNECTED", "name": "Front" },
            { "id": "cam2", "modelKey": "camera", "state": "DISCONNECTED", "name": "Back" }
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let cameras = client_for(&server).await.cameras().await.unwrap();
    assert_eq!(cameras.len(), 2);
    assert_eq!(cameras[0].id, "cam1");
    assert_eq!(cameras[1].name, "Back");
}

#[tokio::test]
async fn test_patch_camera_sends_only_set_fields() {
    let server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/proxy/protect/integration/v1/cameras/cam1"))
        .and(header("X-API-Key", "test-api-key"))
        .and(body_json(json!({ "name": "Renamed", "micVolume": 42 })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "cam1", "modelKey": "camera", "state": "CONNECTED", "name": "Renamed",
            "micVolume": 42
        })))
        .expect(1)
        .mount(&server)
        .await;

    let patch = CameraPatch {
        name: Some("Renamed".to_string()),
        mic_volume: Some(42),
        ..Default::default()
    };
    let camera = client_for(&server)
        .await
        .patch_camera("cam1", patch)
        .await
        .unwrap();
    assert_eq!(camera.name, "Renamed");
    assert_eq!(camera.mic_volume, Some(42));
}

#[tokio::test]
async fn test_snapshot_returns_binary_bytes() {
    let server = MockServer::start().await;
    // Not valid JSON and not valid UTF-8 — must round-trip untouched.
    let jpeg_bytes: Vec<u8> = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];

    Mock::given(method("GET"))
        .and(path("/proxy/protect/integration/v1/cameras/cam1/snapshot"))
        .and(query_param("highQuality", "true"))
        .and(header("X-API-Key", "test-api-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(jpeg_bytes.clone())
                .insert_header("content-type", "image/jpeg"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let bytes = client_for(&server)
        .await
        .snapshot("cam1", true)
        .await
        .unwrap();
    assert_eq!(bytes, jpeg_bytes);
}

#[tokio::test]
async fn test_create_rtsps_stream_body_and_response() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/proxy/protect/integration/v1/cameras/cam1/rtsps-stream",
        ))
        .and(body_json(json!({ "qualities": ["high", "low"] })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "high": "rtsps://10.0.0.1:7441/aaaa",
            "low": "rtsps://10.0.0.1:7441/bbbb"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let streams = client_for(&server)
        .await
        .create_rtsps_streams("cam1", vec![RtspsQuality::High, RtspsQuality::Low])
        .await
        .unwrap();
    assert_eq!(streams.high.as_deref(), Some("rtsps://10.0.0.1:7441/aaaa"));
    assert_eq!(streams.medium, None);
}

#[tokio::test]
async fn test_delete_rtsps_stream_query_params_and_empty_body() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/proxy/protect/integration/v1/cameras/cam1/rtsps-stream",
        ))
        .and(query_param("qualities", "high"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    client_for(&server)
        .await
        .delete_rtsps_streams("cam1", vec![RtspsQuality::High])
        .await
        .unwrap();
}

#[tokio::test]
async fn test_error_status_maps_to_request_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/proxy/protect/integration/v1/cameras/nope"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let err = client_for(&server).await.camera("nope").await.unwrap_err();
    assert!(matches!(err, rustifi::Error::Request(_)));
}

#[tokio::test]
async fn test_meta_info() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/proxy/protect/integration/v1/meta/info"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "applicationVersion": "6.2.88" })),
        )
        .mount(&server)
        .await;

    let info = client_for(&server).await.meta_info().await.unwrap();
    assert_eq!(info.application_version, "6.2.88");
}
