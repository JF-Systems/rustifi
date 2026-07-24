//! UniFi Protect Integration API support.
//!
//! This module wraps the official Protect Integration API (Protect >= 5.3),
//! available at `https://{console}/proxy/protect/integration/v1`. It uses the
//! same `X-API-Key` authentication as the Network Integration API.
//!
//! Enable with the `protect` cargo feature:
//!
//! ```toml
//! rustifi = { version = "2", features = ["protect"] }
//! ```

pub mod api;
pub mod client;
pub mod models;
pub mod ws;

pub use client::{ProtectClient, PROTECT_BASE_PATH};

pub mod prelude {
    pub use crate::protect::client::ProtectClient;
    pub use crate::protect::models::{
        Camera, CameraPatch, Chime, Light, ModelKey, Nvr, ProtectDeviceState, ProtectEvent,
        ProtectEventType, RtspsQuality, RtspsStreams, Sensor, SmartDetectType, Viewer, WsAction,
        WsMessage,
    };
    pub use crate::protect::ws::{DeviceUpdateStream, ProtectEventStream};
}
