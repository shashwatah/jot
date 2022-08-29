use crate::traits::FileIO;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    editor: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "nvim".to_string(),
        }
    }
}

impl FileIO for Config {
    fn path(&self) -> PathBuf {
        if let Some(dirs) = ProjectDirs::from("com", "", "jot") {
            let mut path = dirs.config_dir().to_path_buf();
            path.push("config");
            path
        } else {
            panic!("config path couldn't be generated")
        }
    }
}

impl Config {
    pub fn get_editor(&self) -> &String {
        &self.editor
    }

    pub fn set_editor(&mut self, editor: String) {
        self.editor = editor;
        self.store()
    }
}
