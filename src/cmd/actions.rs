use std::{env, path::PathBuf};

use colored::Colorize;
use dialoguer::{Input, Select};
use log::error;

use crate::{
    enums::imdb::IMDbCategory,
    models::imdb::{IMDbConfiguration, Poster, Source}, utils::io::{create_dir_if_not_exist, create_file_if_not_exist, is_path_exist, write_to_file}
};


pub fn create_new_imdb_project()
{
    let categorys: Vec<String> = vec![
        "Movies".truecolor(255, 140, 0).bold().to_string(),
        "TV Series".truecolor(255, 215, 0).bold().to_string(),
        "Documentaries".truecolor(0, 255, 255).bold().to_string(),
        "Short Films".truecolor(0,191,255).bold().to_string()
    ];

    let category: usize = Select::new()
        .with_prompt("Select the type of IMDb project you want to create")
        .items(&categorys)
        .default(0)
        .interact()
        .unwrap();

    let id: String = Input::new()
        .with_prompt("Enter the IMDbId for this project".bold().white().to_string())
        .interact_text()
        .unwrap();

    let name: String = Input::new()
        .with_prompt("Enter the name".bold().white().to_string())
        .interact_text()
        .unwrap();

    let description: String = Input::new()
        .with_prompt("Enter the IMDb's description".bold().white().to_string())
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let current_dir: PathBuf = env::current_dir().unwrap();
    let target_dirpath: String = current_dir.join(&id).to_string_lossy().to_string();
    drop(current_dir);

    create_dir_if_not_exist(&target_dirpath);

    let imdb_configuration: IMDbConfiguration = IMDbConfiguration {
        id,
        category: IMDbCategory::from_interger(category).to_string(),
        name,
        description,
        casts: Vec::new(),
        directors: Vec::new(),
        writers: Vec::new(),
        poster: Poster {
            source: String::from(""),
            url: String::from("")
        },
        source: Source {
            url: String::from("")
        },
    };

    let imdb_config_filepath: String = PathBuf::from(&target_dirpath).join("imdb.config.json").to_string_lossy().to_string();
    println!("{}",imdb_config_filepath);
    if !is_path_exist(&imdb_config_filepath) {
        create_file_if_not_exist(&imdb_config_filepath);
        let stringify_imdb_config: String = serde_json::to_string_pretty(&imdb_configuration).unwrap();
        write_to_file(&stringify_imdb_config, &imdb_config_filepath);
    }
    else {
        error!(
            "Found existed imdb.config.json under {}!",
                &target_dirpath
        )
    }
}
