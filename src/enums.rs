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

    pub fn fs_name(&self) -> &str {
        match self {
            Item::Vl => "folder",
            Item::Nt => "note",
            Item::Fd => "folder",
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

    pub fn full(&self) -> &str {
        match self {
            VaultItem::Nt => "note",
            VaultItem::Fd => "folder",
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ConfigType {
    Editor,
    Conflict,
}
