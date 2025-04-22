use std::fs::read_dir;
use std::path::Path;
use std::process::Command;

use super::msg::info;


fn check_bedrock() -> bool {
    if Path::new("/bedrock").exists() {
        return true
    } else {
        false
    }
}

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

pub fn detect_stratum_init() {
    if !check_bedrock() {
        return;
    }

    let command = Command::new("brl")
        .args(["which", "/sbin/init"])
        .output()
        .expect("Failed to run");


    let output_str = String::from_utf8_lossy(&command.stdout);

    let output = format!("{}", output_str);

    info(&output);
}
