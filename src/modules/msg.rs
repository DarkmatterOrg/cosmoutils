use colored::Colorize;

/// ERROR;
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::msg::error;
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
/// use cosmoutils::modules::msg::info;
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
/// use cosmoutils::modules::msg::notice;
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
/// use cosmoutils::modules::msg::warn;
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
/// use cosmoutils::modules::msg::suggest;
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
/// use cosmoutils::modules::msg::success;
/// success("Text");
/// ```
///
pub fn success(msg: &str) {
    print!("{}: {}", "SUCCESS".bold().green(), msg);
}
