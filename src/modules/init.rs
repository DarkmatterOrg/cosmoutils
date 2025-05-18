#[cfg(feature = "systemd")]
pub mod systemd;
#[cfg(feature = "dinit")]
pub mod dinit;

#[cfg(feature = "runit")]
pub mod runit;

#[cfg(feature = "openrc")]
pub mod openrc;

const DISABLE_MESSAGE: &str = "Disabled";
const ENABLE_MESSAGE: &str = "Enabled";
const FAILED_ENABLE_MESSAGE: &str = "Failed to enable";
const FAILED_DISABLE_MESSAGE: &str = "Failed to disable";
const SET_MESSAGE: &str = "Set target to";
const FAILED_SET_MESSAGE: &str = "Failed to set target to";






