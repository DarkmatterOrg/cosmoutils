#[cfg(feature = "bedrock")]
pub mod bedrock;
pub mod detect_os;
pub mod file_contains;
#[allow(dead_code)]
pub mod init;
pub mod is_cmd_installed;
pub mod is_root;
pub mod msg;
pub mod spinner;
#[cfg(feature = "nixos")]
pub mod nixos;
#[cfg(feature = "notifications")]
pub mod notifications;
pub mod process_running;
