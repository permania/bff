use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser, Subcommand,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, styles=STYLES)]
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
    /// Delete hidden files used by bff
    Clean,
}

#[derive(Debug, Parser)]
pub struct SearchArgs {
    /// Keywords to search for
    #[clap()]
    pub terms: Vec<String>,

    /// Only yield results that match all search terms
    #[clap(short, long)]
    pub strict: bool,

    /// The number of results to return (default 1)
    #[clap(short, long)]
    pub count: Option<u32>,

    /// Display a tree view of the resulting path
    #[clap(short, long)]
    pub tree: bool,

    /// Display all possible results (overridden by --count)
    #[clap(short, long)]
    pub all: bool,

    /// Disable skipping hidden directories (can decrease indexing time drastically)
    #[clap(short = 'S', long)]
    pub skip: bool,

    /// Skip writing the cache file, good for not cluttering directories
    #[clap(short = 'C', long)]
    pub no_cache: bool,
}

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::BrightCyan.on_default())
    .placeholder(AnsiColor::BrightWhite.on_default());
