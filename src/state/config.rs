use crate::{enums::ConfigType, output::error::Error, traits::FileIO, utils::run_editor};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    editor: String,
    conflict: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "nvim".to_string(),
            conflict: true,
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
    pub fn get_editor_data(&self) -> (&String, bool) {
        (&self.editor, self.conflict)
    }

    pub fn set_config(&mut self, config_type: &ConfigType, value: &String) {
        match config_type {
            ConfigType::Editor => self.set_editor(value.to_owned()),
            ConfigType::Conflict => self.set_conflict(value.to_owned()),
        }
    }

    pub fn get_config(&self, config_type: &ConfigType) -> String {
        match config_type {
            ConfigType::Editor => self.get_editor().to_owned(),
            ConfigType::Conflict => match self.get_conflict() {
                true => "true".to_string(),
                false => "false".to_string(),
            },
        }
    }

    pub fn open_config(&self) -> Result<(), Error> {
        run_editor(self.get_editor_data(), &self.path())?;
        Ok(())
    }

    fn get_editor(&self) -> &String {
        &self.editor
    }

    fn set_editor(&mut self, editor: String) {
        self.editor = editor;
        self.store()
    }

    fn get_conflict(&self) -> &bool {
        &self.conflict
    }

    fn set_conflict(&mut self, conflict: String) {
        self.conflict = conflict.parse().unwrap();
        self.store()
    }
}
