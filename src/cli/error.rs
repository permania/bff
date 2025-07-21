use std::{fmt, io};

use log::error;
use rmp_serde::{decode, encode};
use thiserror::Error;
use toml::de;

#[derive(Debug, Error)]
pub enum BFFError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("No config file found")]
    NoConfig,

    #[error("No search result found")]
    NoResult,

    #[error("Error deserializing from TOML: {0}")]
    TOMLDeError(#[from] de::Error),

    #[error("Error serializing to msgpack: {0}")]
    RMPEncodeError(#[from] encode::Error),

    #[error("Error deserializing from msgpack: {0}")]
    RMPDecodeError(#[from] decode::Error),

    #[error("Wrong number of arguments: {0}")]
    ArgumentCount(u32),

    #[error("Unable to convert system time to bytes")]
    NoBytes,

    #[error("File path contains invalid UTF-8")]
    NoUTF8,

    #[error("Failure formatting: {0}")]
    FormatError(#[from] fmt::Error),
}
