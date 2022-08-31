pub mod current;
pub mod data;

use crate::{
    traits::FileIO,
    types::{Item, Vault, VaultItem},
    utils::{create_item, join_paths, move_item, process_path, remove_item, rename_item},
};
use core::panic;
use std::path::PathBuf;

use current::CurrentVault;
use data::Data;

#[derive(Debug)]
pub struct Vaults {
    current: Option<CurrentVault>,
    data: Data,
}

impl Vaults {
    pub fn load() -> Self {
        let mut vaults = Vaults {
            current: None,
            data: Data::load(),
        };
        vaults.load_current_vault();
        vaults
    }

    fn load_current_vault(&mut self) {
        self.current = if let Some(current_vault_name) = self.data.get_current_vault() {
            let current_vault_location = self.data.get_vault_location(current_vault_name).unwrap();

            let path = join_paths(vec![
                current_vault_location.to_str().unwrap(),
                current_vault_name,
                ".jot/data",
            ]);

            Some(CurrentVault::load_path(path))
        } else {
            None
        }
    }

    pub fn list(&self) {}

    pub fn ref_current(&self) -> &CurrentVault {
        self.current.as_ref().expect("not inside a vault")
    }

    pub fn create(&mut self, name: &String, location: &PathBuf) {
        if self.data.vault_exists(name) {
            panic!("vault {} already exists", name)
        }

        let location = process_path(location);
        let path = create_item(Item::Vl, name, &location);
        let data_path = join_paths(vec![path.to_str().unwrap(), ".jot/data"]);

        let mut vault = Vault::load_path(data_path);
        vault.set_name(name.to_owned());
        vault.set_location(process_path(&location));
        vault.store();

        self.data.add_vault(name.to_owned(), location);

        print!("vault {} created", name)
    }

    pub fn remove(&mut self, name: &String) {
        if let Some(vault_location) = self.data.get_vault_location(name) {
            remove_item(Item::Vl, name, vault_location);

            if let Some(current_vault_name) = self.data.get_current_vault() {
                if name == current_vault_name {
                    self.data.set_current_vault(None);
                }
            }

            print!("vault {} removed", name);
        } else {
            panic!("vault {} doesn't exist", name);
        }
    }

    pub fn rename(&mut self, name: &String, new_name: &String) {
        if self.data.vault_exists(new_name) {
            panic!("vault named {} already exists", new_name)
        }

        if let Some(vault_location) = self.data.get_vault_location(name) {
            let path = rename_item(Item::Vl, name, new_name, vault_location);
            let data_path = join_paths(vec![path.to_str().unwrap(), ".jot/data"]);

            Vault::load_path(data_path).set_name(new_name.to_owned());
            self.data.rename_vault(name, new_name.to_owned());

            if let Some(current_vault) = self.data.get_current_vault() {
                if name == current_vault {
                    self.data.set_current_vault(Some(new_name.to_owned()));
                }
            }

            print!("vault {} renamed to {}", name, new_name)
        } else {
            panic!("vault {} doesn't exist", name)
        }
    }

    pub fn mov(&mut self, name: &String, new_location: &PathBuf) {
        if let Some(original_location) = self.data.get_vault_location(name) {
            let new_path = move_item(Item::Vl, name, original_location, &new_location);
            let data_path = join_paths(vec![new_path.to_str().unwrap(), ".jot/data"]);

            let new_location = process_path(new_location);
            Vault::load_path(data_path).set_location(new_location.to_owned());
            self.data.set_vault_location(name, new_location);

            print!("vault {} moved", name)
        } else {
            panic!("vault {} doesn't exist", name)
        }
    }

    pub fn move_to_vault(&self, item_type: &VaultItem, name: &String, vault_name: &String) {
        if let Some(vault_location) = self.data.get_vault_location(vault_name) {
            self.ref_current()
                .vmove(item_type, name, vault_name, vault_location)
        } else {
            panic!("vault {} doesn't exist", vault_name)
        }
    }
}
