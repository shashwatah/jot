use fs_extra::{dir::CopyOptions, move_items};
use std::fs::{remove_dir_all, rename, DirBuilder};
use std::path::Path;

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

pub fn create_directory(path: &str) {
    DirBuilder::new().create(path).unwrap();
}

pub fn delete_directory(path: &str) {
    remove_dir_all(path).unwrap()
}

pub fn rename_directory(name: &str, new_name: &str, path: &str) {
    let og_path = Path::new(path).join(name);
    let new_path = Path::new(path).join(new_name);
    rename(og_path, new_path).unwrap();
}

pub fn move_dirctory(name: &str, path: &str, new_path: &str) {
    // using crate: *fs_extra* here but i might implement a custom recursive move function later
    if let Some(og_path_str) = Path::new(path).join(name).to_str() {
        let og_path = vec![og_path_str];
        move_items(&og_path, new_path, &CopyOptions::new()).unwrap();
    }
}
