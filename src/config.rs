use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
enum FileFormat {
    Text, 
    Markdown
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    editor: String,
    format: FileFormat,
    vaults: HashMap<String, String>
}

impl Config {
    pub fn load_config() -> Config {
        Config {
            editor: "nvim".to_string(),
            format: FileFormat::Markdown,
            vaults: HashMap::new()
        }
    }
}
