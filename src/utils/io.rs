use std::{fs::{self, File}, io::{Read, Write}, path::Path};

use serde::de;

pub fn is_path_exist(path: &str) -> bool
{
    return Path::new(path).exists();
}

pub fn create_file_if_not_exist(path: &str)
{
    if !self::is_path_exist(path) {
        File::create(path).unwrap();
    }
}

pub fn create_dir_if_not_exist(path: &str)
{
    if !self::is_path_exist(path) {
        let _ = fs::create_dir(path);
    }
}

pub fn write_to_file(content: &str, path: &str)
{
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn read_from_file<T>(path: &str) -> T
where
    T : de::DeserializeOwned
{
    let mut file = File::open(path).unwrap();
    let mut json_string = String::new();

    file.read_to_string(&mut json_string).unwrap();

    return serde_json::from_str::<T>(&json_string).unwrap();
}