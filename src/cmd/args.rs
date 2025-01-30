use clap::Parser;

use super::commands::ConfigCommand;

#[derive(Parser)]
pub struct NewArgs {
    /// Project dirname
    pub dirname: Option<String>,
}

#[derive(Parser)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand
}

#[derive(Parser)]
pub struct ConfigGetArgs {
    pub key: String,
}

#[derive(Parser)]
pub struct ConfigSetArgs {
    pub key: String,
    pub value: String
}