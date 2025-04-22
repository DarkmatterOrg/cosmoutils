use std::process::Command;


pub fn is_process_running(process_name: &str) -> bool {
    let command = Command::new("pgrep")
        .args(["-x", process_name])
        .output()
        .expect("Failed to run");


    let output_str = String::from_utf8_lossy(&command.stdout);

    let process = format!("{}", output_str);

    let result = if !process.is_empty() {
        true
    } else {
        false
    };

    return result;

}