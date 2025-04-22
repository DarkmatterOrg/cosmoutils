use which::which;

pub fn is_cmd_installed(command: String) -> bool {
    let result = which(command).unwrap().exists();

    return result;
}