use super::msg::{error, info};
use regex::Regex;
use std::fs::read_dir;
use std::path::Path;
use std::process::Command;

/// Check if Bedrock is present
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::bedrock::check_bedrock;
/// if check_bedrock() {
///     // Code
/// }
/// ```
///
pub fn check_bedrock() -> bool {
    Path::new("/bedrock").exists()
}

/// Get the count of stratas on a Bedrock system
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::bedrock::list_strata;
/// println!("Count of stratums: {}", list_strata());
/// ```
///
pub fn list_strata() -> usize {
    if !check_bedrock() {
        info("Not running on a Bedrock system..");
        std::process::exit(-1)
    }

    let bedrock_strata_folder = "/bedrock/strata";

    let read_count = read_dir(bedrock_strata_folder).unwrap();

    read_count.count() - 3
}

/// Detect if running in Bedrock's strat restriction
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::bedrock::running_in_strat;
/// if running_in_strat() {
///     // Code
/// }
/// ```
///
pub fn running_in_strat() -> bool {
    let path = std::env::var("PATH");

    match path {
        Ok(path) => {
            if !path.contains("/bedrock/cross/bin") {
                return true;
            }
        }

        Err(_) => {
            error("Failed to read PATH");
        }
    }

    false
}

/// Detect what package manager interface is Bedrock's PMM using
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::bedrock::detect_pmm_interface;
/// let pmm_interface = detect_pmm_interface();
/// match pmm_interface {
///     Some(pmm_interface) => {
///         let pmm_interface_str = pmm_interface.as_str();
///
///         match pmm_interface_str.trim() {
///             "pacman" => {
///                 println!("Pmm is using Pacman");
///              },
///
///              _ => println!("Failed to get the pmm interface."),
///         }
///     }
///
///     None => println!("Failed to get the pmm interface."),
/// }
///
/// ```
///
pub fn detect_pmm_interface() -> Option<String> {
    let bedrock_conf = "/bedrock/etc/bedrock.conf";

    let content = std::fs::read_to_string(bedrock_conf).ok()?;
    let re = Regex::new(r#"(?m)^user-interface = (?:(?:"(.*?)")|(?:(.*)))$"#).ok()?;
    let captures = re.captures(&content)?;

    captures
        .get(2)
        .or(captures.get(1))
        .map(|m| m.as_str().to_string())
}

/// Detect what stratum is providing the init
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::bedrock::detect_stratum_init;
/// println!("Strata: {}", detect_stratum_init());
/// ```
///
pub fn detect_stratum_init() -> String {
    if !check_bedrock() {
        std::process::exit(1)
    }

    let command = Command::new("brl")
        .args(["which", "/sbin/init"])
        .output()
        .expect("Failed to run");

    format!("{}", String::from_utf8_lossy(&command.stdout))
}
