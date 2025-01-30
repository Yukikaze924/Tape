use std::path::PathBuf;

use clap::Parser;
use cmd::{actions::{add_imdb_package, list_imdb_packages, new_tape_project}, cli::Cli, commands::{Commands, ConfigCommand}};
use config::{Config, ConfigBuilder, File, FileFormat, ValueKind};
use log::error;
use utils::{config::{ensure_config_file, set_defaults}, io::{read_toml, write_to_file}, log::setup_logger, string::BooleanIdentifiable};

mod cmd;
mod constants;
mod models;
mod enums;
mod utils;


#[tokio::main]
async fn main()
{
    setup_logger().unwrap();

    let home_dir: PathBuf = dirs::home_dir().unwrap_or(PathBuf::from("./"));
    let config_dir: PathBuf = PathBuf::from(home_dir);

    let config_file: PathBuf = config_dir.join(constants::CONFIG_FILENAME);

    ensure_config_file(&config_file);

    let builder: ConfigBuilder<config::builder::DefaultState> = set_defaults(
        Config::builder().add_source(File::new(&config_file.to_string_lossy().to_string(), FileFormat::Toml)),
        vec![
            ("format", ValueKind::from("imdb")),
            ("pretty-json", ValueKind::from(true)),
        ]
    ).unwrap();

    match builder.build()
    {
        Ok(config) =>
        {
            let cli: Cli = Cli::parse();

            match &cli.command
            {
                Commands::Add =>
                {
                    add_imdb_package().await;
                },
                Commands::New(args) =>
                {
                    new_tape_project(args).await;
                },
                Commands::Config(config_args) =>
                {
                    match &config_args.command
                    {
                        ConfigCommand::Get(config_get_args) =>
                        {
                            let key: &String = &config_get_args.key;

                            if !key.is_empty()
                            {
                                let result: String = config.get(key).unwrap();
                                println!("{}", result);
                            }
                        },
                        ConfigCommand::Set(config_set_args) =>
                        {
                            let key: &String = &config_set_args.key;
                            let value: &String = &config_set_args.value;

                            if !key.is_empty() && !value.is_empty()
                            {
                                if utils::config::is_valid_field(key)
                                {
                                    let mut map: toml::map::Map<String, toml::Value> = toml::map::Map::new();

                                    if !value.is_bool_string()
                                    {
                                        map.insert(key.to_string(), toml::Value::String(value.to_string()));
                                    }
                                    else
                                    {
                                        map.insert(key.to_string(), toml::Value::Boolean(value.to_string().parse::<bool>().unwrap()));
                                    }

                                    let mut original_config: toml::Value = read_toml::<toml::Value>(&config_file).unwrap();
                                    let original_config_map: &mut toml::map::Map<String, toml::Value> = original_config.as_table_mut().unwrap();
                                    let new_config_map = utils::config::merge_toml_tables(original_config_map, &map);

                                    write_to_file(new_config_map.to_string().as_str(), &config_file).unwrap();
                                }
                                else {
                                    error!("{0} is not found in Configuration.", key);
                                }
                            }
                        },
                    }
                },
                Commands::List =>
                {
                    list_imdb_packages().await
                }
            }
        },
        Err(err) => {
            println!("Failed to initialize configuration ! \n{0}", err)
        }
    }
}
