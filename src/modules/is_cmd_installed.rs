use which::which;

/// Check if a command is installed
/// ## Example
/// ```rust
/// if is_cmd_installed("firefox") {
///    // Code
/// }
/// ```
///
pub fn is_cmd_installed(command: String) -> bool {
    let result = which(command).unwrap().exists();

    return result;
}