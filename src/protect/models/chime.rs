use super::common::{ModelKey, ProtectDeviceState};
use crate::models::common::MacAddress;
use serde::{Deserialize, Serialize};

/// A Protect chime as returned by `GET /chimes` and `GET /chimes/{id}`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Chime {
    pub id: String,
    #[serde(default)]
    pub model_key: ModelKey,
    #[serde(default)]
    pub state: ProtectDeviceState,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub mac: Option<MacAddress>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub firmware_version: Option<String>,
    #[serde(default)]
    pub volume: Option<u8>,
    #[serde(default)]
    pub camera_ids: Option<Vec<String>>,
    #[serde(default)]
    pub ring_settings: Option<Vec<RingSetting>>,
}

/// Per-camera ring configuration for a chime.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RingSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat_times: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_no: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<u8>,
}

/// Partial update body for `PATCH /chimes/{id}`.
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChimePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_settings: Option<Vec<RingSetting>>,
}
