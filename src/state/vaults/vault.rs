use crate::{
    enums::VaultItem,
    output::error::Error,
    traits::FileIO,
    state::config::EditorData,
    utils::{
        create_item, join_paths, move_item, process_path, list_notes, remove_item, rename_item,
        run_editor,
    },
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    name: Option<String>,
    location: Option<PathBuf>,
    folder: PathBuf,
    history: Vec<(String, PathBuf)>,
    aliases: HashMap<String, String>, }

impl Default for Vault {
    fn default() -> Self {
        Vault {
            name: None,
            location: None,
            folder: PathBuf::new(),
            history: vec![],
            aliases: HashMap::new(),
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

    fn name_in_use(&self, new_name: String) -> bool {
        let notes = self.get_notes();

        /*
         * Check if a note is using the given name.
         */
        if notes.contains(&new_name) {
            return true
        }
        
        /*
         * Check if a note alias has the given name.
         */
        for (_note, alias) in self.aliases.iter() {
            if new_name == *alias {
                return true;
            }
        }

        false
    }

    /**
     * Returns the list of all notes stored in this vault.
     * Note: returned notes are file stem and do not have  
     * the .md extension 
     */
    pub fn get_notes(&self) -> Vec<String> {
        let path = self.generate_location();
        let mut notes = vec![];

        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap().path();

            if entry.is_file() && entry.extension().unwrap() == "md" {
                let note_name = entry.file_stem().unwrap().to_str().unwrap().to_string();
                notes.push(note_name);
            }
        }

        notes
    }

    /**
     * Check if the vault contains a note with the 
     * give name.
     */
    fn contains_note(&self, note_name: &String) -> bool {
        self.get_notes().contains(note_name)
    }

    /**
     * Try to retrieve a note from an alias
     */
    fn get_note_from_alias(&self, alias: &String) -> Option<String> {
        for (note, note_alias) in self.aliases.iter() {
            if note_alias == alias {
                return Some(note.clone())
            }
        }

        None
    }
}

impl Vault {
    pub fn set_alias(&mut self, note_name: String, alias_name: String) -> Result<(), Error> {
        if self.name_in_use(alias_name.clone()) {
            return Err(Error::InvalidName)
        }

        self.aliases.insert(note_name, alias_name);
        self.store();

        Ok(())
    }

    pub fn remove_alias_from_note(&mut self, note_name: String) -> Result<String, Error> {

        if self.aliases.contains_key(&note_name) {
            let alias_removed = self.aliases.remove(&note_name).unwrap();
            self.store();

            Ok(alias_removed)
        } else {
            Err(Error::AliasDoesNotExist(note_name))
        }
    }

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

        let new_location = process_path(&join_paths(vec![&original_location, new_location]));

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

    /**
     * Opens a note with the given name, and creates it if it doesn't exist.
     */
    pub fn create_and_open_note(&mut self, name: &str, editor_data: &EditorData) -> Result<(), Error> {
        if !self.contains_note(&name.to_owned()) {
            self.create_vault_item(VaultItem::Nt, name)?;
        }

        self.open_note(name, editor_data)
    }

    pub fn open_note(&self, name_str: &str, editor_data: &EditorData) -> Result<(), Error> {
        let location = self.generate_location();
        let name = name_str.to_string();

        /*
         * Check if the name is a valid note name, then check 
         * if the name is alias to a note. If neither is true, throw
         * an error.
         */
        if self.contains_note(&name) {
            run_editor(editor_data, &name, &location)?;
        } else if let Some(note_name) = self.get_note_from_alias(&name) {
            run_editor(editor_data, &note_name, &location)?;
        } else {
            return Err(Error::InvalidName);
        }

        Ok(())
    }


    pub fn change_folder(&mut self, path: &PathBuf) -> Result<(), Error> {
        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        let new_location = process_path(&join_paths(vec![&vault_path, self.get_folder(), path]));

        if !new_location.exists() {
            return Err(Error::PathNotFound);
        }

        if !new_location.starts_with(&vault_path) {
            return Err(Error::OutOfBounds);
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

        let notes = self.get_notes();

        list_notes(&notes, &self.aliases);
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
