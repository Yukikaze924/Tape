use std::{fs::{self, File}, io::{Read, Write}, path::Path};
use serde::de;
use tokio;

pub fn is_path_exist(path: &Path) -> bool {
    path.exists()
}

pub async fn is_path_exist_async(path: &Path) -> bool {
    tokio::fs::try_exists(path).await.unwrap_or(false)
}

pub fn create_file_if_not_exist(path: &Path) -> std::io::Result<()> {
    if !is_path_exist(path) {
        File::create(path)?;
    }
    Ok(())
}

pub async fn create_file_if_not_exist_async(path: &Path) -> std::io::Result<()> {
    if !is_path_exist_async(path).await {
        tokio::fs::File::create(path).await?;
    }
    Ok(())
}

pub fn create_dir_if_not_exist(path: &Path) -> std::io::Result<()> {
    if !is_path_exist(path) {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub async fn create_dir_if_not_exist_async(path: &Path) -> std::io::Result<()> {
    if !is_path_exist_async(path).await {
        tokio::fs::create_dir_all(path).await?;
    }
    Ok(())
}

pub fn write_to_file(content: &str, path: &Path) -> std::io::Result<()> {
    let mut file: File = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn write_to_file_append(content: &str, filepath: &Path) -> std::io::Result<()> {
    let mut file: File = fs::OpenOptions::new()
        .append(true)
        .open(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_toml<T>(path: &Path) -> Result<T, Box<dyn std::error::Error>>
where
    T : de::DeserializeOwned
{
    let mut file: File = File::open(path)?;
    let mut _string: String = String::new();
    file.read_to_string(&mut _string)?;
    let data: T = toml::from_str::<T>(&_string)?;
    Ok(data)
}
