use clap::Subcommand;

use super::args::{AddArgs, ConfigArgs, ConfigGetArgs, ConfigSetArgs};

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a new item
    Add(AddArgs),
    New,
    Config(ConfigArgs),
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    Get(ConfigGetArgs),
    Set(ConfigSetArgs)
}