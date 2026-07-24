#![cfg(feature = "protect")]

use rustifi::protect::models::{
    DeviceUpdate, ModelKey, ProtectEvent, ProtectEventType, SmartDetectType, WsAction, WsMessage,
};
use serde_json::json;

#[test]
fn test_smart_detect_add_frame() {
    let json_data = json!({
        "type": "add",
        "item": {
            "id": "68386a43006bf603e4002b3d",
            "modelKey": "event",
            "type": "smartDetectZone",
            "start": 1748517571853i64,
            "device": "61ddb03ba0a70603e4004fab",
            "smartDetectTypes": ["person"]
        }
    });

    let msg: WsMessage<ProtectEvent> = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.action, WsAction::Add);
    assert_eq!(msg.item.model_key, ModelKey::Event);
    assert_eq!(msg.item.event_type, ProtectEventType::SmartDetectZone);
    assert_eq!(msg.item.start, 1748517571853);
    assert_eq!(msg.item.end, None);
    assert_eq!(msg.item.device, "61ddb03ba0a70603e4004fab");
    assert_eq!(msg.item.smart_detect_types, vec![SmartDetectType::Person]);
}

#[test]
fn test_smart_detect_update_frame_with_end() {
    let json_data = json!({
        "type": "update",
        "item": {
            "id": "68386a43006bf603e4002b3d",
            "modelKey": "event",
            "type": "smartDetectZone",
            "start": 1748517571853i64,
            "end": 1748517581453i64,
            "device": "61ddb03ba0a70603e4004fab",
            "smartDetectTypes": ["face", "person"]
        }
    });

    let msg: WsMessage<ProtectEvent> = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.action, WsAction::Update);
    assert_eq!(msg.item.end, Some(1748517581453));
    assert_eq!(
        msg.item.smart_detect_types,
        vec![SmartDetectType::Face, SmartDetectType::Person]
    );
}

#[test]
fn test_motion_event_frame() {
    let json_data = json!({
        "type": "add",
        "item": {
            "id": "68386a43006bf603e4002b3e",
            "modelKey": "event",
            "type": "motion",
            "start": 1748517571853i64,
            "device": "61ddb03ba0a70603e4004fab"
        }
    });

    let msg: WsMessage<ProtectEvent> = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.item.event_type, ProtectEventType::Motion);
    assert!(msg.item.smart_detect_types.is_empty());
}

#[test]
fn test_doorbell_ring_frame() {
    let json_data = json!({
        "type": "add",
        "item": {
            "id": "68386a43006bf603e4002b3f",
            "modelKey": "event",
            "type": "doorbell",
            "start": 1748517571853i64,
            "device": "61ddb03ba0a70603e4004fab"
        }
    });

    let msg: WsMessage<ProtectEvent> = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.item.event_type, ProtectEventType::Doorbell);
}

#[test]
fn test_unknown_event_and_detect_types_tolerated() {
    let json_data = json!({
        "type": "add",
        "item": {
            "id": "abc",
            "modelKey": "event",
            "type": "someFutureEventType",
            "start": 1i64,
            "device": "dev1",
            "smartDetectTypes": ["hoverboard", "person"]
        }
    });

    let msg: WsMessage<ProtectEvent> = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.item.event_type, ProtectEventType::Unknown);
    assert_eq!(
        msg.item.smart_detect_types,
        vec![SmartDetectType::Unknown, SmartDetectType::Person]
    );
}

#[test]
fn test_unknown_ws_action_tolerated() {
    let json_data = json!({
        "type": "somethingElse",
        "item": {
            "id": "abc",
            "modelKey": "event",
            "type": "motion",
            "start": 1i64,
            "device": "dev1"
        }
    });

    let msg: WsMessage<ProtectEvent> = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.action, WsAction::Unknown);
}

#[test]
fn test_device_update_frame() {
    let json_data = json!({
        "type": "update",
        "item": {
            "id": "61ddb03ba0a70603e4004fab",
            "modelKey": "camera",
            "state": "CONNECTED",
            "isMicEnabled": false
        }
    });

    let msg: WsMessage<DeviceUpdate> = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.action, WsAction::Update);
    assert_eq!(msg.item.id, "61ddb03ba0a70603e4004fab");
    assert_eq!(msg.item.model_key, ModelKey::Camera);
    assert_eq!(msg.item.fields.get("isMicEnabled"), Some(&json!(false)));
    assert_eq!(msg.item.fields.get("state"), Some(&json!("CONNECTED")));
}
