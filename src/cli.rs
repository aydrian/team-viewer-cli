use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// City to get weather for
    #[arg(short, long)]
    pub city: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a coworker
    Add {
        /// Name of the coworker
        name: String,
        /// City of the coworker
        city: String,
    },
    /// Remove a coworker
    Remove {
        /// Name of the coworker to remove
        name: String,
    },
    /// List all coworkers
    List,
}
