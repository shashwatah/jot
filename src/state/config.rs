use crate::{enums::ConfigType, traits::FileIO};
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
        print!(
            "set {} to {}",
            match config_type {
                ConfigType::Editor => {
                    self.set_editor(value.to_owned());
                    "editor"
                }
                ConfigType::Conflict => {
                    self.set_conflict(value);
                    "conflict"
                }
            },
            value
        );
    }

    pub fn display_config(&self, config_type: &ConfigType) {
        print!(
            "{}",
            match config_type {
                ConfigType::Editor => {
                    self.get_editor()
                }
                ConfigType::Conflict => {
                    match self.get_conflict() {
                        true => "true",
                        false => "false",
                    }
                }
            }
        );
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

    fn set_conflict(&mut self, conflict: &str) {
        self.conflict = conflict.parse().unwrap();
        self.store()
    }
}
