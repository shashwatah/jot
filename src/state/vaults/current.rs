pub use crate::types::Vault as CurrentVault;
use crate::types::VaultItem;

impl CurrentVault {
    pub fn create(&self, item_type: VaultItem, name: &String) {}
}