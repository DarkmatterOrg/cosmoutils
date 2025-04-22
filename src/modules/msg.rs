use colored::Colorize;

/// ERROR;
pub fn error(msg: &str) {
    print!("{}: {}", "ERROR".bold().red(), msg);
}

/// INFO:
pub fn info(msg: &str) {
    print!("{}: {}", "INFO".bold().cyan(), msg);
}

/// NOTICE:
pub fn notice(msg: &str) {
    print!("{}: {}", "NOTICE".bold().blue(), msg);
}

/// WARN:
pub fn warn(msg: &str) {
    print!("{}: {}", "WARN".bold().yellow(), msg);
}

/// SUGGEST:
pub fn suggest(msg: &str) {
    print!("{}: {}", "SUGGEST".bold().magenta(), msg);
}

/// SUCCESS:
pub fn success(msg: &str) {
    print!("{}: {}", "SUCCESS".bold().green(), msg);
}
