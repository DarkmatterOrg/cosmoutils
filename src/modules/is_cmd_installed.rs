use which::which;

/// Check if a command is installed
/// ## Example
/// ```rust
/// use cosmoutils::modules::is_cmd_installed::is_cmd_installed;
/// if is_cmd_installed("firefox") {
///    // Code
/// }
/// ```
///
pub fn is_cmd_installed(command: &str) -> bool {
    which(command).map(|path| path.exists()).unwrap_or(false)
}
