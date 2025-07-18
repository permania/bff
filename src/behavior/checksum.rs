use hex;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::error;

const EMPTY_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub fn gen_checksum() -> Result<String, error::BFFError> {
    let metadata = std::fs::metadata(".")?;
    let modified = metadata.modified()?;

    let hash = Sha256::digest(system_time_as_bytes(modified).unwrap_or(vec![]));
    let hex_string = hex::encode(hash);

    if hex_string == EMPTY_HASH {
        return Err(error::BFFError::NoBytes);
    }

    Ok(hex_string)
}

pub fn check_checksum(check: &str, sum: &str) -> bool {
    check == sum
}

fn system_time_as_bytes(time: SystemTime) -> Option<Vec<u8>> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&d.as_secs().to_le_bytes());
        bytes.extend_from_slice(&d.subsec_nanos().to_le_bytes());
        bytes
    })
}
