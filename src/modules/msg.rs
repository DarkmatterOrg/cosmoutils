use colored::Colorize;

/// ERROR;
///
/// ## Example
/// ```rust
/// error("Text");
/// ```
///
pub fn error(msg: &str) {
    print!("{}: {}", "ERROR".bold().red(), msg);
}

/// INFO:
///
/// ## Example
/// ```rust
/// info("Text");
/// ```
///
pub fn info(msg: &str) {
    print!("{}: {}", "INFO".bold().cyan(), msg);
}

/// NOTICE:
///
/// ## Example
/// ```rust
/// notice("Text");
/// ```
///
pub fn notice(msg: &str) {
    print!("{}: {}", "NOTICE".bold().blue(), msg);
}

/// WARN:
///
/// ## Example
/// ```rust
/// warn("Text");
/// ```
///
pub fn warn(msg: &str) {
    print!("{}: {}", "WARN".bold().yellow(), msg);
}

/// SUGGEST:
///
/// ## Example
/// ```rust
/// suggest("Text");
/// ```
///
pub fn suggest(msg: &str) {
    print!("{}: {}", "SUGGEST".bold().magenta(), msg);
}

/// SUCCESS:
///
/// ## Example
/// ```rust
/// success("Text");
/// ```
///
pub fn success(msg: &str) {
    print!("{}: {}", "SUCCESS".bold().green(), msg);
}
