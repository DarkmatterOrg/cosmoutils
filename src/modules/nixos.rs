use std::path::Path;

/// Check if NixOS is present
/*
    Example
    if running_on_nixos() {
        // Code
    }
*/
pub fn running_on_nixos() -> bool {
    if Path::new("/run/current-system/sw/bin").exists() {
        true
    } else {
        false;
        std::process::exit(1)
    }
}