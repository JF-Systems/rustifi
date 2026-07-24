use super::common::{ModelKey, ProtectDeviceState};
use crate::models::common::MacAddress;
use serde::{Deserialize, Serialize};

/// A Protect camera as returned by `GET /cameras` and `GET /cameras/{id}`.
///
/// Fields that are not guaranteed to be present on every camera model or
/// Protect release are `Option` — Ubiquiti adds fields per release.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
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
    #[serde(default, rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub firmware_version: Option<String>,
    #[serde(default)]
    pub is_mic_enabled: Option<bool>,
    #[serde(default)]
    pub mic_volume: Option<u8>,
    #[serde(default)]
    pub video_mode: Option<String>,
    #[serde(default)]
    pub hdr_type: Option<String>,
    #[serde(default)]
    pub osd_settings: Option<OsdSettings>,
    #[serde(default)]
    pub led_settings: Option<LedSettings>,
    #[serde(default)]
    pub lcd_message: Option<LcdMessage>,
    #[serde(default)]
    pub smart_detect_settings: Option<SmartDetectSettings>,
    #[serde(default)]
    pub recording_settings: Option<RecordingSettings>,
    #[serde(default)]
    pub feature_flags: Option<CameraFeatureFlags>,
    #[serde(default)]
    pub is_third_party_camera: Option<bool>,
    #[serde(default)]
    pub active_patrol_slot: Option<i64>,
}

/// On-screen display settings for a camera.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OsdSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_name_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_date_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_logo_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_debug_enabled: Option<bool>,
}

/// Status LED settings for a camera.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LedSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

/// LCD message shown on doorbell cameras.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LcdMessage {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<i64>,
}

/// Smart detection configuration for a camera.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SmartDetectSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_tracking_object_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_types: Option<Vec<String>>,
}

/// Recording configuration for a camera.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordingSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_padding_secs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_padding_secs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_duration_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_motion_event_delay: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppress_illumination_surge: Option<bool>,
}

/// Capabilities reported by a camera.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CameraFeatureFlags {
    #[serde(default)]
    pub has_hdr: Option<bool>,
    #[serde(default)]
    pub has_mic: Option<bool>,
    #[serde(default)]
    pub has_led_status: Option<bool>,
    #[serde(default)]
    pub has_speaker: Option<bool>,
    #[serde(default)]
    pub has_chime: Option<bool>,
    #[serde(default)]
    pub has_smart_detect: Option<bool>,
    #[serde(default)]
    pub smart_detect_types: Option<Vec<String>>,
    #[serde(default)]
    pub smart_detect_audio_types: Option<Vec<String>>,
    #[serde(default)]
    pub video_modes: Option<Vec<String>>,
    #[serde(default)]
    pub hdr_types: Option<Vec<String>>,
    #[serde(default)]
    pub is_ptz: Option<bool>,
}

/// Partial update body for `PATCH /cameras/{id}`.
///
/// Only fields set to `Some` are serialized and sent.
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CameraPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osd_settings: Option<OsdSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub led_settings: Option<LedSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_message: Option<LcdMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mic_volume: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_detect_settings: Option<SmartDetectSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_settings: Option<RecordingSettings>,
}

/// RTSPS stream URLs per quality, from the `/cameras/{id}/rtsps-stream` endpoints.
///
/// A quality is `None` when no stream is provisioned for it.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RtspsStreams {
    #[serde(default)]
    pub high: Option<String>,
    #[serde(default)]
    pub medium: Option<String>,
    #[serde(default)]
    pub low: Option<String>,
    #[serde(default)]
    pub package: Option<String>,
}

/// Stream quality selector for RTSPS stream management.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum RtspsQuality {
    High,
    Medium,
    Low,
    Package,
}

impl RtspsQuality {
    pub fn as_str(&self) -> &'static str {
        match self {
            RtspsQuality::High => "high",
            RtspsQuality::Medium => "medium",
            RtspsQuality::Low => "low",
            RtspsQuality::Package => "package",
        }
    }
}
