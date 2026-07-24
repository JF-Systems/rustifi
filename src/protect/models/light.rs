use super::common::{ModelKey, ProtectDeviceState};
use crate::models::common::MacAddress;
use serde::{Deserialize, Serialize};

/// A Protect floodlight as returned by `GET /lights` and `GET /lights/{id}`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Light {
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
    pub is_light_on: Option<bool>,
    #[serde(default)]
    pub is_pir_motion_detected: Option<bool>,
    #[serde(default)]
    pub is_locating: Option<bool>,
    #[serde(default)]
    pub camera_id: Option<String>,
    #[serde(default)]
    pub light_mode_settings: Option<LightModeSettings>,
    #[serde(default)]
    pub light_device_settings: Option<LightDeviceSettings>,
}

/// When and how the light activates.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LightModeSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_at: Option<String>,
}

/// Hardware settings for the light.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LightDeviceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_indicator_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub led_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pir_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pir_sensitivity: Option<i64>,
}

/// Partial update body for `PATCH /lights/{id}`.
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LightPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_light_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_mode_settings: Option<LightModeSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_device_settings: Option<LightDeviceSettings>,
}
