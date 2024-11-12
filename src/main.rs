use std::path::PathBuf;

use clap::Parser;
use cmd::{actions::create_new_imdb_project, cli::Cli, commands::{Commands, ConfigCommand}};
use config::{Config, ConfigBuilder, File, FileFormat, ValueKind};
use models::config::RunConfiguration;
use serde_json::{json, to_value, Value};
use utils::{config::{ensure_config_file, set_defaults}, io::write_to_file, log::setup_logger, string::BooleanIdentifiable};

mod cmd;
mod constants;
mod models;
mod enums;
mod utils;


fn main()
{
    setup_logger().unwrap();

    let home_dir: PathBuf = dirs::home_dir().unwrap_or(PathBuf::from("./"));
    let config_dir: PathBuf = PathBuf::from(home_dir);

    let config_filepath: String = config_dir.join(constants::CONFIG_FILENAME).to_string_lossy().to_string();

    ensure_config_file(&config_filepath);

    let builder: ConfigBuilder<config::builder::DefaultState> = set_defaults(
        Config::builder().add_source(File::new(&config_filepath, FileFormat::Toml)),
        vec![
            ("format", ValueKind::from("imdb")),
        ]
    ).unwrap();

    match builder.build()
    {
        Ok(config) =>
        {
            let cli: Cli = Cli::parse();

            match &cli.command
            {
                Commands::Add(_args) =>
                {
                    
                },
                Commands::New =>
                {
                    create_new_imdb_project();
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
                                let serialized_config: RunConfiguration
                                    = config.try_deserialize::<RunConfiguration>().unwrap();
                                let mut json_config: Value = to_value(&serialized_config).unwrap();

                                if !value.is_bool_string()
                                {
                                    json_config[key] = json!(value);
                                }
                                else
                                {
                                    json_config[key] = json!(value.parse::<bool>().unwrap());
                                }

                                let toml_stringify_config: String = toml::to_string_pretty(&json_config).unwrap();
                                write_to_file(&toml_stringify_config, &config_filepath);
                            }
                        },
                    }
                },
            }
        },
        Err(err) => {
            println!("Failed to initialize configuration ! \n{0}", err)
        }
    }
}
