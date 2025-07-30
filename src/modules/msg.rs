use std::fmt::Display;
use colored::Colorize;
use std::io::{self, Write};

/// ERROR;
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::msg::error;
/// error("Text");
/// ```
///

pub fn error<T: Display>(msg: T) {
    let _ = writeln!(io::stderr(), "{}: {}", "ERROR".bold().red(), msg);
}

/// INFO:
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::msg::info;
/// info("Text");
/// ```
///
pub fn info<T: Display>(msg: T) {
    println!("{}: {}", "INFO".bold().cyan(), msg);
}

/// NOTICE:
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::msg::notice;
/// notice("Text");
/// ```
///
pub fn notice<T: Display>(msg: T) {
    println!("{}: {}", "NOTICE".bold().blue(), msg);
}

/// WARN:
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::msg::warn;
/// warn("Text");
/// ```
///
pub fn warn<T: Display>(msg: T) {
    println!("{}: {}", "WARN".bold().yellow(), msg);
}

/// SUGGEST:
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::msg::suggest;
/// suggest("Text");
/// ```
///
pub fn suggest<T: Display>(msg: T) {
    println!("{}: {}", "SUGGEST".bold().magenta(), msg);
}

/// SUCCESS:
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::msg::success;
/// success("Text");
/// ```
///
pub fn success<T: Display>(msg: T) {
    println!("{}: {}", "SUCCESS".bold().green(), msg);
}
