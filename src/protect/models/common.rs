use serde::Deserialize;

/// Connection state reported for every Protect device.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProtectDeviceState {
    Connected,
    Connecting,
    Disconnected,
    #[serde(other)]
    #[default]
    Unknown,
}

/// The `modelKey` discriminator present on every Protect object.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModelKey {
    Camera,
    Light,
    Sensor,
    Chime,
    Viewer,
    Nvr,
    Doorlock,
    Event,
    #[serde(other)]
    #[default]
    Unknown,
}

/// A millisecond-precision Unix timestamp, as used by Protect events.
///
/// Distinct from [`crate::models::common::Timestamp`], which holds seconds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimestampMillis(pub i64);

impl TimestampMillis {
    pub fn as_millis(&self) -> i64 {
        self.0
    }

    pub fn as_secs(&self) -> i64 {
        self.0 / 1000
    }
}

impl std::fmt::Display for TimestampMillis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for TimestampMillis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = i64::deserialize(deserializer)?;
        Ok(TimestampMillis(n))
    }
}
