use log::info;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

use crate::behavior::checksum;
use crate::cli::error;

pub fn write_cache_file(checksum: String) -> Result<(), error::BFFError> {
    let path = PathBuf::from("./.tree.bffcache");
    let mut contents = format!("{}\n_\n", checksum);

    for leaf in &get_file_tree()? {
        contents.push_str(leaf);
        contents.push('\n');
    }

    fs::write(path, contents)?;
    Ok(())
}

pub fn get_file_tree() -> Result<Box<[String]>, error::BFFError> {
    info!("building file tree");

    Ok(WalkDir::new(".")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() {
                let displayed = entry.path().display().to_string();
                info!("adding path to file tree: {}", displayed);
                Some(displayed)
            } else {
                None
            }
        })
        .collect::<Box<[String]>>())
}
