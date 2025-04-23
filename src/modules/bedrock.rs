use std::fs::read_dir;
use std::path::Path;
use std::process::Command;

use super::msg::info;

/// Check if Bedrock is present
/*
    Example:
    if check_bedrock() {
        // Code
    }

*/
fn check_bedrock() -> bool {
    if Path::new("/bedrock").exists() {
        return true
    } else {
        false
    }
}

/// Get the count of stratas on a Bedrock system
/*
    Example
    println!("Count: {}", list_strata());
*/

pub fn list_strata() -> usize {
    if !check_bedrock() {
        info("Not running on a Bedrock system..");
        std::process::exit(-1)
    }

    let bedrock_strata_folder = "/bedrock/strata";

    let read_count = read_dir(bedrock_strata_folder).unwrap();

    let count = read_count.count() - 3;

    return count;
}

/// Detect what stratum is providing the init
/*
    Example
    println!("Strata: {}", detect_stratum_init());

*/
pub fn detect_stratum_init() -> String {
    if !check_bedrock() {
        std::process::exit(1)
    }

    let command = Command::new("brl")
        .args(["which", "/sbin/init"])
        .output()
        .expect("Failed to run");


    let output_str = String::from_utf8_lossy(&command.stdout);

    let output = format!("{}", output_str);

    return output
}
