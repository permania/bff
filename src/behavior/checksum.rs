use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

use hex;
use log::info;
use sha2::{Digest, Sha256};

use super::cache::CACHE_FILE;
use crate::cli::error::{self, BFFError};

const EMPTY_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub fn gen_checksum() -> Result<String, error::BFFError> {
    info!("generating new checksum from file tree");

    let metadata = std::fs::metadata(".")?;
    let modified = metadata.modified()?;

    let hash = Sha256::digest(system_time_as_bytes(modified).unwrap_or_default());
    let hex_string = hex::encode(hash);

    if hex_string == EMPTY_HASH {
        return Err(error::BFFError::NoBytes);
    }

    Ok(hex_string)
}

pub fn read_checksum() -> Result<String, error::BFFError> {
    info!("reading checksum from cache file");

    if fs::exists(CACHE_FILE)? {
        let file = File::open(CACHE_FILE)?;
        let mut reader = BufReader::new(file);

        let mut line = String::new();
        reader.read_line(&mut line)?;

        return Ok(line.trim().to_string());
    } else {
        info!("cache file doesn't exist");
    }

    Ok(String::new())
}

pub fn check_cache(check: &str, mincache: bool) -> Result<bool, BFFError> {
    let checksum_matches = check == read_checksum()?;

    if std::fs::exists(CACHE_FILE)? {
        let file = File::open(CACHE_FILE)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        lines.next();

        let second_line = match lines.next() {
            Some(line) => line?,
            None => return Ok(false),
        };

        let expected_flag = if mincache { "-" } else { "+" };

        let flag_matches = expected_flag == second_line.trim_end();

        if !flag_matches {
            info!("mincache flag in cache file doesn't match, rebuilding cache")
        }

        Ok(checksum_matches && flag_matches)
    } else {
        Ok(false)
    }
}

fn system_time_as_bytes(time: SystemTime) -> Option<Vec<u8>> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&d.as_secs().to_le_bytes());
        bytes.extend_from_slice(&d.subsec_nanos().to_le_bytes());
        bytes
    })
}
