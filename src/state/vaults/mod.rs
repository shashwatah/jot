pub mod data;
pub mod vault;

use crate::{
    enums::{Item, VaultItem},
    output::error::Error,
    traits::FileIO,
    utils::{create_item, join_paths, move_item, remove_item, rename_item, resolve_path},
};
use data::Data;
use std::path::Path;
use vault::Vault;

#[derive(Debug)]
pub struct Vaults {
    current: Option<Vault>,
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

            Some(Vault::load_path(path))
        } else {
            None
        }
    }

    pub fn list_vaults(&self, show_loc: &bool) {
        let current_vault_name = self.data.get_current_vault();

        for vault_name in self.data.get_vaults().keys() {
            if current_vault_name.is_some() && vault_name == current_vault_name.unwrap() {
                print!("👉 \x1b[0;34m{}\x1b[0m", vault_name)
            } else {
                print!("   {}", vault_name)
            }

            if *show_loc {
                println!(
                    " \t {}",
                    self.data.get_vault_location(vault_name).unwrap().display()
                )
            } else {
                println!();
            }
        }
    }

    pub fn ref_current(&self) -> Result<&Vault, Error> {
        if self.current.is_none() {
            return Err(Error::NotInsideVault);
        }

        Ok(self.current.as_ref().unwrap())
    }

    pub fn mut_current(&mut self) -> Result<&mut Vault, Error> {
        if self.current.is_none() {
            return Err(Error::NotInsideVault);
        }

        Ok(self.current.as_mut().unwrap())
    }

    pub fn create_vault(&mut self, name: &str, location: &Path) -> Result<(), Error> {
        if self.data.vault_exists(name) {
            return Err(Error::VaultAlreadyExists(name.to_owned()));
        }

        if !location.is_absolute() {
            return Err(Error::PathNotAbsolute);
        }

        let location = resolve_path(location)?;
        let path = create_item(Item::Vl, name, &location)?;
        let data_path = join_paths(vec![path.to_str().unwrap(), ".jot/data"]);

        let mut vault = Vault::load_path(data_path);
        vault.set_name(name.to_owned());
        vault.set_location(location.to_owned());
        vault.store();

        self.data.add_vault(name.to_owned(), location);

        Ok(())
    }

    pub fn remove_vault(&mut self, name: &str) -> Result<(), Error> {
        if let Some(vault_location) = self.data.get_vault_location(name) {
            remove_item(Item::Vl, name, vault_location)?;
            self.data.remove_vault(name);

            if let Some(current_vault_name) = self.data.get_current_vault() {
                if name == current_vault_name {
                    self.data.set_current_vault(None);
                }
            }

            Ok(())
        } else {
            Err(Error::VaultNotFound(name.to_owned()))
        }
    }

    pub fn rename_vault(&mut self, name: &str, new_name: &str) -> Result<(), Error> {
        if self.data.vault_exists(new_name) {
            return Err(Error::VaultAlreadyExists(new_name.to_owned()));
        }

        if let Some(vault_location) = self.data.get_vault_location(name) {
            let path = rename_item(Item::Vl, name, new_name, vault_location)?;
            let data_path = join_paths(vec![path.to_str().unwrap(), ".jot/data"]);

            Vault::load_path(data_path).set_name(new_name.to_owned());
            self.data.rename_vault(name, new_name.to_owned());

            if let Some(current_vault) = self.data.get_current_vault() {
                if name == current_vault {
                    self.data.set_current_vault(Some(new_name.to_owned()));
                }
            }

            Ok(())
        } else {
            Err(Error::VaultNotFound(name.to_owned()))
        }
    }

    pub fn move_vault(&mut self, name: &str, new_location: &Path) -> Result<(), Error> {
        if !new_location.is_absolute() {
            return Err(Error::PathNotAbsolute);
        }

        if let Some(original_location) = self.data.get_vault_location(name) {
            let new_path = move_item(Item::Vl, name, original_location, new_location)?;
            let data_path = join_paths(vec![new_path.to_str().unwrap(), ".jot/data"]);

            let new_location = resolve_path(new_location)?;
            Vault::load_path(data_path).set_location(new_location.to_owned());
            self.data.set_vault_location(name, new_location);

            Ok(())
        } else {
            Err(Error::VaultNotFound(name.to_owned()))
        }
    }

    pub fn move_to_vault(
        &self,
        item_type: &VaultItem,
        name: &str,
        vault_name: &str,
    ) -> Result<(), Error> {
        if let Some(vault_location) = self.data.get_vault_location(vault_name) {
            self.ref_current()?
                .vmove_vault_item(item_type, name, vault_name, vault_location)?;

            Ok(())
        } else {
            Err(Error::VaultNotFound(name.to_owned()))
        }
    }

    pub fn enter_vault(&mut self, name: &str) -> Result<(), Error> {
        if !self.data.vault_exists(name) {
            return Err(Error::VaultNotFound(name.to_owned()));
        }

        if let Some(current_vault_name) = self.data.get_current_vault() {
            if name == current_vault_name {
                return Err(Error::AlreadyInVault(name.to_owned()));
            }
        }

        self.data.set_current_vault(Some(name.to_owned()));

        Ok(())
    }
}
