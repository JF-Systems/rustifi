use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::protect::models::Nvr;

/// Fetch NVR / console information.
/// Endpoint: GET /proxy/protect/integration/v1/nvrs
///
/// The console returns a single NVR object (not an array).
#[derive(Debug, Clone, Default)]
pub struct GetNvr;

impl Endpoint for GetNvr {
    const PATH: &'static str = "nvrs";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Nvr;
}
