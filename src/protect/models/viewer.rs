use super::common::{ModelKey, ProtectDeviceState};
use crate::models::common::MacAddress;
use serde::{Deserialize, Serialize};

/// A Protect viewer (ViewPort) as returned by `GET /viewers` and `GET /viewers/{id}`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
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
    /// ID of the liveview currently shown on this viewer.
    #[serde(default)]
    pub liveview: Option<String>,
}

/// Partial update body for `PATCH /viewers/{id}`.
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ViewerPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveview: Option<String>,
}
