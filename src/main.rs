mod behavior;
mod cli;
mod config;
mod parser;

use behavior::{cache::clean, search::run_search, tree::path_to_tree};
use clap::Parser;
use cli::error::BFFError;
use cli::{
    arg_parser::{
        BFFArgs,
        BFFCommands::{Clean, Search, Ui},
    },
    error::BFFError::NoConfig,
};
use config::{config_reader::read_config, schema::TreeConfig};
use env_logger::Builder;
use log::{info, warn, LevelFilter};
use main_error::MainError;

fn run() -> Result<(), BFFError> {
    let args = BFFArgs::parse();

    let level = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };
    Builder::new().filter_level(level).init();

    info!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let conf = match read_config() {
        Ok(c) => c,
        Err(NoConfig) => {
            warn!("no config found!");
            TreeConfig::default()
        }
        Err(e) => {
            return Err(e);
        }
    };

    info!("using config: {conf:?}");

    match args.cmd {
        Search(obj) => run_search(obj, conf)?,

        Clean => clean()?,

        Ui => {
            println!(
                "{:?}",
                path_to_tree(
                    r"./incremental/strict-183ylcu1607np/s-h9etbuj4fx-1qoqa9l-2kzrhsgpwf5ey55i1hvdgz4wr/work-products.bin" // r"C:\Users\Alice\Desktop\example.txt"
                                                                                                                           // r"./home/corn/.config/emacs/eln-cache/29.4-576e0a91/lsp-openscad-4f0d4fd1-23d4a5e7.eln"
                )?
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), MainError> {
    run().map_err(Into::<MainError>::into)?;
    info!("end execution successfully");
    Ok(())
}
