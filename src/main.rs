mod behavior;
mod cli;
mod config;
mod parser;

use behavior::{cache::clean, search::run_search};
use clap::Parser;
use cli::error::BFFError;
use cli::{
    arg_parser::{
        BFFArgs,
        BFFCommands::{Clean, Search},
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
        Search(obj) => {
            run_search(obj, conf)?;
        }

        Clean => clean()?,
    }
    Ok(())
}

fn main() -> Result<(), MainError> {
    run().map_err(Into::<MainError>::into)?;
    info!("end execution successfully");
    Ok(())
}
