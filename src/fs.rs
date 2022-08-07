use std::path::Path;
use std::fs::DirBuilder;

pub fn check_path(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn create_path_string(name: &str, path: &str) -> String {
    if let Some(path_str) = Path::new(path).join(name).as_os_str().to_str() {
        path_str.to_string()
    } else {
        panic!("path string couldn't be generated")
    }
}

pub fn create_directory(path: String) {
    DirBuilder::new().recursive(true).create(path).unwrap();
}