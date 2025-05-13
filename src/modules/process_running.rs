use std::fs;
use std::path::{Path, PathBuf};

/// Chec if a process is running
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::process_running::is_process_running;
/// if is_process_running("firefox") {
///     // Code
/// }
/// ```
///
pub fn is_process_running(target_name: &str) -> bool {
    let proc_dir = Path::new("/proc");

    fs::read_dir(proc_dir)
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .filter_map(|entry| {
            let file_name = entry.file_name();
            if file_name.to_str()?.chars().all(|c| c.is_ascii_digit()) {
                Some(entry.path().join("comm"))
            } else {
                None
            }
        })
        .filter_map(|comm_path: PathBuf| fs::read_to_string(comm_path).ok())
        .any(|name| name.trim() == target_name)
}