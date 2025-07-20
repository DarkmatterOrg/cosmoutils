use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

/// Create a Dots12 spinner
/// ```rust
///     use cosmoutils::modules::spinner::create_spinner;
///
///     fn main() {
///         create_spinner(10, "Loading...");
///     }
/// ```
pub fn create_spinner(seconds: u64, message: &str) {
    let mut sp = Spinner::new(Spinners::Dots12, message.into());
    sleep(Duration::from_secs(seconds));
    sp.stop();

    print!("\r\x1b[2K");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
