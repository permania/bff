mod behavior;
mod cli;
mod config;
mod parser;

use behavior::{cache::clean, search::search};
use clap::Parser;
use cli::{
    arg_parser::{BFFArgs, BFFCommands::*},
    error::{handle_error, BFFError::*},
};
use config::{config_reader::read_config, schema::TreeConfig};
use env_logger::Builder;
use log::{info, warn, LevelFilter};
use parser::alias_expansion::ExpandAlias;
use scopeguard::defer;

fn main() {
    defer! { info!("end execution successfully") }

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
            handle_error(e);
        }
    };

    info!("using config: {conf:?}");

    match args.cmd {
        Search(obj) => {
            info!("searching for files");

            let expd = obj.terms.expand(&conf);

            info!("before alias expansion: {:?}", obj.terms);
            info!("after alias expansion: {expd:?}");

            let count = obj.count.unwrap_or(if obj.all { u32::MAX } else { 1 });
            match search(expd, obj.strict, count) {
                Ok(ss) => {
                    for s in ss {
                        println!("{s}");
                    }
                }
                Err(e) => handle_error(e),
            }
        }

        Clean => {
            if let Err(e) = clean() {
                handle_error(e)
            }
        }

        Ui => todo!("implement ui"),
    }
}
