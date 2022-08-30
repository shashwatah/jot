pub mod current;
pub mod data;

use crate::{traits::FileIO, types::{Item, Vault}, utils::{create_item, join_paths, process_path}};
use std::path::PathBuf;

pub use current::CurrentVault;
use data::Data;

#[derive(Debug)]
pub struct Vaults {
    current: Option<CurrentVault>,
    data: Data
}

impl Vaults {
    pub fn load() -> Self {
        let mut vaults = Vaults {
            current: None,
            data: Data::load()
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

    pub fn ref_current(&self) -> &CurrentVault {
        self.current.as_ref().expect("not inside a vault")
    }

    pub fn create(&mut self, name: &String, location: &PathBuf) {
        if self.data.vault_exists(name) {
            panic!("vault {} already exists", name)
        }

        let path = create_item(Item::Vl, name, location);

        let data_path = join_paths(vec![path.to_str().unwrap(), ".jot/data"]); 

        let mut vault = Vault::load_path(data_path);
        vault.set_name(name.to_owned());
        vault.set_location(process_path(location));
        vault.store();

        self.data.add_vault(name.to_owned(), location.to_owned());

        print!("vault {} created", name)
    }

    pub fn list(&self) {}
    
    fn remove_vault(&self) {}
    fn rename_vault(&self) {}
    fn move_vault(&self) {}
}