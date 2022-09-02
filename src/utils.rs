use crate::types::Item;
use fs_extra::{dir::CopyOptions, move_items};
use std::{
    fs::{remove_dir_all, remove_file, rename, DirBuilder, File},
    path::{Path, PathBuf},
    process::Command,
};

fn valid_name(name: &str) -> bool {
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
// not using canonicalize because it returns \\?\C:\*path* on windows
pub fn process_path(path: &Path) -> PathBuf {
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

// creates item(folders(dr & vl) and md files) and returns path to created item
// might rename this later
pub fn create_item(item_type: Item, name: &str, location: &PathBuf) -> PathBuf {
    let path = generate_item_path(&item_type, name, location);

    if let Item::Nt = item_type {
        File::options()
            .create_new(true)
            .write(true)
            .open(&path)
            .unwrap();
    } else {
        DirBuilder::new().create(&path).unwrap();
    }

    path
}

pub fn remove_item(item_type: Item, name: &str, location: &PathBuf) {
    let path = generate_item_path(&item_type, name, location);

    if let Item::Nt = item_type {
        remove_file(path).unwrap();
    } else {
        remove_dir_all(path).unwrap();
    }
}

pub fn rename_item(item_type: Item, name: &str, new_name: &str, location: &PathBuf) -> PathBuf {
    if new_name == name {
        panic!("new name can't be same as old name")
    }

    let original_path = generate_item_path(&item_type, name, location);
    let new_path = generate_item_path(&item_type, new_name, location);

    rename(original_path, &new_path).unwrap();
    new_path
}

pub fn move_item(
    item_type: Item,
    name: &str,
    original_location: &PathBuf,
    new_location: &PathBuf,
) -> PathBuf {
    if new_location == original_location {
        panic!(
            "{} {} already exists in this location",
            item_type.full(),
            name
        )
    }

    let new_path = generate_item_path(&item_type, name, new_location);
    if new_path.exists() {
        panic!(
            "a {} named {} already exists in new location",
            item_type.to_vault_item().full(),
            name
        )
    }

    let original_path = vec![generate_item_path(&item_type, name, original_location)];
    move_items(&original_path, &new_location, &CopyOptions::new()).unwrap();

    new_path
}

pub fn run_editor(editor_data: (&String, bool), name: &str, location: &PathBuf) {
    let path = generate_item_path(&Item::Nt, name, location);

    if !path.exists() {
        panic!("note {} doesn't exist", name)
    }

    let (editor, conflict) = editor_data;

    let mut cmd = Command::new(editor)
        .arg(path.to_str().unwrap())
        .spawn()
        .unwrap();

    if conflict {
        cmd.wait().unwrap();
    }
}

fn generate_item_path(item_type: &Item, name: &str, location: &PathBuf) -> PathBuf {
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let mut path = join_paths(vec![location.to_str().unwrap(), name]);

    if let Item::Nt = item_type {
        path.set_extension("md");
    }

    path
}
