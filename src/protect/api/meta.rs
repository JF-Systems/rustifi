use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::protect::models::MetaInfo;

/// Fetch Protect application version info.
/// Endpoint: GET /proxy/protect/integration/v1/meta/info
#[derive(Debug, Clone, Default)]
pub struct GetMetaInfo;

impl Endpoint for GetMetaInfo {
    const PATH: &'static str = "meta/info";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = MetaInfo;
}
