pub mod api;
#[cfg(feature = "unifi")]
pub mod client;
pub mod error;
pub mod models;
#[cfg(feature = "unifi")]
pub mod pagination;
#[cfg(feature = "protect")]
pub mod protect;
#[cfg(feature = "unifi")]
pub mod response;
#[cfg(feature = "unifi")]
pub mod stats;
#[cfg(any(feature = "unifi", feature = "protect"))]
pub(crate) mod transport;
#[cfg(feature = "unifi")]
pub mod wrappers;

#[cfg(feature = "unifi")]
pub use client::{UnifiClient, REMOTE_API_URL};
pub use error::{Error, Result};
#[cfg(feature = "unifi")]
pub use pagination::DEFAULT_PAGE_SIZE;
#[cfg(feature = "protect")]
pub use protect::ProtectClient;
#[cfg(feature = "unifi")]
pub use stats::{aggregate_clients_by_device, get_device_client_stats, DeviceClientStats};
#[cfg(feature = "unifi")]
pub use wrappers::DeviceWithInfo;

pub mod prelude {
    #[cfg(feature = "unifi")]
    pub use crate::api::networks::{Network, NetworkRequest};
    pub use crate::api::Endpoint;
    #[cfg(feature = "unifi")]
    pub use crate::client::UnifiClient;
    pub use crate::error::{Error, Result};
    #[cfg(feature = "unifi")]
    pub use crate::models::{
        APModel, Client, Device, DeviceType, FirewallAction, FirewallPolicy, FirewallZone, Site,
        Voucher, WifiBroadcast, WifiSecurity,
    };
    #[cfg(feature = "protect")]
    pub use crate::protect::prelude::*;
    #[cfg(feature = "unifi")]
    pub use crate::stats::DeviceClientStats;
    #[cfg(feature = "unifi")]
    pub use crate::wrappers::DeviceWithInfo;
}
