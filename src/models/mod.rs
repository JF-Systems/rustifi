#[cfg(feature = "unifi")]
pub mod access_point;
#[cfg(feature = "unifi")]
pub mod client;
pub mod common;
#[cfg(feature = "unifi")]
pub mod device;
#[cfg(feature = "unifi")]
pub mod device_details;
#[cfg(feature = "unifi")]
pub mod device_statistics;
#[cfg(feature = "unifi")]
pub mod firewall;
#[cfg(feature = "unifi")]
pub mod site;
#[cfg(feature = "unifi")]
pub mod site_device;
#[cfg(feature = "unifi")]
pub mod voucher;
#[cfg(feature = "unifi")]
pub mod wifi;

#[cfg(feature = "unifi")]
pub use access_point::APModel;
#[cfg(feature = "unifi")]
pub use client::{AccessType, Client, ClientAccess, ClientType};
pub use common::{IpAddress, MacAddress, Timestamp};
#[cfg(feature = "unifi")]
pub use device::{Device, DeviceType};
#[cfg(feature = "unifi")]
pub use device_details::{
    AccessPointFeature, DeviceDetails, DeviceFeatures, DeviceUplink, InterfaceState,
    PhysicalInterfaces, PoE, Port, PortConnector, Radio, SwitchingFeature, WirelessStandard,
};
#[cfg(feature = "unifi")]
pub use device_statistics::{
    DeviceStatistics, RadioStatistics, StatisticsInterfaces, StatisticsUplink,
};
#[cfg(feature = "unifi")]
pub use firewall::{FirewallAction, FirewallPolicy, FirewallZone};
#[cfg(feature = "unifi")]
pub use site::Site;
#[cfg(feature = "unifi")]
pub use site_device::{DeviceFeature, DeviceInterface, DeviceState, SiteDevice};
#[cfg(feature = "unifi")]
pub use voucher::Voucher;
#[cfg(feature = "unifi")]
pub use wifi::{WifiBroadcast, WifiSecurity};
