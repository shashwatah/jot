use fs_extra::{dir::CopyOptions, move_items};
use std::fs::{remove_dir_all, remove_file, rename, DirBuilder, File};
use std::path::{Path, PathBuf};

use path_slash::PathExt as _;

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn join_paths(paths: Vec<&str>) -> String {
    let mut full_path = PathBuf::new();
    for path in paths {
        full_path.push(path);
    }
    full_path.to_str().unwrap().to_string()
}

pub fn collapse_path(path: &str) -> String {
    let mut new_path = PathBuf::new();
    for element in PathBuf::from(path).iter() {
        if element == ".." {
            new_path.pop();
        } else if element != "." {
            new_path.push(element);
        }
    }
    return new_path.to_str().unwrap().to_string();
}

pub fn unix_path(path: &str) -> String {
    // using path_slash crate to convert system created paths on windows (uses \ instead of /)
    // to unix style paths to maintain consistency when printing paths.
    Path::new(path).to_slash().unwrap().to_string()
}

pub fn create_folder(path: &str) {
    DirBuilder::new().create(path).unwrap();
}

pub fn delete_folder(path: &str) {
    remove_dir_all(path).unwrap()
}

pub fn create_file(path: &str) {
    File::create(path).unwrap();
}

pub fn delete_file(path: &str) {
    remove_file(path).unwrap();
}

pub fn rename_item(name: &str, new_name: &str, path: &str) {
    let original_path = join_paths(vec![path, name]);
    let new_path = join_paths(vec![path, new_name]);
    rename(original_path, new_path).unwrap();
}

pub fn move_item(name: &str, path: &str, new_path: &str) {
    // using crate: *fs_extra* here but i might implement a custom recursive move function later
    let original_path = vec![join_paths(vec![path, name])];
    move_items(&original_path, new_path, &CopyOptions::new()).unwrap();
}