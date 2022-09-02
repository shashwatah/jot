pub use crate::types::Vault as CurrentVault;
use crate::{
    types::VaultItem,
    utils::{
        create_item, join_paths, move_item, process_path, rec_list, remove_item, rename_item,
        run_editor,
    },
};
use std::path::PathBuf;

impl CurrentVault {
    pub fn list(&self) {
        let folder = self.get_folder();

        if folder.as_os_str().is_empty() {
            println!("{}", self.get_name());
        } else {
            println!("{} > {}", self.get_name(), folder.display());
        }

        let location = self.generate_location();
        rec_list(1, location);
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
        run_editor(editor_data, name, &location);
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
