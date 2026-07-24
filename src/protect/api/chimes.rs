use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::protect::models::{Chime, ChimePatch};
use serde_json::Value;

/// Fetch all chimes.
/// Endpoint: GET /proxy/protect/integration/v1/chimes
#[derive(Debug, Clone, Default)]
pub struct GetChimes;

impl Endpoint for GetChimes {
    const PATH: &'static str = "chimes";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Vec<Chime>;
}

/// Fetch a single chime by ID.
/// Endpoint: GET /proxy/protect/integration/v1/chimes/{id}
#[derive(Debug, Clone)]
pub struct GetChime {
    pub chime_id: String,
}

impl GetChime {
    pub fn new(chime_id: impl Into<String>) -> Self {
        Self {
            chime_id: chime_id.into(),
        }
    }
}

impl Endpoint for GetChime {
    const PATH: &'static str = "chimes/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Chime;

    fn build_path(&self) -> String {
        format!("chimes/{}", self.chime_id)
    }
}

/// Partially update a chime's settings.
/// Endpoint: PATCH /proxy/protect/integration/v1/chimes/{id}
#[derive(Debug, Clone)]
pub struct PatchChime {
    pub chime_id: String,
    pub patch: ChimePatch,
}

impl PatchChime {
    pub fn new(chime_id: impl Into<String>, patch: ChimePatch) -> Self {
        Self {
            chime_id: chime_id.into(),
            patch,
        }
    }
}

impl Endpoint for PatchChime {
    const PATH: &'static str = "chimes/{id}";
    const METHOD: HttpMethod = HttpMethod::Patch;
    type Response = Chime;

    fn build_path(&self) -> String {
        format!("chimes/{}", self.chime_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.patch)?))
    }
}
