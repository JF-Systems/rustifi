use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::protect::models::{Sensor, SensorPatch};
use serde_json::Value;

/// Fetch all sensors.
/// Endpoint: GET /proxy/protect/integration/v1/sensors
#[derive(Debug, Clone, Default)]
pub struct GetSensors;

impl Endpoint for GetSensors {
    const PATH: &'static str = "sensors";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Vec<Sensor>;
}

/// Fetch a single sensor by ID.
/// Endpoint: GET /proxy/protect/integration/v1/sensors/{id}
#[derive(Debug, Clone)]
pub struct GetSensor {
    pub sensor_id: String,
}

impl GetSensor {
    pub fn new(sensor_id: impl Into<String>) -> Self {
        Self {
            sensor_id: sensor_id.into(),
        }
    }
}

impl Endpoint for GetSensor {
    const PATH: &'static str = "sensors/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = Sensor;

    fn build_path(&self) -> String {
        format!("sensors/{}", self.sensor_id)
    }
}

/// Partially update a sensor's settings.
/// Endpoint: PATCH /proxy/protect/integration/v1/sensors/{id}
#[derive(Debug, Clone)]
pub struct PatchSensor {
    pub sensor_id: String,
    pub patch: SensorPatch,
}

impl PatchSensor {
    pub fn new(sensor_id: impl Into<String>, patch: SensorPatch) -> Self {
        Self {
            sensor_id: sensor_id.into(),
            patch,
        }
    }
}

impl Endpoint for PatchSensor {
    const PATH: &'static str = "sensors/{id}";
    const METHOD: HttpMethod = HttpMethod::Patch;
    type Response = Sensor;

    fn build_path(&self) -> String {
        format!("sensors/{}", self.sensor_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.patch)?))
    }
}
