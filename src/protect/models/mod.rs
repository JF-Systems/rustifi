pub mod camera;
pub mod chime;
pub mod common;
pub mod event;
pub mod light;
pub mod nvr;
pub mod sensor;
pub mod viewer;

pub use camera::{
    Camera, CameraFeatureFlags, CameraPatch, LcdMessage, LedSettings, OsdSettings,
    RecordingSettings, RtspsQuality, RtspsStreams, SmartDetectSettings,
};
pub use chime::{Chime, ChimePatch, RingSetting};
pub use common::{ModelKey, ProtectDeviceState, TimestampMillis};
pub use event::{
    DeviceUpdate, ProtectEvent, ProtectEventType, SmartDetectType, WsAction, WsMessage,
};
pub use light::{Light, LightDeviceSettings, LightModeSettings, LightPatch};
pub use nvr::{MetaInfo, Nvr};
pub use sensor::{
    BatteryStatus, Sensor, SensorAlarmSettings, SensorPatch, SensorReading, SensorSettings,
    SensorStats,
};
pub use viewer::{Viewer, ViewerPatch};
