use fs_extra::{dir::CopyOptions, move_items};
use std::fs::{remove_dir_all, remove_file, rename, DirBuilder, File};
use std::path::{Path, PathBuf};

pub fn valid_name(name: &str) -> bool {
    name.chars().all(|char| !r#"\/?%*:|"<>"#.contains(char))
}

pub fn join_paths<T: AsRef<Path>>(paths: Vec<T>) -> PathBuf {
    let mut full_path = PathBuf::new();
    for path in paths {
        full_path.push(path);
    }
    full_path
}

// returns new pathbuf -> with slashes formatted according to os & '..'s collapsed
// use this when storing or displaying paths
pub fn process_path(path: &PathBuf) -> PathBuf {
    let mut processed_path = PathBuf::new();

    for element in path.iter() {
        if element == ".." {
            processed_path.pop();
        } else if element != "." {
            processed_path.push(element);
        }
    }

    processed_path
}

pub fn create_folder(path: &PathBuf) {
    DirBuilder::new().create(path).unwrap();
}

pub fn delete_folder(path: &PathBuf) {
    remove_dir_all(path).unwrap()
}

pub fn create_file(path: &PathBuf) {
    File::create(path).unwrap();
}

pub fn delete_file(path: &PathBuf) {
    remove_file(path).unwrap();
}

pub fn rename_item(name: &str, new_name: &str, location: &PathBuf) {
    let original_path = join_paths(vec![location.to_str().unwrap(), name]);
    let new_path = join_paths(vec![location.to_str().unwrap(), new_name]);
    rename(original_path, new_path).unwrap();
}

pub fn move_item(name: &str, original_location: &PathBuf, new_location: &PathBuf) {
    // using crate: *fs_extra* here but i might implement a custom recursive move function later
    let original_path = vec![join_paths(vec![original_location.to_str().unwrap(), name])];
    move_items(&original_path, new_location, &CopyOptions::new()).unwrap();
}
