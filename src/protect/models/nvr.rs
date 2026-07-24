use super::common::ModelKey;
use crate::models::common::MacAddress;
use serde::Deserialize;

/// The Protect NVR / console as returned by `GET /nvrs`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Nvr {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub model_key: ModelKey,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub mac: Option<MacAddress>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub firmware_version: Option<String>,
    #[serde(default)]
    pub uptime: Option<i64>,
    #[serde(default)]
    pub hardware_platform: Option<String>,
    #[serde(default)]
    pub hardware_id: Option<String>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub ports: Option<serde_json::Value>,
    #[serde(default)]
    pub storage_stats: Option<serde_json::Value>,
}

/// Application version info from `GET /meta/info`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo {
    #[serde(default)]
    pub application_version: String,
}
