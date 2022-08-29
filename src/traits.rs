use std::path::PathBuf;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{ErrorKind, Write};
use std::fmt::Debug;
use serde::{Serialize, de::DeserializeOwned};

pub trait FileIO: Debug + Default + Serialize + DeserializeOwned {
    fn path() -> PathBuf;

    fn load() -> Self {
        let path = <Self as FileIO>::path();
     
        match read_to_string(&path) {
            Ok(file_string) => {
                if let Ok(file_data) = toml::from_str::<Self>(&file_string) {
                    return file_data
                } else {
                    panic!("couldn't parse data")
                }
            },
            Err(ref err) if err.kind() == ErrorKind::NotFound => {
                let data = <Self as FileIO>::create(path);           
                return data
            },
            Err(_) => panic!("couldn't load file")
        }
    } 
    
    fn store(&self) {
        let path = <Self as FileIO>::path();

        let mut file = File::options().write(true).truncate(true).open(path).unwrap();

        <Self as FileIO>::write(&mut file, self)
    }

    fn create(path: PathBuf) -> Self {        
        create_dir_all(&path.parent().unwrap()).unwrap();
        
        let mut file = File::options().create(true).write(true).open(path).unwrap();

        let data = Self::default();

        <Self as FileIO>::write(&mut file, &data);
    
        data
    }

    fn write(file: &mut File, data: &Self) {
        print!("{:#?}", data);
        let data_string = toml::to_string_pretty(&data).unwrap();

        file.write_all(data_string.as_bytes()).unwrap();
    } 
}

