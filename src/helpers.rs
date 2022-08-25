use crate::{fs::join_paths, vault::Vault};
use std::path::PathBuf;

pub fn generate_location(vault: &Vault) -> PathBuf {
    let (vault_name, vault_location, folder) = vault.get_path_data();
    join_paths(vec![vault_location, &PathBuf::from(vault_name), folder])
}
