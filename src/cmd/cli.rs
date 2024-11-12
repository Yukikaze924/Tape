use clap::Parser;

use super::commands::Commands;

#[derive(Parser)]
#[command(name = "demo", about = "A demo CLI with subcommands", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}