use std::str::FromStr;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

/// Create a spinner
/// ```rust
///     use cosmoutils::modules::spinner::create_spinner;
///
///     fn main() {
///         create_spinner(10, "Loading...", "Dots12");
///     }
/// ```
pub fn create_spinner(seconds: u64, message: &str, spinner_name: &str) {
    let spinner_enum = Spinners::from_str(spinner_name)
        .unwrap_or(Spinners::Dots12);

    let mut sp = Spinner::new(spinner_enum, message.into());
    sleep(Duration::from_secs(seconds));
    sp.stop();

    print!("\r\x1b[2K");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
