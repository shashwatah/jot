pub use crate::types::Vault as CurrentVault;
use crate::{
    types::VaultItem,
    utils::{create_item, join_paths, move_item, process_path, remove_item, rename_item},
};
use std::path::PathBuf;

impl CurrentVault {
    pub fn folder(&self) {
        print!("{}:{}", self.get_name(), self.get_folder().display());
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
        let original_location = self.generate_location();

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

    fn generate_location(&self) -> PathBuf {
        let (current_vault_name, current_vault_location, folder) = self.get_path_data();
        join_paths(vec![
            current_vault_location,
            &PathBuf::from(current_vault_name),
            folder,
        ])
    }
}
