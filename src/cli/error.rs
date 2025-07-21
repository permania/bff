use std::{error::Error, io, process};

use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BFFError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("No config file found")]
    NoConfig,

    #[error("No search result found")]
    NoResult,

    #[error("Error deserializing from TOML: {0}")]
    TOMLDeError(#[from] toml::de::Error),

    #[error("Error serializing to msgpack: {0}")]
    RMPEncodeError(#[from] rmp_serde::encode::Error),

    #[error("Error deserializing from msgpack: {0}")]
    RMPDecodeError(#[from] rmp_serde::decode::Error),

    #[error("Wrong number of arguments: {0}")]
    ArgumentCount(u32),

    #[error("Unable to convert system time to bytes")]
    NoBytes,
}

pub fn handle_error(e: BFFError) -> ! {
    let code = e
        .source()
        .and_then(|source| source.downcast_ref::<std::io::Error>())
        .and_then(|io_err| io_err.raw_os_error())
        .unwrap_or(1);

    error!("{e}");

    process::exit(code);
}
