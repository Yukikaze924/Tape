use clap::Parser;
use colored::Colorize;

use super::commands::Commands;

#[derive(Parser)]
#[command(name = "tape", long_about = format!("\n\nA IMDb cli-tool made with rust. \n\nTo learn more about tape, go to {}", "https://doc.carp.org/tape".cyan()))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}