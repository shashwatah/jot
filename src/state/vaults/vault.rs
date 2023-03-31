use crate::{
    enums::VaultItem,
    output::error::Error,
    traits::FileIO,
    utils::{
        create_item, join_paths, move_item, rec_list, remove_item, rename_item, resolve_path,
        run_editor,
    },
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    name: Option<String>,
    location: Option<PathBuf>,
    folder: PathBuf,
    history: Vec<(String, PathBuf)>,
}

impl Default for Vault {
    fn default() -> Self {
        Vault {
            name: None,
            location: None,
            folder: PathBuf::new(),
            history: vec![],
        }
    }
}

impl FileIO for Vault {
    fn path(&self) -> PathBuf {
        join_paths(vec![
            self.get_location().to_str().unwrap(),
            self.get_name(),
            ".jot/data",
        ])
    }
}

impl Vault {
    pub fn get_name(&self) -> &String {
        self.name.as_ref().unwrap()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
        if self.location.is_some() {
            self.store()
        }
    }

    pub fn get_location(&self) -> &PathBuf {
        self.location.as_ref().unwrap()
    }

    pub fn set_location(&mut self, location: PathBuf) {
        self.location = Some(location);
        if self.name.is_some() {
            self.store()
        }
    }

    pub fn get_folder(&self) -> &PathBuf {
        &self.folder
    }

    pub fn set_folder(&mut self, folder: PathBuf) {
        self.folder = folder;
        self.store()
    }

    pub fn get_path_data(&self) -> (&String, &PathBuf, &PathBuf) {
        (self.get_name(), self.get_location(), self.get_folder())
    }
}

impl Vault {
    pub fn create_vault_item(&self, item_type: VaultItem, name: &str) -> Result<(), Error> {
        let location = self.generate_location();

        create_item(item_type.to_item(), name, &location)?;

        Ok(())
    }

    pub fn remove_vault_item(&self, item_type: VaultItem, name: &str) -> Result<(), Error> {
        let location = self.generate_location();

        remove_item(item_type.to_item(), name, &location)?;

        Ok(())
    }

    pub fn rename_vault_item(
        &self,
        item_type: VaultItem,
        name: &str,
        new_name: &str,
    ) -> Result<(), Error> {
        let location = self.generate_location();

        rename_item(item_type.to_item(), name, new_name, &location)?;

        Ok(())
    }

    pub fn move_vault_item(
        &self,
        item_type: VaultItem,
        name: &str,
        new_location: &PathBuf,
    ) -> Result<(), Error> {
        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        let original_location = join_paths(vec![&vault_path, self.get_folder()]);

        let new_location = resolve_path(&join_paths(vec![&original_location, new_location]))?;

        if !new_location.starts_with(vault_path) {
            return Err(Error::OutOfBounds);
        }

        move_item(item_type.to_item(), name, &original_location, &new_location)?;

        Ok(())
    }

    pub fn vmove_vault_item(
        &self,
        item_type: &VaultItem,
        name: &str,
        vault_name: &str,
        vault_location: &Path,
    ) -> Result<(), Error> {
        let original_location = self.generate_location();

        if vault_name == self.get_name() {
            print!(
                "{} {} already exists in vault {}",
                item_type.full(),
                name,
                vault_name
            )
        }

        let new_location = join_paths(vec![vault_location.to_str().unwrap(), vault_name]);
        move_item(item_type.to_item(), name, &original_location, &new_location)?;

        Ok(())
    }

    pub fn open_note(&self, name: &str, editor_data: (&String, bool)) -> Result<(), Error> {
        let location = self.generate_location();

        run_editor(editor_data, name, &location)?;
        Ok(())
    }

    pub fn change_folder(&mut self, path: &PathBuf) -> Result<(), Error> {
        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        let current_location = resolve_path(&join_paths(vec![&vault_path, self.get_folder()]))?;
        let new_location = resolve_path(&join_paths(vec![&current_location, path]))?;

        if !new_location.starts_with(&vault_path) {
            return Err(Error::OutOfBounds);
        }

        if new_location == current_location {
            return Err(Error::SameLocation);
        }

        let mut destination_folder = new_location.strip_prefix(vault_path).unwrap();
        if destination_folder.has_root() {
            destination_folder = destination_folder.strip_prefix("/").unwrap();
        }
        let destination_folder = destination_folder.to_path_buf();

        self.set_folder(destination_folder);

        Ok(())
    }

    pub fn list(&self) {
        let folder = self.get_folder();

        if folder.as_os_str().is_empty() {
            println!("{}", self.get_name());
        } else {
            println!("{} > {}", self.get_name(), folder.display());
        }

        let location = self.generate_location();
        rec_list(vec![true], location);
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
