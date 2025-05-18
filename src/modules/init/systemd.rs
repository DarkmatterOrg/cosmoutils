use std::process::Command;
use crate::modules::init::{DISABLE_MESSAGE, ENABLE_MESSAGE, FAILED_DISABLE_MESSAGE, FAILED_ENABLE_MESSAGE, FAILED_SET_MESSAGE, SET_MESSAGE};
use crate::modules::msg::{error, success};

/// Enable a Systemd service by running systemctl enable or systemctl --user enable
///
/// ## Example (system-service)
/// ```rust
/// use cosmoutils::modules::init::enable_systemd_service;
/// enable_systemd_service("NetworkManager", false)
/// ```
/// ## Example (user-service)
/// ```rust
/// use cosmoutils::modules::init::enable_systemd_service;
/// enable_systemd_service("pipewire", true)
/// ```
///
pub fn enable_systemd_service(service_name: &str, user_mode: bool) {
    if user_mode {
        let mut command = Command::new("systemctl")
            .args(["--user", "enable", service_name])
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
        let mut command = Command::new("systemctl")
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
}

/// Disable a Systemd service by running systemctl disable or systemctl --user disable
///
/// ## Example (system-service)
/// ```rust
/// use cosmoutils::modules::init::disable_systemd_service;
/// disable_systemd_service("NetworkManager", false)
/// ```
/// ## Example (user-service)
/// ```rust
/// use cosmoutils::modules::init::disable_systemd_service;
/// disable_systemd_service("pipewire", true)
/// ```
///
pub fn disable_systemd_service(service_name: &str, user_mode: bool) {
    if user_mode {
        let mut command = Command::new("systemctl")
            .args(["--user", "disable", service_name])
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
        let mut command = Command::new("systemctl")
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
}

/// Switch to a different systemd target
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::init::systemd::set_systemd_target;
/// set_systemd_target("multi-user");
/// ```
///
pub fn set_systemd_target(target_name: &str) {
    let mut command = Command::new("systemctl")
        .args(["isolate", target_name])
        .spawn()
        .expect("Failed to run");
    let msg = format!("{} {}", SET_MESSAGE, target_name);
    let failed_msg = format!("{} {}", FAILED_SET_MESSAGE, target_name);

    let status = command.wait().expect("Failed to run the command.");

    if !status.success() {
        error(&failed_msg)
    } else {
        success(&msg);
    }
}