use crate::vault::Vault;
use crate::fs::{create_file, delete_file, join_paths, path_exists, rename_item, move_item};

fn create_note_path(location_data: (&str, &str, &str), name: &str) -> String {
    let (vault_name, vault_location, current_location) = location_data;
    join_paths(vec![vault_location, vault_name, current_location, name])
}

pub fn create_note(name: &str, current_vault: &Vault) {
    let full_path = create_note_path(current_vault.get_location_data(), name);

    if path_exists(&full_path) == false {
        create_file(&full_path);
        print!("note {} created", name)
    } else {
        panic!("note named {} aready exists", name)
    }
}

pub fn rename_note(name: &str, new_name: &str, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data(); 
    
    let path = join_paths(vec![vault_location, vault_name, current_location]);

    if path_exists(&join_paths(vec![&path, new_name])) == true {
       panic!("note named {} already exists", new_name)   
    }

    rename_item(name, new_name, &path);

    println!("note {} renamed to {}", name, new_name)
}

pub fn move_note(name: &str, new_location: &str, current_vault: &Vault) {
    let (vault_name, vault_location, current_location) = current_vault.get_location_data();

    let path = join_paths(vec![vault_location, vault_name, current_location]);
    let new_path = join_paths(vec![&path, new_location]);

    if path_exists(&join_paths(vec![&new_path, name])) {
        panic!("note named {} already exists at new path", name)
    }

    move_item(name, &path, &new_path);
    print!("note {} moved", name)
}

pub fn delete_note(name: &str, current_vault: &Vault ) {
    let full_path = create_note_path(current_vault.get_location_data(), name);
    
    delete_file(&full_path);

    print!("note {} deleted", name)
}



// pub fn open_note() {}
// pub fn movev_note() {}