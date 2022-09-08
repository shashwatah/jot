use crate::{enums::Item, error::Error};
use fs_extra::{dir::CopyOptions, move_items};
use std::{
    fs::{remove_dir_all, remove_file, rename, DirBuilder, File},
    io::{Error as IOError, ErrorKind},
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
pub fn create_item(item_type: Item, name: &str, location: &Path) -> Result<PathBuf, Error> {
    let path = generate_item_path(&item_type, name, location)?;

    if let Item::Nt = item_type {
        File::options().create_new(true).write(true).open(&path)?;
    } else {
        DirBuilder::new().create(&path)?;
    }

    Ok(path)
}

pub fn remove_item(item_type: Item, name: &str, location: &Path) -> Result<(), Error> {
    let path = generate_item_path(&item_type, name, location)?;

    if let Item::Nt = item_type {
        remove_file(path)?;
    } else {
        remove_dir_all(path)?;
    }

    Ok(())
}

pub fn rename_item(
    item_type: Item,
    name: &str,
    new_name: &str,
    location: &Path,
) -> Result<PathBuf, Error> {
    if new_name == name {
        return Err(Error::SameName);
    }

    let original_path = generate_item_path(&item_type, name, location)?;
    let new_path = generate_item_path(&item_type, new_name, location)?;

    rename(original_path, &new_path)?;
    Ok(new_path)
}

pub fn move_item(
    item_type: Item,
    name: &str,
    original_location: &PathBuf,
    new_location: &Path,
) -> Result<PathBuf, Error> {
    if new_location == original_location {
        return Err(Error::SameLocation(item_type));
    }

    let new_path = generate_item_path(&item_type, name, new_location)?;
    if new_path.exists() {
        return Err(Error::ItemAlreadyExists(
            item_type.to_vault_item(),
            name.to_owned(),
        ));
    }

    let original_path = vec![generate_item_path(&item_type, name, original_location)?];
    move_items(&original_path, &new_location, &CopyOptions::new()).expect("move_items err");

    Ok(new_path)
}

pub fn run_editor(editor_data: (&String, bool), name: &str, location: &Path) -> Result<(), Error> {
    let path = generate_item_path(&Item::Nt, name, location)?;

    if !path.exists() {
        return Err(Error::FSError(IOError::new(
            ErrorKind::NotFound,
            format!("note {} doesn't exist", name),
        )));
    }

    let (editor, conflict) = editor_data;

    let mut cmd = Command::new(editor).arg(path.to_str().unwrap()).spawn()?;

    if conflict {
        cmd.wait()?;
    }

    Ok(())
}

pub fn rec_list(mut were_last: Vec<bool>, path: PathBuf) -> Vec<bool> {
    let length = path.read_dir().unwrap().count();

    for (count, entry) in path.read_dir().unwrap().enumerate() {
        let entry = entry.unwrap().path();
        let entry_name = entry.file_stem().unwrap().to_str().unwrap();

        if entry_name == ".jot" {
            continue;
        }

        let is_last = length - count == 1;

        for level in 0..were_last.len() - 1 {
            if were_last[level + 1] {
                print!("    ")
            } else {
                print!("│   ")
            }
        }

        if is_last {
            print!("└── ")
        } else {
            print!("├── ")
        }

        println!("{}", entry_name);

        if entry.is_dir() {
            were_last.push(is_last);
            were_last = rec_list(were_last, entry);
            were_last.pop();
        }
    }

    were_last
}

fn generate_item_path(item_type: &Item, name: &str, location: &Path) -> Result<PathBuf, Error> {
    if !valid_name(name) {
        return Err(Error::InvalidName(name.to_owned()));
    }

    let mut path = join_paths(vec![location.to_str().unwrap(), name]);

    if let Item::Nt = item_type {
        path.set_extension("md");
    }

    Ok(path)
}
