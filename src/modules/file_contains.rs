use std::fs;

pub fn does_file_contain(filepath: &str, text: &str) -> bool {
    let file = fs::read_to_string(filepath).unwrap();

    if file.contains(text) {
        true
    } else {
        false
    }
}