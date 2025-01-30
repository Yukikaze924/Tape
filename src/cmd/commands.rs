use clap::Subcommand;

use super::args::{NewArgs, ConfigArgs, ConfigGetArgs, ConfigSetArgs};

#[derive(Subcommand)]
pub enum Commands
{
    #[clap(about = "Create a new Tape project")]
    New(NewArgs),

    #[clap(about = "Add a new package into a existing Tape project")]
    Add,

    #[clap(about = "Modify Tape's runtime configuration")]
    Config(ConfigArgs),

    #[clap(alias = "ls")]
    #[clap(about = "List all the package you've installed")]
    List
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    Get(ConfigGetArgs),
    Set(ConfigSetArgs)
}