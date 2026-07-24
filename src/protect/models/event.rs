use super::common::ModelKey;
use serde::Deserialize;

/// Envelope for messages received on the Protect WebSocket subscriptions.
///
/// Both `/subscribe/events` and `/subscribe/devices` deliver frames shaped as
/// `{"type": "add" | "update", "item": {...}}`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WsMessage<T> {
    #[serde(rename = "type")]
    pub action: WsAction,
    pub item: T,
}

/// The action discriminator on a WebSocket frame.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum WsAction {
    Add,
    Update,
    Remove,
    #[serde(other)]
    #[default]
    Unknown,
}

/// A motion, smart-detection, doorbell, or connectivity event.
///
/// Delivered via the `/subscribe/events` WebSocket. Events arrive as an `add`
/// when they start and an `update` (with `end` set) when they finish.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProtectEvent {
    pub id: String,
    #[serde(default)]
    pub model_key: ModelKey,
    #[serde(default, rename = "type")]
    pub event_type: ProtectEventType,
    /// Event start, epoch milliseconds.
    #[serde(default)]
    pub start: i64,
    /// Event end, epoch milliseconds. `None` while the event is ongoing.
    #[serde(default)]
    pub end: Option<i64>,
    /// ID of the device that produced the event.
    #[serde(default)]
    pub device: String,
    #[serde(default)]
    pub smart_detect_types: Vec<SmartDetectType>,
}

/// The kind of a Protect event.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ProtectEventType {
    Motion,
    SmartDetectZone,
    SmartDetectLine,
    /// Doorbell ring.
    Doorbell,
    Disconnect,
    #[serde(other)]
    #[default]
    Unknown,
}

/// Object classes recognized by smart detection.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum SmartDetectType {
    Person,
    Vehicle,
    Animal,
    Face,
    Package,
    LicensePlate,
    #[serde(other)]
    #[default]
    Unknown,
}

/// A partial device-state patch from the `/subscribe/devices` WebSocket.
///
/// Frames carry heterogeneous partial updates for any device type, so only the
/// identifying fields are typed; the remaining fields are kept as raw JSON in
/// [`Self::fields`].
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUpdate {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub model_key: ModelKey,
    #[serde(flatten)]
    pub fields: serde_json::Map<String, serde_json::Value>,
}
