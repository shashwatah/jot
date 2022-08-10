use fs_extra::{dir::CopyOptions, move_items};
use std::fs::{remove_dir_all, rename, DirBuilder};
use std::path::Path;

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn create_path_with_name(path: &str, name: &str) -> String {
    if let Some(path_with_name) = Path::new(path).join(name).to_str() {
        path_with_name.to_string()
    } else {
        panic!("path string couldn't be generated")
    }
}

pub fn create_folder(path: &str) {
    DirBuilder::new().create(path).unwrap();
}

pub fn delete_folder(path: &str) {
    remove_dir_all(path).unwrap()
}

pub fn rename_folder(name: &str, new_name: &str, path: &str) {
    let original_path = create_path_with_name(path, name);
    let new_path = create_path_with_name(path, new_name);
    rename(original_path, new_path).unwrap();
}

pub fn move_folder(name: &str, path: &str, new_path: &str) {
    // using crate: *fs_extra* here but i might implement a custom recursive move function later
    let original_path = create_path_with_name(path, name);
    let original_path_vec = vec![original_path];
    move_items(&original_path_vec, new_path, &CopyOptions::new()).unwrap();
}
