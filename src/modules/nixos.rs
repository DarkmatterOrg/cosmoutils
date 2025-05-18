use std::path::Path;

/// Check if running on NixOS
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::nixos::running_on_nixos;
/// if running_on_nixos() {
///     // Code
/// }
/// ```
///
pub fn running_on_nixos() -> bool {
    Path::new("/run/current-system/sw/bin").exists()
}
