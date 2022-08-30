use crate::types::Item;
use std::path::{Path, PathBuf};
use std::fs::{DirBuilder, File};

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
pub fn create_item(item_type: Item, name: &str, location: &PathBuf) -> PathBuf{
    if !valid_name(name) {
        panic!("not a valid name")
    }

    let mut path = join_paths(vec![location.to_str().unwrap(), name]);

    if let Item::Nt = item_type {
        path.set_extension("md");
        File::create(&path).unwrap();
    } else {
        DirBuilder::new().create(&path).unwrap();
    }

    path
}