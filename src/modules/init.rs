use std::fs as standard_fs;
use std::os::unix::fs;
use std::{path::Path, process::Command};

use super::msg::{error, success};

const DISABLE_MESSAGE: &str = "Disabled";
const ENABLE_MESSAGE: &str = "Enabled";
const FAILED_ENABLE_MESSAGE: &str = "Failed to enable";
const FAILED_DISABLE_MESSAGE: &str = "Failed to disable";

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
