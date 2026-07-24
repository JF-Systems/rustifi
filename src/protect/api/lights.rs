use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::protect::models::{Light, LightPatch};
use serde_json::Value;

/// Fetch all lights.
/// Endpoint: GET /proxy/protect/integration/v1/lights
#[derive(Debug, Clone, Default)]
pub struct GetLights;

impl Endpoint for GetLights {
    const PATH: &'static str = "lights";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Vec<Light>;
}

/// Fetch a single light by ID.
/// Endpoint: GET /proxy/protect/integration/v1/lights/{id}
#[derive(Debug, Clone)]
pub struct GetLight {
    pub light_id: String,
}

impl GetLight {
    pub fn new(light_id: impl Into<String>) -> Self {
        Self {
            light_id: light_id.into(),
        }
    }
}

impl Endpoint for GetLight {
    const PATH: &'static str = "lights/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Light;

    fn build_path(&self) -> String {
        format!("lights/{}", self.light_id)
    }
}

/// Partially update a light's settings.
/// Endpoint: PATCH /proxy/protect/integration/v1/lights/{id}
#[derive(Debug, Clone)]
pub struct PatchLight {
    pub light_id: String,
    pub patch: LightPatch,
}

impl PatchLight {
    pub fn new(light_id: impl Into<String>, patch: LightPatch) -> Self {
        Self {
            light_id: light_id.into(),
            patch,
        }
    }
}

impl Endpoint for PatchLight {
    const PATH: &'static str = "lights/{id}";
    const METHOD: HttpMethod = HttpMethod::Patch;
    type Response = Light;

    fn build_path(&self) -> String {
        format!("lights/{}", self.light_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.patch)?))
    }
}
