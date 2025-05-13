/// Check if running as root
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::is_root::is_root;
/// if is_root() {
///    // Code
/// }
/// ```
///
pub fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}