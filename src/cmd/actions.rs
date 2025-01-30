use std::{env, path::PathBuf};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use log::{error, warn};
use tokio::fs::{create_dir, File};

use crate::{
    enums::imdb::{IMDbCategory, MPAAClassification}, models::imdb::{IMDbConfiguration, Poster, Source, Stream}, utils::io::{create_dir_if_not_exist_async, create_file_if_not_exist_async, is_path_exist, is_path_exist_async, write_to_file}
};

use super::args::NewArgs;


pub async fn new_tape_project(args: &NewArgs)
{
    let current_dir: PathBuf = env::current_dir().unwrap();

    let project_dir: PathBuf;
    if let Some(dirname) = &args.dirname {
        project_dir = current_dir.join(dirname);
        let _ = create_dir(project_dir.to_str().unwrap()).await;
    }
    else {
        project_dir = current_dir;
    }

    let tape_config_file: PathBuf = project_dir.join("tape.json");

    if is_path_exist(&tape_config_file) {
        warn!("Found existing project");
        warn!("at {}", tape_config_file.to_string_lossy().to_string().cyan());
        return;
    }
    else {
        let _ = File::create(&tape_config_file).await;
    }

    for i in 0..IMDbCategory::count() {
        let _ = create_dir_if_not_exist_async(&project_dir.join(IMDbCategory::from_int(i).to_string())).await;
    }

    println!("\n{}", "Project created successfully!".bold());
    if let Some(dirname) = &args.dirname {
        println!("\n{}: \n{}\n{}\n",
        "To start managing your project".bold(),
        format!("   cd {}", dirname),
        "   tape <COMMAND> <OPTION>");
    }
    else {
        println!("\n{}: \n{}\n",
        "To start managing your project",
        "   tape <COMMAND> <OPTION>");
    }
}

pub async fn add_imdb_package()
{
    let current_dir: PathBuf = env::current_dir().unwrap();
    let tape_config_file: PathBuf = current_dir.join("tape.json");

    if !is_path_exist_async(&tape_config_file).await {
        error!("Invalid Tape project directory!");
        error!("Missing {} at {}", "tape.json".cyan(), current_dir.to_string_lossy().to_string().cyan());
        return;
    }

    let category: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of IMDb package you want to create")
        .items(&IMDbCategory::get_color_list())
        .default(0)
        .interact()
        .unwrap();
    let category: String = IMDbCategory::from_int(category).to_string();

    let id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("ID")
        .interact_text()
        .unwrap();

    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let rating: String = Input::with_theme(&ColorfulTheme::default())
        .with_initial_text::<String>(0.to_string())
        .with_prompt("Rating(0-10)")
        .allow_empty(true)
        .default(0.to_string())
        .show_default(false)
        .interact_text()
        .unwrap();
    let rating: f32 = rating.parse::<f32>().unwrap_or_else(|_err: std::num::ParseFloatError| panic!("Using core::str::parse::<f32>() in non-float string.\n Rating must be a number that meets the IMDb rating system.") );
    if rating > 10.0 || rating < 0.0 {
        error!("Invalid rating! IMDb's rating system is a floating number within the range of 0 ~ 10.");
        error!("{} {} {}", "Go to", "https://www.imdb.com/list/ls076459507/".cyan(), "for more information.");
        return;
    }

    let classification: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Classification")
        .items(&MPAAClassification::to_string_array())
        .default(0)
        .interact()
        .unwrap();

    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Description")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let target_dir: PathBuf = current_dir.join(&category).join(&id);

    create_dir_if_not_exist_async(&target_dir).await.unwrap();

    let imdb_configuration: IMDbConfiguration = IMDbConfiguration {
        id,
        _type: 0,
        title,
        category,
        rating,
        classification: MPAAClassification::from_integer(classification).to_string(),
        interests: vec![],
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
        stream: Stream {
            mp4: Source { url: "".to_string() },
        },
        tracks: vec![],
    };

    let imdb_config_filepath: PathBuf = PathBuf::from(&target_dir).join("imdb.config.json");
    println!("{}",imdb_config_filepath.to_string_lossy().to_string());
    if !is_path_exist_async(&imdb_config_filepath).await {
        let _ = create_file_if_not_exist_async(&imdb_config_filepath).await;
        let stringify_imdb_config: String = serde_json::to_string_pretty(&imdb_configuration).unwrap();
        let _ = write_to_file(&stringify_imdb_config, &imdb_config_filepath);
    }
    else {
        error!(
            "Found existed imdb.config.json under {}!", &target_dir.to_string_lossy().to_string()
        )
    }
}

pub async fn list_imdb_packages()
{
    // 创建选项列表
    let options = vec!["Javascript", "Typescript"];

    // 创建选择提示
    let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Which one do you prefer (Javascript/Typescript)?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Failed to get user selection");

    // 获取用户选择的选项
    println!("You selected: {}", options[selection]);
}
