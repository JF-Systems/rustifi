use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::protect::models::{Viewer, ViewerPatch};
use serde_json::Value;

/// Fetch all viewers.
/// Endpoint: GET /proxy/protect/integration/v1/viewers
#[derive(Debug, Clone, Default)]
pub struct GetViewers;

impl Endpoint for GetViewers {
    const PATH: &'static str = "viewers";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Vec<Viewer>;
}

/// Fetch a single viewer by ID.
/// Endpoint: GET /proxy/protect/integration/v1/viewers/{id}
#[derive(Debug, Clone)]
pub struct GetViewer {
    pub viewer_id: String,
}

impl GetViewer {
    pub fn new(viewer_id: impl Into<String>) -> Self {
        Self {
            viewer_id: viewer_id.into(),
        }
    }
}

impl Endpoint for GetViewer {
    const PATH: &'static str = "viewers/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Viewer;

    fn build_path(&self) -> String {
        format!("viewers/{}", self.viewer_id)
    }
}

/// Partially update a viewer's settings.
/// Endpoint: PATCH /proxy/protect/integration/v1/viewers/{id}
#[derive(Debug, Clone)]
pub struct PatchViewer {
    pub viewer_id: String,
    pub patch: ViewerPatch,
}

impl PatchViewer {
    pub fn new(viewer_id: impl Into<String>, patch: ViewerPatch) -> Self {
        Self {
            viewer_id: viewer_id.into(),
            patch,
        }
    }
}

impl Endpoint for PatchViewer {
    const PATH: &'static str = "viewers/{id}";
    const METHOD: HttpMethod = HttpMethod::Patch;
    type Response = Viewer;

    fn build_path(&self) -> String {
        format!("viewers/{}", self.viewer_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.patch)?))
    }
}
