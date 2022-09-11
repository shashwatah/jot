use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Item {
    Vl,
    Nt,
    Fd,
}

impl Item {
    pub fn to_vault_item(&self) -> VaultItem {
        match self {
            Item::Vl => VaultItem::Fd,
            Item::Nt => VaultItem::Nt,
            Item::Fd => VaultItem::Fd,
        }
    }

    pub fn fs_name(&self) -> String {
        self.to_vault_item().full()
    }

    pub fn full(&self) -> String {
        match self {
            Item::Vl => "vault".to_string(),
            Item::Nt => "note".to_string(),
            Item::Fd => "folder".to_string(),
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum VaultItem {
    Nt,
    Fd,
}

impl VaultItem {
    pub fn to_item(&self) -> Item {
        match self {
            VaultItem::Nt => Item::Nt,
            VaultItem::Fd => Item::Fd,
        }
    }

    pub fn full(&self) -> String {
        match self {
            VaultItem::Nt => "note".to_string(),
            VaultItem::Fd => "folder".to_string(),
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ConfigType {
    Editor,
    Conflict,
}

impl ConfigType {
    pub fn to_str(&self) -> &str {
        match self {
            ConfigType::Editor => "editor",
            ConfigType::Conflict => "conflict",
        }
    }
}
