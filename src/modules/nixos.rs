use std::path::Path;

pub fn running_on_nixos() -> bool {
    if Path::new("/run/current-system/sw/bin").exists() {
        true
    } else {
        false;
        std::process::exit(1)
    }
}