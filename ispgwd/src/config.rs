//! ISP gateway daemon Configuration

use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
/// Top Level Ispgwd Configuration Object
pub struct IspgwdConfig {
    #[serde(default)]
    /// ISP configs
    pub isp_configs: Vec<IspConfig>,

    /// Simulate operations
    pub simulation: bool,

    /// Standard Polling duration
    pub poll_duration_ms: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
/// ISP Configuration
pub struct IspConfig {
    /// ISP name
    pub name: String,

    /// ISP interface
    pub interface: String,

    /// ISP priority
    pub priority: u32,
}
