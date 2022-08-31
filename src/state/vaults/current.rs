pub use crate::types::Vault as CurrentVault;
use crate::{
    types::{Item, VaultItem},
    utils::{create_item, join_paths, move_item, process_path, remove_item, rename_item},
};
use std::path::PathBuf;

impl CurrentVault {
    pub fn folder(&self) {
        print!("{}:{}", self.get_name(), self.get_folder().display());
    }

    pub fn create_vault_item(&self, item_type: VaultItem, name: &String) {
        let (location, item_type, item_type_full) = self.parse_item_data(&item_type);

        create_item(item_type, name, &location);
        print!("{} {} created", item_type_full, name)
    }

    pub fn remove_vault_item(&self, item_type: VaultItem, name: &String) {
        let (location, item_type, item_type_full) = self.parse_item_data(&item_type);

        remove_item(item_type, name, &location);
        print!("{} {} removed", item_type_full, name)
    }

    pub fn rename_vault_item(&self, item_type: VaultItem, name: &String, new_name: &String) {
        let (location, item_type, item_type_full) = self.parse_item_data(&item_type);

        rename_item(item_type, name, new_name, &location);
        print!("{} {} renamed to {}", item_type_full, name, new_name)
    }

    pub fn move_vault_item(&self, item_type: VaultItem, name: &String, new_location: &PathBuf) {
        let (original_location, item_type, item_type_full) = self.parse_item_data(&item_type);

        let new_location = join_paths(vec![&original_location, new_location]);
        let new_location = process_path(&new_location);

        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        if !new_location
            .to_str()
            .unwrap()
            .contains(vault_path.to_str().unwrap())
        {
            panic!("location crosses the bounds of vault")
        }

        move_item(item_type, name, &original_location, &new_location);

        print!("{} {} moved", item_type_full, name)
    }

    pub fn vmove_vault_item(
        &self,
        item_type: &VaultItem,
        name: &String,
        vault_name: &String,
        vault_location: &PathBuf,
    ) {
        let (original_location, item_type, item_type_full) = self.parse_item_data(item_type);

        if vault_name == self.get_name() {
            panic!(
                "{} {} already exists in vault {}",
                item_type_full, name, vault_name
            )
        }

        let new_location = join_paths(vec![vault_location.to_str().unwrap(), vault_name]);
        move_item(item_type, name, &original_location, &new_location);

        print!("{} {} moved to vault {}", item_type_full, name, vault_name)
    }

    fn parse_item_data(&self, item_type: &VaultItem) -> (PathBuf, Item, &str) {
        let (current_vault_name, current_vault_location, folder) = self.get_path_data();
        let location = join_paths(vec![
            current_vault_location,
            &PathBuf::from(current_vault_name),
            folder,
        ]);

        let (item_type, item_type_full) = match item_type {
            VaultItem::Dr => (Item::Dr, "folder"),
            VaultItem::Nt => (Item::Nt, "note"),
        };

        (location, item_type, item_type_full)
    }
}
