pub use crate::types::Vault as CurrentVault;
use crate::{
    types::VaultItem,
    utils::{create_item, join_paths, move_item, process_path, remove_item, rename_item},
};
use std::{path::PathBuf, process::Command};
use walkdir::WalkDir;

impl CurrentVault {
    // god help anyone who ever stumbles on this
    pub fn list(&self) {
        let mut current = self.generate_location();

        let mut level = 0;

        if self.get_folder().to_str().unwrap() != "" {
            println!("{} > {}", self.get_name(), self.get_folder().display());
        } else {
            println!("{}", self.get_name());
        }

        let length = WalkDir::new(&current)
            .into_iter()
            .filter_map(|e| e.ok())
            .count();
        for (count, entry) in WalkDir::new(&current)
            .into_iter()
            .filter_map(|e| e.ok())
            .enumerate()
        {
            let entry_name = entry.path().file_stem().unwrap().to_str().unwrap();

            if entry_name == self.get_name()
                || entry_name == ".jot"
                || entry.path().ends_with(".jot/data")
            {
                continue;
            }

            let folder_name = self.get_folder().file_name();
            if folder_name.is_some() {
                if entry_name == folder_name.unwrap().to_str().unwrap() {
                    continue;
                }
            }

            if entry.path().is_dir() {
                if entry.path().starts_with(&current) {
                    level += 1;
                    current.push(entry_name);
                } else {
                    let mut anc = entry.path().ancestors();

                    loop {
                        let a = anc.next().unwrap();

                        current.pop();
                        level -= 1;

                        if a.starts_with(&current) {
                            level += 1;
                            current.push(entry_name);
                            break;
                        }
                    }
                }
            }

            for i in 0..level {
                if length - count == 1 {
                    print!("└──")
                } else if level - i == 1 {
                    print!("├──")
                } else {
                    print!("│  ")
                }
            }

            println!("{}", entry_name);
        }
    }

    pub fn create_vault_item(&self, item_type: VaultItem, name: &String) {
        let location = self.generate_location();

        create_item(item_type.to_item(), name, &location);
        print!("{} {} created", item_type.full(), name)
    }

    pub fn remove_vault_item(&self, item_type: VaultItem, name: &String) {
        let location = self.generate_location();

        remove_item(item_type.to_item(), name, &location);
        print!("{} {} removed", item_type.full(), name)
    }

    pub fn rename_vault_item(&self, item_type: VaultItem, name: &String, new_name: &String) {
        let location = self.generate_location();

        rename_item(item_type.to_item(), name, new_name, &location);
        print!("{} {} renamed to {}", item_type.full(), name, new_name)
    }

    pub fn move_vault_item(&self, item_type: VaultItem, name: &String, new_location: &PathBuf) {
        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        let original_location = join_paths(vec![&vault_path, self.get_folder()]);

        let new_location = process_path(&join_paths(vec![&original_location, new_location]));

        if !new_location.starts_with(vault_path) {
            panic!("path crosses the bounds of vault")
        }

        move_item(item_type.to_item(), name, &original_location, &new_location);

        print!("{} {} moved", item_type.full(), name)
    }

    pub fn vmove_vault_item(
        &self,
        item_type: &VaultItem,
        name: &String,
        vault_name: &String,
        vault_location: &PathBuf,
    ) {
        let original_location = self.generate_location();

        if vault_name == self.get_name() {
            panic!(
                "{} {} already exists in vault {}",
                item_type.full(),
                name,
                vault_name
            )
        }

        let new_location = join_paths(vec![vault_location.to_str().unwrap(), vault_name]);
        move_item(item_type.to_item(), name, &original_location, &new_location);

        print!(
            "{} {} moved to vault {}",
            item_type.full(),
            name,
            vault_name
        )
    }

    pub fn open_note(&self, name: &String, editor_data: (&String, bool)) {
        let location = self.generate_location();
        let mut path = join_paths(vec![location.to_str().unwrap(), name]);
        path.set_extension("md");

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

    pub fn change_folder(&mut self, path: &PathBuf) {
        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        let new_location = process_path(&join_paths(vec![&vault_path, self.get_folder(), path]));

        if !new_location.exists() {
            panic!("path doesn't exist")
        }

        if !new_location.starts_with(&vault_path) {
            panic!("path crosses the bounds of vault")
        }

        let mut destination_folder = new_location.strip_prefix(vault_path).unwrap();
        if destination_folder.has_root() {
            destination_folder = destination_folder.strip_prefix("/").unwrap();
        }
        let destination_folder = destination_folder.to_path_buf();

        self.set_folder(destination_folder);
        print!("changed folder");
    }

    fn generate_location(&self) -> PathBuf {
        let (current_vault_name, current_vault_location, folder) = self.get_path_data();
        join_paths(vec![
            current_vault_location,
            &PathBuf::from(current_vault_name),
            folder,
        ])
    }
}
