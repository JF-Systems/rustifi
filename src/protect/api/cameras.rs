use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::protect::models::{Camera, CameraPatch, RtspsQuality, RtspsStreams};
use serde_json::Value;

/// Fetch all cameras.
/// Endpoint: GET /proxy/protect/integration/v1/cameras
#[derive(Debug, Clone, Default)]
pub struct GetCameras;

impl Endpoint for GetCameras {
    const PATH: &'static str = "cameras";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Vec<Camera>;
}

/// Fetch a single camera by ID.
/// Endpoint: GET /proxy/protect/integration/v1/cameras/{id}
#[derive(Debug, Clone)]
pub struct GetCamera {
    pub camera_id: String,
}

impl GetCamera {
    pub fn new(camera_id: impl Into<String>) -> Self {
        Self {
            camera_id: camera_id.into(),
        }
    }
}

impl Endpoint for GetCamera {
    const PATH: &'static str = "cameras/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Camera;

    fn build_path(&self) -> String {
        format!("cameras/{}", super::encode_id(&self.camera_id))
    }
}

/// Partially update a camera's settings.
/// Endpoint: PATCH /proxy/protect/integration/v1/cameras/{id}
#[derive(Debug, Clone)]
pub struct PatchCamera {
    pub camera_id: String,
    pub patch: CameraPatch,
}

impl PatchCamera {
    pub fn new(camera_id: impl Into<String>, patch: CameraPatch) -> Self {
        Self {
            camera_id: camera_id.into(),
            patch,
        }
    }
}

impl Endpoint for PatchCamera {
    const PATH: &'static str = "cameras/{id}";
    const METHOD: HttpMethod = HttpMethod::Patch;
    type Response = Camera;

    fn build_path(&self) -> String {
        format!("cameras/{}", super::encode_id(&self.camera_id))
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.patch)?))
    }
}

/// Fetch the currently provisioned RTSPS stream URLs for a camera.
/// Endpoint: GET /proxy/protect/integration/v1/cameras/{id}/rtsps-stream
#[derive(Debug, Clone)]
pub struct GetRtspsStreams {
    pub camera_id: String,
}

impl GetRtspsStreams {
    pub fn new(camera_id: impl Into<String>) -> Self {
        Self {
            camera_id: camera_id.into(),
        }
    }
}

impl Endpoint for GetRtspsStreams {
    const PATH: &'static str = "cameras/{id}/rtsps-stream";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = RtspsStreams;

    fn build_path(&self) -> String {
        format!("cameras/{}/rtsps-stream", super::encode_id(&self.camera_id))
    }
}

/// Provision RTSPS streams for the given qualities.
/// Endpoint: POST /proxy/protect/integration/v1/cameras/{id}/rtsps-stream
#[derive(Debug, Clone)]
pub struct CreateRtspsStream {
    pub camera_id: String,
    pub qualities: Vec<RtspsQuality>,
}

impl CreateRtspsStream {
    pub fn new(camera_id: impl Into<String>, qualities: Vec<RtspsQuality>) -> Self {
        Self {
            camera_id: camera_id.into(),
            qualities,
        }
    }
}

impl Endpoint for CreateRtspsStream {
    const PATH: &'static str = "cameras/{id}/rtsps-stream";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = RtspsStreams;

    fn build_path(&self) -> String {
        format!("cameras/{}/rtsps-stream", super::encode_id(&self.camera_id))
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::json!({ "qualities": self.qualities })))
    }
}

/// Remove provisioned RTSPS streams for the given qualities.
/// Endpoint: DELETE /proxy/protect/integration/v1/cameras/{id}/rtsps-stream?qualities=...
#[derive(Debug, Clone)]
pub struct DeleteRtspsStream {
    pub camera_id: String,
    pub qualities: Vec<RtspsQuality>,
}

impl DeleteRtspsStream {
    pub fn new(camera_id: impl Into<String>, qualities: Vec<RtspsQuality>) -> Self {
        Self {
            camera_id: camera_id.into(),
            qualities,
        }
    }
}

impl Endpoint for DeleteRtspsStream {
    const PATH: &'static str = "cameras/{id}/rtsps-stream";
    const METHOD: HttpMethod = HttpMethod::Delete;
    // The console returns an empty body on success.
    type Response = Option<serde_json::Value>;

    fn build_path(&self) -> String {
        format!("cameras/{}/rtsps-stream", super::encode_id(&self.camera_id))
    }

    fn query_params(&self) -> Vec<(&'static str, String)> {
        self.qualities
            .iter()
            .map(|q| ("qualities", q.as_str().to_string()))
            .collect()
    }
}
