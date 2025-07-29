use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
};

use jwalk::WalkDir;
use log::info;
use rmp_serde::{decode, encode};
use serde::{Deserialize, Serialize};

use crate::{cli::error, config::schema};

pub const CACHE_FILE: &str = ".cache.bff";

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FileTree {
    pub files: Box<[String]>,
}

pub fn write_cache_file(checksum: &str, file_tree: &FileTree) -> Result<(), error::BFFError> {
    info!("writing to cache file");

    let mut file = File::create(CACHE_FILE)?;
    writeln!(file, "{checksum}\n-")?;

    let buf = encode::to_vec(&file_tree)?;
    file.write_all(&buf)?;

    Ok(())
}

pub fn read_cache_file() -> Result<FileTree, error::BFFError> {
    info!("reading from cache file");

    let file = File::open(CACHE_FILE)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();

    for _ in 0..2 {
        reader.read_until(b'\n', &mut buf)?;
    }

    buf.clear();
    reader.read_to_end(&mut buf)?;

    let decoded: FileTree = decode::from_slice(buf.as_slice())?;

    Ok(decoded)
}

pub fn get_file_tree(skip_hidden_dirs: bool) -> Result<FileTree, error::BFFError> {
    info!("building file tree");

    Ok(FileTree {
        files: WalkDir::new(".")
            .skip_hidden(!skip_hidden_dirs)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.file_type().is_file() {
                    let displayed = entry.path().display().to_string();
                    info!("adding path to file tree: {displayed}");
                    Some(displayed)
                } else {
                    None
                }
            })
            .collect::<Box<[String]>>(),
    })
}

pub fn clean() -> Result<(), error::BFFError> {
    info!("cleaning files");

    if fs::exists(CACHE_FILE)? {
        info!("cleaning cache file");
        fs::remove_file(CACHE_FILE)?;
    }

    if fs::exists(schema::CONFIG_FILE)? {
        info!("cleaning config file");
        fs::remove_file(schema::CONFIG_FILE)?;
    }

    Ok(())
}
