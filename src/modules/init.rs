use std::fs as standard_fs;
use std::os::unix::fs;
use std::{path::Path, process::Command};

use super::msg::{error, success};

const DISABLE_MESSAGE: &str = "Disabled";
const ENABLE_MESSAGE: &str = "Enabled";
const FAILED_ENABLE_MESSAGE: &str = "Failed to enable";
const FAILED_DISABLE_MESSAGE: &str = "Failed to disable";

/// Enable a runit service by making a symlink from /etc/sv to /var/service
/*
    Example
    enable_runit_service("NetworkManager")
*/
pub fn enable_runit_service(service_name: String) -> std::io::Result<()> {
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
/*
    Example
    disable_runit_service("NetworkManager")
*/
pub fn disable_runit_service(service_name: String) -> std::io::Result<()> {
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

/// Enable a openRC service by running rc-update add or rc-update --user add
/*
    Example (system-service)
    enable_openrc_service("NetworkManager", false)

    Example (user-service)
    enable_openrc_service("pipewire", true)
*/
pub fn enable_openrc_service(service_name: String, user_mode: bool) {
    let user = std::env::var("USER").unwrap();

    if user_mode && Path::new(&format!("/etc/init.d/{}.{}", user, user)).exists() {
        let mut command = Command::new("rc-update")
            .args(["--user", "add", &service_name])
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
            .args(["add", &service_name])
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
/*
    Example (system-service)
    disable_openrc_service("NetworkManager", false)

    Example (user-service)
    disable_openrc_service("pipewire", true)
*/
pub fn disable_openrc_service(service_name: String, user_mode: bool) {
    let user = std::env::var("USER").unwrap();

    if user_mode && Path::new(&format!("/etc/init.d/{}.{}", user, user)).exists() {
        let mut command = Command::new("rc-update")
            .args(["--user", "del", &service_name])
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
            .args(["del", &service_name])
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


/// Enable a Dinit service by running dinitctl enable
/*
    Example (system-service)
    enable_dinit_service("NetworkManager")

    Example (user-service)
    enable_dinit_service("pipewire")
*/
pub fn enable_dinit_service(service_name: String) {
    let mut command = Command::new("dinitctl")
        .args(["enable", &service_name])
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
/*
    Example (system-service)
    disable_dinit_service("NetworkManager")

    Example (user-service)
    disable_dinit_service("pipewire")
*/
pub fn disable_dinit_service(service_name: String) {
    let mut command = Command::new("dinitctl")
        .args(["disable", &service_name])
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

/// Enable a Systemd service by running systemctl enable or systemctl --user enable
/*
    Example (system-service)
    enable_systemd_service("NetworkManager", false)

    Example (user-service)
    enable_systemd_service("pipewire", true)
*/
pub fn enable_systemd_service(service_name: String, user_mode: bool) {
    if user_mode {
        let mut command = Command::new("systemctl")
            .args(["--user", "enable", &service_name])
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
            .args(["enable", &service_name])
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
/*
    Example (system-service)
    disable_systemd_service("NetworkManager", false)

    Example (user-service)
    disable_systemd_service("pipewire", true)
*/
pub fn disable_systemd_service(service_name: String, user_mode: bool) {
    if user_mode {
        let mut command = Command::new("systemctl")
            .args(["--user", "disable", &service_name])
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
            .args(["--user", "disable", &service_name])
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
