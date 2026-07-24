use super::common::{ModelKey, ProtectDeviceState};
use crate::models::common::MacAddress;
use serde::{Deserialize, Serialize};

/// A Protect sensor as returned by `GET /sensors` and `GET /sensors/{id}`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sensor {
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
    pub mount_type: Option<String>,
    #[serde(default)]
    pub is_opened: Option<bool>,
    #[serde(default)]
    pub is_motion_detected: Option<bool>,
    #[serde(default)]
    pub battery_status: Option<BatteryStatus>,
    #[serde(default)]
    pub stats: Option<SensorStats>,
    #[serde(default)]
    pub alarm_settings: Option<SensorAlarmSettings>,
    #[serde(default)]
    pub light_settings: Option<SensorSettings>,
    #[serde(default)]
    pub motion_settings: Option<SensorSettings>,
    #[serde(default)]
    pub temperature_settings: Option<SensorSettings>,
    #[serde(default)]
    pub humidity_settings: Option<SensorSettings>,
}

/// Battery level and charge state.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BatteryStatus {
    #[serde(default)]
    pub percentage: Option<i64>,
    #[serde(default)]
    pub is_low: Option<bool>,
}

/// Latest environmental readings from the sensor.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensorStats {
    #[serde(default)]
    pub light: Option<SensorReading>,
    #[serde(default)]
    pub temperature: Option<SensorReading>,
    #[serde(default)]
    pub humidity: Option<SensorReading>,
}

/// A single reading with its value and status.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensorReading {
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub status: Option<String>,
}

/// Alarm sound detection settings.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SensorAlarmSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

/// Generic enable/threshold settings block shared by sensor capabilities.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensorSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_threshold: Option<f64>,
}

/// Partial update body for `PATCH /sensors/{id}`.
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensorPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_settings: Option<SensorAlarmSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_settings: Option<SensorSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_settings: Option<SensorSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_settings: Option<SensorSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity_settings: Option<SensorSettings>,
}
