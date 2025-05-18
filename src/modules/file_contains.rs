use std::fs;

/// Checks if a file contains a string
///
/// ## Example
/// ```rust
///  use cosmoutils::modules::file_contains::does_file_contain;
///  if does_file_contain("test.txt", "hello!") {
///        // Code
///  }
/// ```
///
pub fn does_file_contain(filepath: &str, text: &str) -> bool {
    let file = fs::read_to_string(filepath).unwrap();

    file.contains(text)
}
