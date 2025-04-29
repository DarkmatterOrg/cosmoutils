/// Check if running as root
///
/// ## Example
/// ```rust
/// if is_root() {
///    // Code
/// }
/// ```
///
pub fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}