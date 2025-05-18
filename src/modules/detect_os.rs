use regex::Regex;
use std::path::Path;

/// get_os() gets the system name from /etc/os-release or in case of Bedrock from /bedrock/etc/os-release
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::detect_os::get_os;
/// if let Some(os_name) = get_os() {
///     println!("OS: {}", os_name);
/// }
/// ```
///
pub fn get_os() -> Option<String> {
    let os_release_path = find_os_release();

    read_os_release(os_release_path.as_str())
}

fn find_os_release() -> String {
    let os_release_path = if Path::new("/bedrock").exists() {
        "/bedrock/etc/os-release"
    } else {
        "/etc/os-release"
    };

    os_release_path.to_string()
}

/// get_os_version() gets the system name from /etc/os-release or in case of Bedrock from /bedrock/etc/os-release
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::detect_os::get_os_version;
/// if let Some(os_version) = get_os_version() {
///     println!("OS: {}", os_version);
/// }
/// ```
///
pub fn get_os_version() -> Option<String> {
    let os_release_path = find_os_release();

    let content = std::fs::read_to_string(os_release_path).ok()?;
    let re = Regex::new(r#"(?m)^VERSION_ID=(?:(?:"(.*?)")|(?:(.*)))$"#).ok()?;
    let captures = re.captures(&content)?;

    captures
        .get(2)
        .or(captures.get(1))
        .map(|m| m.as_str().to_string())
}

/// Gets the value of ID using Regex
fn read_os_release(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let re = Regex::new(r#"(?m)^ID=(?:(?:"(.*?)")|(?:(.*)))$"#).ok()?;
    let captures = re.captures(&content)?;

    captures
        .get(2)
        .or(captures.get(1))
        .map(|m| m.as_str().to_string())
}
