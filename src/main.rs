mod behavior;
mod cli;
mod config;
mod parser;

use alias_expansion::*;
use cli::*;
use config::*;
use parser::*;

use clap::Parser;
use log::{info, warn, LevelFilter};
use scopeguard::*;

use crate::cli::error::handle_error;

fn main() {
    defer! { info!("end execution successfully") }

    let args = cli::arg_parser::BFFArgs::parse();

    let level = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };
    env_logger::Builder::new().filter_level(level).init();

    info!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let conf = match config_reader::read_config() {
        Ok(c) => c,
        Err(cli::error::BFFError::NoConfig) => {
            warn!("no config found!");
            config::config::TreeConfig::default()
        }
        Err(e) => {
            cli::error::handle_error(e);
        }
    };

    info!("using config: {:?}", conf);

    match args.cmd {
        arg_parser::BFFCommands::Search(obj) => {
            info!("searching for files");

            let expd = obj.terms.expand(&conf);

            info!("before alias expansion: {:?}", obj.terms);
            info!("after alias expansion: {:?}", expd);

            let count = obj
                .count
                .unwrap_or_else(|| if obj.all { u32::MAX } else { 1 });
            match behavior::search::search(expd, obj.strict, count) {
                Ok(ss) => {
                    for s in ss {
                        println!("{s}");
                    }
                }
                Err(e) => handle_error(e),
            }
        }

        arg_parser::BFFCommands::Ui => todo!("implement ui"),
    }
}
