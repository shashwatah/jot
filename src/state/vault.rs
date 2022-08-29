use crate::traits::FileIO;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    name: Option<String>,
    location: Option<PathBuf>,
    folder: PathBuf, 
    history: Vec<(String, PathBuf)>
}

impl Default for Vault {
    fn default() -> Self {
        Vault {
            name: None,
            location: None,
            folder: PathBuf::new(),
            history: vec![]
        }
    }
}

impl FileIO for Vault {
    fn path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(self.get_location());
        path.push(self.get_name());
        path.push(".jot/data");
        path
    }
}

impl Vault {
    pub fn get_name(&self) -> &String {
        self.name.as_ref().unwrap()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
        self.store()
    }

    pub fn get_location(&self) -> &PathBuf {
        self.location.as_ref().unwrap()
    }

    pub fn set_location(&mut self, location: PathBuf) {
        self.location = Some(location);
        self.store()
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