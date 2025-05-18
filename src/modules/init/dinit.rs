use std::process::Command;
use crate::modules::init::{DISABLE_MESSAGE, ENABLE_MESSAGE, FAILED_DISABLE_MESSAGE, FAILED_ENABLE_MESSAGE};
use crate::modules::msg::{error, success};

/// Enable a Dinit service by running dinitctl enable
///
/// ## Example (system-service)
/// ```rust
/// use cosmoutils::modules::init::dinit::enable_dinit_service;
/// enable_dinit_service("NetworkManager")
/// ```
/// ## Example (user-service)
/// ```rust
/// use cosmoutils::modules::init::dinit::enable_dinit_service;
/// enable_dinit_service("pipewire")
/// ```
///
pub fn enable_dinit_service(service_name: &str) {
    let mut command = Command::new("dinitctl")
        .args(["enable", service_name])
        .spawn()
        .expect("Failed to run");
    let msg = format!("{} {}", ENABLE_MESSAGE, service_name);
    let failed_msg = format!("{} {}", FAILED_ENABLE_MESSAGE, service_name);

    let status = command.wait().expect("Failed to run the command.");

    if !status.success() {
        error(&failed_msg)
    } else {
        success(&msg);
    }
}

/// Disable a Dinit service by running dinitctl disable
///
/// ## Example (system-service)
/// ```rust
/// use cosmoutils::modules::init::dinit::disable_dinit_service;
/// disable_dinit_service("NetworkManager")
/// ```
/// ## Example (user-service)
/// ```rust
/// use cosmoutils::modules::init::dinit::disable_dinit_service;
/// disable_dinit_service("pipewire")
/// ```
///
pub fn disable_dinit_service(service_name: &str) {
    let mut command = Command::new("dinitctl")
        .args(["disable", service_name])
        .spawn()
        .expect("Failed to run");
    let msg = format!("{} {}", DISABLE_MESSAGE, service_name);
    let failed_msg = format!("{} {}", FAILED_DISABLE_MESSAGE, service_name);

    let status = command.wait().expect("Failed to run the command.");

    if !status.success() {
        error(&failed_msg)
    } else {
        success(&msg);
    }
}