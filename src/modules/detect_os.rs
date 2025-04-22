use std::path::Path;
use regex::Regex;

/// get_os() gets the system name from /etc/os-release or in case of Bedrock from /bedrock/etc/os-release
pub fn get_os() -> String {
    let os_release_path: &str;

    let _path = if Path::new("/bedrock").exists() {
        os_release_path = "/bedrock/etc/os-release";
    } else {
        os_release_path = "/etc/os-release";
    };

    let system_name = read_os_release(os_release_path);

    return system_name.unwrap_or("Unknown".to_string());
}

/// Gets the value of PRETTY_NAME using Regex
fn read_os_release(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let re = Regex::new(r#"(?m)^PRETTY_NAME=(?:(?:"(.*?)")|(?:(.*)))$"#).ok()?;
    let captures = re.captures(&content)?;

    captures.get(2).or(captures.get(1)).map(|m| m.as_str().to_string())
}