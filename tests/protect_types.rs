#![cfg(feature = "protect")]

use rustifi::protect::models::{
    Camera, CameraPatch, Chime, Light, MetaInfo, ModelKey, Nvr, ProtectDeviceState, RtspsQuality,
    RtspsStreams, Sensor, Viewer,
};
use serde_json::json;

#[test]
fn test_camera_deserialization() {
    let json_data = json!({
        "id": "61ddb03ba0a70603e4004fab",
        "modelKey": "camera",
        "state": "CONNECTED",
        "name": "Front Door",
        "mac": "74ACB9000000",
        "model": "UVC G4 Doorbell",
        "type": "UVC G4 Doorbell",
        "firmwareVersion": "4.71.42",
        "isMicEnabled": true,
        "micVolume": 100,
        "videoMode": "default",
        "hdrType": "auto",
        "osdSettings": {
            "isNameEnabled": false,
            "isDateEnabled": false,
            "isLogoEnabled": true,
            "isDebugEnabled": false
        },
        "ledSettings": { "isEnabled": true },
        "lcdMessage": { "type": "LEAVE_PACKAGE_AT_DOOR", "text": "" },
        "smartDetectSettings": {
            "objectTypes": ["person", "vehicle"],
            "autoTrackingObjectTypes": [],
            "audioTypes": ["alrmSmoke"]
        },
        "recordingSettings": {
            "mode": "always",
            "prePaddingSecs": 3,
            "postPaddingSecs": 3
        },
        "featureFlags": {
            "hasHdr": true,
            "hasMic": true,
            "hasLedStatus": true,
            "smartDetectTypes": ["person", "vehicle"],
            "videoModes": ["default"],
            "isPtz": false
        }
    });

    let camera: Camera = serde_json::from_value(json_data).unwrap();
    assert_eq!(camera.id, "61ddb03ba0a70603e4004fab");
    assert_eq!(camera.model_key, ModelKey::Camera);
    assert_eq!(camera.state, ProtectDeviceState::Connected);
    assert_eq!(camera.name, "Front Door");
    assert_eq!(camera.mac.unwrap().as_str(), "74ACB9000000");
    assert_eq!(camera.mic_volume, Some(100));
    assert_eq!(camera.osd_settings.unwrap().is_logo_enabled, Some(true));
    assert_eq!(
        camera.smart_detect_settings.unwrap().object_types.unwrap(),
        vec!["person", "vehicle"]
    );
    assert!(camera.feature_flags.unwrap().has_hdr.unwrap());
}

#[test]
fn test_camera_minimal_and_unknown_fields() {
    // Only guaranteed fields plus an unknown field from a future release
    let json_data = json!({
        "id": "abc123",
        "modelKey": "camera",
        "state": "CONNECTED",
        "name": "Minimal Cam",
        "someFutureField": { "nested": true }
    });

    let camera: Camera = serde_json::from_value(json_data).unwrap();
    assert_eq!(camera.id, "abc123");
    assert_eq!(camera.mac, None);
    assert_eq!(camera.osd_settings, None);
}

#[test]
fn test_unknown_enum_variants_tolerated() {
    let json_data = json!({
        "id": "abc123",
        "modelKey": "somethingNew",
        "state": "SOMETHING_NEW",
        "name": "Future Device"
    });

    let camera: Camera = serde_json::from_value(json_data).unwrap();
    assert_eq!(camera.model_key, ModelKey::Unknown);
    assert_eq!(camera.state, ProtectDeviceState::Unknown);
}

#[test]
fn test_camera_patch_skips_none_fields() {
    let patch = CameraPatch {
        name: Some("New Name".to_string()),
        mic_volume: Some(50),
        ..Default::default()
    };

    let value = serde_json::to_value(&patch).unwrap();
    assert_eq!(value, json!({ "name": "New Name", "micVolume": 50 }));
}

#[test]
fn test_rtsps_streams_deserialization() {
    let json_data = json!({
        "high": "rtsps://192.168.1.1:7441/abcd1234",
        "medium": null,
        "low": "rtsps://192.168.1.1:7441/wxyz9876"
    });

    let streams: RtspsStreams = serde_json::from_value(json_data).unwrap();
    assert_eq!(
        streams.high.as_deref(),
        Some("rtsps://192.168.1.1:7441/abcd1234")
    );
    assert_eq!(streams.medium, None);
    assert_eq!(streams.package, None);
}

#[test]
fn test_rtsps_quality_serialization() {
    assert_eq!(
        serde_json::to_value(RtspsQuality::High).unwrap(),
        json!("high")
    );
    assert_eq!(RtspsQuality::Package.as_str(), "package");
}

#[test]
fn test_light_deserialization() {
    let json_data = json!({
        "id": "5f9b1c000000000000000000",
        "modelKey": "light",
        "state": "CONNECTED",
        "name": "Backyard Floodlight",
        "isLightOn": false,
        "isPirMotionDetected": false,
        "lightModeSettings": { "mode": "motion", "enableAt": "fulltime" },
        "lightDeviceSettings": {
            "isIndicatorEnabled": true,
            "ledLevel": 4,
            "pirDuration": 15000,
            "pirSensitivity": 50
        }
    });

    let light: Light = serde_json::from_value(json_data).unwrap();
    assert_eq!(light.name, "Backyard Floodlight");
    assert_eq!(light.is_light_on, Some(false));
    assert_eq!(
        light.light_mode_settings.unwrap().mode.as_deref(),
        Some("motion")
    );
    assert_eq!(light.light_device_settings.unwrap().led_level, Some(4));
}

#[test]
fn test_sensor_deserialization() {
    let json_data = json!({
        "id": "5f9b2d000000000000000000",
        "modelKey": "sensor",
        "state": "CONNECTED",
        "name": "Garage Door",
        "mountType": "door",
        "isOpened": false,
        "batteryStatus": { "percentage": 87, "isLow": false },
        "stats": {
            "light": { "value": 12.0, "status": "neutral" },
            "temperature": { "value": 21.5, "status": "neutral" },
            "humidity": { "value": 40.0, "status": "neutral" }
        },
        "motionSettings": { "isEnabled": true, "sensitivity": 80 }
    });

    let sensor: Sensor = serde_json::from_value(json_data).unwrap();
    assert_eq!(sensor.mount_type.as_deref(), Some("door"));
    assert_eq!(sensor.battery_status.unwrap().percentage, Some(87));
    let stats = sensor.stats.unwrap();
    assert_eq!(stats.temperature.unwrap().value, Some(21.5));
    assert_eq!(sensor.motion_settings.unwrap().sensitivity, Some(80));
}

#[test]
fn test_chime_deserialization() {
    let json_data = json!({
        "id": "5f9b3e000000000000000000",
        "modelKey": "chime",
        "state": "CONNECTED",
        "name": "Hallway Chime",
        "volume": 80,
        "cameraIds": ["61ddb03ba0a70603e4004fab"],
        "ringSettings": [
            { "cameraId": "61ddb03ba0a70603e4004fab", "repeatTimes": 1, "trackNo": 1, "volume": 80 }
        ]
    });

    let chime: Chime = serde_json::from_value(json_data).unwrap();
    assert_eq!(chime.volume, Some(80));
    assert_eq!(chime.camera_ids.unwrap().len(), 1);
    assert_eq!(chime.ring_settings.unwrap()[0].repeat_times, Some(1));
}

#[test]
fn test_viewer_deserialization() {
    let json_data = json!({
        "id": "5f9b4f000000000000000000",
        "modelKey": "viewer",
        "state": "DISCONNECTED",
        "name": "Lobby Screen",
        "liveview": "5f9b5a000000000000000000"
    });

    let viewer: Viewer = serde_json::from_value(json_data).unwrap();
    assert_eq!(viewer.state, ProtectDeviceState::Disconnected);
    assert_eq!(viewer.liveview.as_deref(), Some("5f9b5a000000000000000000"));
}

#[test]
fn test_nvr_deserialization() {
    let json_data = json!({
        "id": "5f9b6b000000000000000000",
        "modelKey": "nvr",
        "name": "Dream Machine Pro",
        "mac": "74ACB9111111",
        "version": "5.3.41",
        "uptime": 123456789i64,
        "hardwarePlatform": "al324"
    });

    let nvr: Nvr = serde_json::from_value(json_data).unwrap();
    assert_eq!(nvr.name, "Dream Machine Pro");
    assert_eq!(nvr.version.as_deref(), Some("5.3.41"));
    assert_eq!(nvr.uptime, Some(123456789));
}

#[test]
fn test_meta_info_deserialization() {
    let json_data = json!({ "applicationVersion": "6.2.88" });
    let info: MetaInfo = serde_json::from_value(json_data).unwrap();
    assert_eq!(info.application_version, "6.2.88");
}
