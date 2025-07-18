use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct BFFArgs {
    /// The command to use
    #[clap(subcommand)]
    pub cmd: BFFCommands,

    /// Toggle verbose logging mode
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum BFFCommands {
    /// Search for files using the CLI
    Search(SearchArgs),
    /// Find files using the TUI file browser
    Ui,
}

#[derive(Debug, Parser)]
pub struct SearchArgs {
    /// Keywords to search for, in order of directory
    #[clap()]
    pub terms: Vec<String>,

    /// Only yield results that match all search terms
    #[clap(short, long)]
    pub strict: bool,

    /// The number of results to return (default 1)
    #[clap(short, long)]
    pub count: Option<u32>,

    /// Display all possible results (overridden by --count)
    #[clap(short, long)]
    pub all: bool,
}
