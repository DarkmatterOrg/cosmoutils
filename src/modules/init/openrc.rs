use std::path::Path;
use std::process::Command;
use crate::modules::init::{DISABLE_MESSAGE, ENABLE_MESSAGE, FAILED_DISABLE_MESSAGE, FAILED_ENABLE_MESSAGE};
use crate::modules::msg::{error, success};

/// Enable a openRC service by running rc-update add or rc-update --user add
///
/// ## Example (system-service)
/// ```rust
/// use cosmoutils::modules::init::openrc::enable_openrc_service;
/// enable_openrc_service("NetworkManager", false)
/// ```
/// ## Example (user-service)
/// ```rust
/// use cosmoutils::modules::init::openrc::enable_openrc_service;
/// enable_openrc_service("pipewire", true)
/// ```
///
pub fn enable_openrc_service(service_name: &str, user_mode: bool) {
    let user = std::env::var("USER").unwrap();

    if user_mode && Path::new(&format!("/etc/init.d/{}.{}", user, user)).exists() {
        let mut command = Command::new("rc-update")
            .args(["--user", "add", service_name])
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
    } else {
        let mut command = Command::new("rc-update")
            .args(["add", service_name])
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
}

/// Disable a openRC service by running rc-update del or rc-update --user del
///
/// ## Example (system-service)
/// ```rust
/// use cosmoutils::modules::init::openrc::disable_openrc_service;
/// disable_openrc_service("NetworkManager", false)
/// ```
/// ## Example (user-service)
/// ```rust
/// use cosmoutils::modules::init::openrc::disable_openrc_service;
/// disable_openrc_service("pipewire", true)
/// ```
///
pub fn disable_openrc_service(service_name: &str, user_mode: bool) {
    let user = std::env::var("USER").unwrap();

    if user_mode && Path::new(&format!("/etc/init.d/{}.{}", user, user)).exists() {
        let mut command = Command::new("rc-update")
            .args(["--user", "del", service_name])
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
    } else {
        let mut command = Command::new("rc-update")
            .args(["del", service_name])
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
}