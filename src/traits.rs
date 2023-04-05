use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    fs::{create_dir_all, read_to_string, File},
    io::{ErrorKind, Write},
    path::PathBuf,
};

pub trait FileIO: Debug + Default + Serialize + DeserializeOwned {
    fn path(&self) -> PathBuf;

    fn load() -> Self {
        let path = <Self as FileIO>::path(&Self::default());

        <Self as FileIO>::load_path(path)
    }

    fn load_path(path: PathBuf) -> Self {
        match read_to_string(&path) {
            Ok(file_string) => {
                if let Ok(file_data) = toml::from_str::<Self>(&file_string) {
                    file_data
                } else {
                    panic!("couldn't parse data")
                }
            }
            Err(ref err) if err.kind() == ErrorKind::NotFound => {
                <Self as FileIO>::create_file(path)
            }
            Err(_) => panic!("couldn't load file"),
        }
    }

    fn store(&self) {
        let path = <Self as FileIO>::path(self);

        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        <Self as FileIO>::write_file(&mut file, self)
    }

    fn create_file(path: PathBuf) -> Self {
        create_dir_all(path.parent().unwrap()).unwrap();

        let mut file = File::options().create(true).write(true).open(path).unwrap();

        let data = Self::default();

        <Self as FileIO>::write_file(&mut file, &data);

        data
    }

    fn write_file(file: &mut File, data: &Self) {
        let data_string = toml::to_string_pretty(&data).unwrap();

        file.write_all(data_string.as_bytes()).unwrap();
    }
}
