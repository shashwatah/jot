use crate::{
    enums::{Item, VaultItem},
    output::error::Error,
};
use dunce::canonicalize;
use fs_extra::{dir::CopyOptions, move_items};
use std::{
    env::consts::OS,
    fs::{remove_dir_all, remove_file, rename, DirBuilder, File},
    path::{Path, PathBuf},
    process::Command,
};

pub fn join_paths<T: AsRef<Path>>(paths: Vec<T>) -> PathBuf {
    let mut full_path = PathBuf::new();
    for path in paths {
        full_path.push(path);
    }
    full_path
}

// @desc: Wraps around dunce::canonicalize merely to translate error.
// 
// @notes:
//      -> Not the ideal place to check for a PathNotFound error, but has to stay for now.
pub fn resolve_path(path: &Path) -> Result<PathBuf, Error> {
    if let Ok(processed_path) = canonicalize(path) {
        Ok(processed_path)
    } else {
        Err(Error::PathNotFound)
    }
}

pub fn create_item(item_type: Item, name: &str, location: &Path) -> Result<PathBuf, Error> {
    let path = generate_item_path(&item_type, name, location)?;

    if let Err(error) = create_item_collect(&item_type, &path) {
        return Err(match error.kind() {
            std::io::ErrorKind::NotFound => Error::PathNotFound,
            std::io::ErrorKind::AlreadyExists => {
                Error::ItemAlreadyExists(item_type, name.to_owned())
            }
            _ => Error::Undefined(error),
        });
    }

    Ok(path)
}

pub fn remove_item(item_type: Item, name: &str, location: &Path) -> Result<(), Error> {
    let path = generate_item_path(&item_type, name, location)?;

    if let Err(error) = remove_item_collect(&item_type, &path) {
        return Err(match error.kind() {
            std::io::ErrorKind::NotFound => Error::ItemNotFound(item_type, name.to_owned()),
            _ => Error::Undefined(error),
        });
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

    if let Err(error) = rename(original_path, &new_path) {
        return Err(match error.kind() {
            std::io::ErrorKind::NotFound => Error::ItemNotFound(item_type, name.to_owned()),
            _ => Error::Undefined(error),
        });
    }

    Ok(new_path)
}

pub fn move_item(
    item_type: Item,
    name: &str,
    original_location: &PathBuf,
    new_location: &Path,
) -> Result<PathBuf, Error> {
    if new_location == original_location {
        return Err(Error::SameLocation);
    }

    let new_path = generate_item_path(&item_type, name, new_location)?;
    if new_path.exists() {
        return Err(Error::ItemAlreadyExists(item_type, name.to_owned()));
    }

    let original_path = vec![generate_item_path(&item_type, name, original_location)?];
    move_items(&original_path, new_location, &CopyOptions::new())?;

    Ok(new_path)
}

pub fn open_note(editor_data: (&String, bool), name: &str, location: &Path) -> Result<(), Error> {
    let path = generate_item_path(&Item::Nt, name, location)?;

    if !path.exists() {
        return Err(Error::ItemNotFound(Item::Nt, name.to_string()));
    }

    run_editor(editor_data, &path)?;
    Ok(())
}

pub fn open_folder(location: &Path) -> Result<(), Error> {
    let cmd = match OS {
        "windows" => "explorer",
        "linux" => "xdg-open",
        "macos" => "open",
        _ => return Ok(()),
    };

    if let Err(err) = Command::new(cmd).arg(location).spawn() {
        Err(Error::Undefined(err))
    } else {
        Ok(())
    }
}

pub fn run_editor(editor_data: (&String, bool), path: &Path) -> Result<(), Error> {
    let (editor, conflict) = editor_data;

    if let Err(error) = run_editor_collect(editor, conflict, path) {
        return Err(match error.kind() {
            std::io::ErrorKind::NotFound => Error::EditorNotFound,
            _ => Error::Undefined(error),
        });
    }

    Ok(())
}

pub fn filtered_list(item_type: &VaultItem, path: PathBuf) {
    let mut filtered_entries: Vec<String> = vec![];

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap().path();
        let entry_name = entry.file_stem().unwrap().to_str().unwrap();

        match item_type {
            VaultItem::Folder | VaultItem::Fd => {
                if entry.is_dir() {
                    filtered_entries.push(entry_name.to_string())
                }
            }
            _ => {
                if entry.is_file() && entry.extension().unwrap() == "md" {
                    filtered_entries.push(format!("\x1b[0;34m{entry_name}\x1b[0m"));
                }
            }
        }
    }

    for (index, entry) in filtered_entries.iter().enumerate() {
        if filtered_entries.len() - index == 1 {
            print!("└── ")
        } else {
            print!("├── ")
        }

        println!("{entry}");
    }
}

// @desc: Recursively goes over the contained elements in a folder, then prints the folder's 
//        tree.
//
// @notes:
//      -> The value in "level" defines how deep into the tree we are.
//      -> "were_last" is a vector of booleans whose length at any moment is equal to the 
//         current level + 1. Everytime a folder is traversed, a boolean represening if it
//         was the last element in its parent folder or not is added to the vector. This 
//         helps in determining when to print a pipe for elements not part of current parent
//         folder.
pub fn rec_list(mut were_last: Vec<bool>, path: PathBuf) -> Vec<bool> {
    let length = path.read_dir().unwrap().count();

    for (count, entry) in path.read_dir().unwrap().enumerate() {
        let entry = entry.unwrap().path();
        let entry_name = entry.file_stem().unwrap().to_str().unwrap();

        if entry_name == ".jot" {
            continue;
        }

        if let Some(extension) = entry.extension() {
            if entry.is_file() && extension != "md" {
                continue;
            }
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

        if entry.is_dir() {
            println!("{entry_name}");

            were_last.push(is_last);
            were_last = rec_list(were_last, entry);
            were_last.pop();
        } else {
            println!("\x1b[0;34m{entry_name}\x1b[0m",);
        }
    }

    were_last
}

fn valid_name(name: &str) -> bool {
    name.chars().all(|char| !r#"\/?%*:|"<>"#.contains(char))
}

fn generate_item_path(item_type: &Item, name: &str, location: &Path) -> Result<PathBuf, Error> {
    if !valid_name(name) {
        return Err(Error::InvalidName);
    }

    let mut path = join_paths(vec![location.to_str().unwrap(), name]);

    if let Item::Nt = item_type {
        path.set_extension("md");
    }

    Ok(path)
}

// All "_collect" functions below are meant to collect errors from all possible routes a
// function can take.
// These errors are then converted to native errors in their corresponding functions above.

fn create_item_collect(item_type: &Item, path: &Path) -> Result<(), std::io::Error> {
    if let Item::Nt = item_type {
        File::options().create_new(true).write(true).open(path)?;
    } else {
        DirBuilder::new().create(path)?;
    }

    Ok(())
}

fn remove_item_collect(item_type: &Item, path: &Path) -> Result<(), std::io::Error> {
    if let Item::Nt = item_type {
        remove_file(path)?;
    } else {
        remove_dir_all(path)?;
    }

    Ok(())
}

fn run_editor_collect(editor: &str, conflict: bool, path: &Path) -> Result<(), std::io::Error> {
    let mut cmd = Command::new(editor).arg(path.to_str().unwrap()).spawn()?;

    if conflict {
        cmd.wait()?;
    }

    Ok(())
}
