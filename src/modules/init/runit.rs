use std::os::unix::fs;
use crate::modules::init::{DISABLE_MESSAGE, ENABLE_MESSAGE, FAILED_DISABLE_MESSAGE, FAILED_ENABLE_MESSAGE};
use crate::modules::msg::{error, success};

/// Enable a runit service by making a symlink from /etc/sv to /var/service
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::init::enable_runit_service;
/// enable_runit_service("NetworkManager")
/// ```
///
pub fn enable_runit_service(service_name: &str) -> std::io::Result<()> {
    let service_dest = "/var/service";

    let service_symlink = fs::symlink(
        format!("/etc/sv/{}", service_name),
        format!("{}/{}", service_dest, service_name),
    );

    let msg = format!("{} {}", ENABLE_MESSAGE, service_name);
    let failed_msg = format!("{} {}", FAILED_ENABLE_MESSAGE, service_name);

    match service_symlink {
        Ok(_s) => success(&msg),
        Err(_e) => error(&failed_msg),
    }

    Ok(())
}

/// Disable a runit service by removing the symlink from /var/service
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::init::disable_runit_service;
/// disable_runit_service("NetworkManager")
/// ```
///
pub fn disable_runit_service(service_name: &str) -> std::io::Result<()> {
    let service_dest = "/var/service";

    let msg = format!("{} {}", DISABLE_MESSAGE, service_name);
    let failed_msg = format!("{} {}", FAILED_DISABLE_MESSAGE, service_name);

    let disable_service = standard_fs::remove_file(format!("{}/{}", service_dest, service_name));

    match disable_service {
        Ok(_removed) => success(&msg),
        Err(_failed) => error(&failed_msg),
    }

    Ok(())
}