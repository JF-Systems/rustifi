use crate::api::endpoint::Endpoint;
use crate::error::{Error, Result};
use crate::protect::api::{
    cameras::{
        CreateRtspsStream, DeleteRtspsStream, GetCamera, GetCameras, GetRtspsStreams, PatchCamera,
    },
    chimes::{GetChime, GetChimes, PatchChime},
    lights::{GetLight, GetLights, PatchLight},
    meta::GetMetaInfo,
    nvr::GetNvr,
    sensors::{GetSensor, GetSensors, PatchSensor},
    viewers::{GetViewer, GetViewers, PatchViewer},
};
use crate::protect::models::{
    Camera, CameraPatch, Chime, ChimePatch, Light, LightPatch, MetaInfo, Nvr, RtspsQuality,
    RtspsStreams, Sensor, SensorPatch, Viewer, ViewerPatch,
};
use crate::transport;
use reqwest::Client;

/// Base path of the official Protect Integration API on a local console.
pub const PROTECT_BASE_PATH: &str = "proxy/protect/integration/v1";

/// Client for the official UniFi Protect Integration API.
///
/// Authenticates with an `X-API-Key` header. Generate a key in the Protect
/// app under Settings -> Control Plane -> Integrations.
///
/// # Example
/// ```no_run
/// use rustifi::protect::ProtectClient;
///
/// # async fn run() -> rustifi::Result<()> {
/// // Local consoles usually have self-signed certs; see `new_insecure`.
/// let client = ProtectClient::new("https://protect.example.com", "your-api-key")?;
/// let cameras = client.cameras().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct ProtectClient {
    http: Client,
    base_url: String,
    api_key: String,
    accept_invalid_certs: bool,
}

impl ProtectClient {
    /// Create a new Protect client with strict TLS validation.
    ///
    /// # Errors
    /// Returns an error if the API key contains invalid HTTP header characters.
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        Self::build(base_url, api_key, false)
    }

    /// Create a new Protect client that accepts invalid/self-signed TLS certificates.
    ///
    /// # Security Warning
    /// **This disables TLS certificate validation.** Only use this for local
    /// consoles with self-signed certificates on trusted networks. Using this
    /// over untrusted networks exposes you to man-in-the-middle attacks.
    ///
    /// # Errors
    /// Returns an error if the API key contains invalid HTTP header characters.
    pub fn new_insecure(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        Self::build(base_url, api_key, true)
    }

    fn build(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        accept_invalid_certs: bool,
    ) -> Result<Self> {
        // Validate the API key can be parsed as a header value early
        let api_key = api_key.into();
        let _: reqwest::header::HeaderValue = api_key.parse()?;

        let http = transport::build_http_client(accept_invalid_certs, false)?;

        Ok(Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key,
            accept_invalid_certs,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub(crate) fn accepts_invalid_certs(&self) -> bool {
        self.accept_invalid_certs
    }

    /// Execute a request for an endpoint instance.
    /// Use this when the endpoint has dynamic path parameters.
    pub async fn execute<E>(&self, endpoint: &E) -> Result<E::Response>
    where
        E: Endpoint,
        E::Response: for<'a> serde::Deserialize<'a>,
    {
        let url_prefix = format!("{}/{}", self.base_url, PROTECT_BASE_PATH);
        transport::execute_endpoint(&self.http, &url_prefix, Some(&self.api_key), endpoint).await
    }

    /// Execute a request for endpoints without dynamic path parameters.
    /// For endpoints with path parameters, use `execute()` instead.
    pub async fn request<E>(&self) -> Result<E::Response>
    where
        E: Endpoint + Default,
        E::Response: for<'a> serde::Deserialize<'a>,
    {
        self.execute(&E::default()).await
    }

    /// Fetch a JPEG snapshot from a camera.
    /// Endpoint: GET /proxy/protect/integration/v1/cameras/{id}/snapshot
    ///
    /// Returns the raw JPEG bytes. This is a dedicated method because the
    /// [`Endpoint`] trait models JSON responses only; if more binary endpoints
    /// are added later, a `RawEndpoint` trait may replace this.
    pub async fn snapshot(&self, camera_id: &str, high_quality: bool) -> Result<Vec<u8>> {
        let url = format!(
            "{}/{}/cameras/{}/snapshot",
            self.base_url,
            PROTECT_BASE_PATH,
            crate::protect::api::encode_id(camera_id)
        );

        let response = self
            .http
            .get(&url)
            .header(
                "X-API-Key",
                self.api_key.parse::<reqwest::header::HeaderValue>()?,
            )
            .query(&[("highQuality", high_quality.to_string())])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status, body });
        }

        Ok(response.bytes().await?.to_vec())
    }

    /// Fetch Protect application version info.
    pub async fn meta_info(&self) -> Result<MetaInfo> {
        self.execute(&GetMetaInfo).await
    }

    /// Fetch all cameras.
    pub async fn cameras(&self) -> Result<Vec<Camera>> {
        self.execute(&GetCameras).await
    }

    /// Fetch a single camera by ID.
    pub async fn camera(&self, id: &str) -> Result<Camera> {
        self.execute(&GetCamera::new(id)).await
    }

    /// Partially update a camera's settings.
    pub async fn patch_camera(&self, id: &str, patch: CameraPatch) -> Result<Camera> {
        self.execute(&PatchCamera::new(id, patch)).await
    }

    /// Fetch the currently provisioned RTSPS stream URLs for a camera.
    pub async fn rtsps_streams(&self, camera_id: &str) -> Result<RtspsStreams> {
        self.execute(&GetRtspsStreams::new(camera_id)).await
    }

    /// Provision RTSPS streams for the given qualities.
    pub async fn create_rtsps_streams(
        &self,
        camera_id: &str,
        qualities: Vec<RtspsQuality>,
    ) -> Result<RtspsStreams> {
        self.execute(&CreateRtspsStream::new(camera_id, qualities))
            .await
    }

    /// Remove provisioned RTSPS streams for the given qualities.
    pub async fn delete_rtsps_streams(
        &self,
        camera_id: &str,
        qualities: Vec<RtspsQuality>,
    ) -> Result<()> {
        self.execute(&DeleteRtspsStream::new(camera_id, qualities))
            .await?;
        Ok(())
    }

    /// Fetch all lights.
    pub async fn lights(&self) -> Result<Vec<Light>> {
        self.execute(&GetLights).await
    }

    /// Fetch a single light by ID.
    pub async fn light(&self, id: &str) -> Result<Light> {
        self.execute(&GetLight::new(id)).await
    }

    /// Partially update a light's settings.
    pub async fn patch_light(&self, id: &str, patch: LightPatch) -> Result<Light> {
        self.execute(&PatchLight::new(id, patch)).await
    }

    /// Fetch all sensors.
    pub async fn sensors(&self) -> Result<Vec<Sensor>> {
        self.execute(&GetSensors).await
    }

    /// Fetch a single sensor by ID.
    pub async fn sensor(&self, id: &str) -> Result<Sensor> {
        self.execute(&GetSensor::new(id)).await
    }

    /// Partially update a sensor's settings.
    pub async fn patch_sensor(&self, id: &str, patch: SensorPatch) -> Result<Sensor> {
        self.execute(&PatchSensor::new(id, patch)).await
    }

    /// Fetch all chimes.
    pub async fn chimes(&self) -> Result<Vec<Chime>> {
        self.execute(&GetChimes).await
    }

    /// Fetch a single chime by ID.
    pub async fn chime(&self, id: &str) -> Result<Chime> {
        self.execute(&GetChime::new(id)).await
    }

    /// Partially update a chime's settings.
    pub async fn patch_chime(&self, id: &str, patch: ChimePatch) -> Result<Chime> {
        self.execute(&PatchChime::new(id, patch)).await
    }

    /// Fetch all viewers.
    pub async fn viewers(&self) -> Result<Vec<Viewer>> {
        self.execute(&GetViewers).await
    }

    /// Fetch a single viewer by ID.
    pub async fn viewer(&self, id: &str) -> Result<Viewer> {
        self.execute(&GetViewer::new(id)).await
    }

    /// Partially update a viewer's settings.
    pub async fn patch_viewer(&self, id: &str, patch: ViewerPatch) -> Result<Viewer> {
        self.execute(&PatchViewer::new(id, patch)).await
    }

    /// Fetch NVR / console information.
    pub async fn nvr(&self) -> Result<Nvr> {
        self.execute(&GetNvr).await
    }
}
