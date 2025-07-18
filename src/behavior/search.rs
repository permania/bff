use crate::behavior::cache;
use crate::behavior::checksum;
use crate::behavior::strings;
use crate::cli::error;
use itertools::Itertools;
use log::{error, info};

pub fn search(
    query: Vec<String>,
    strict: bool,
    count: u32,
) -> Result<Vec<String>, error::BFFError> {
    info!(
        "begin {} search with terms {:?}",
        if strict == true { "strict" } else { "soft" },
        query
    );

    if query.is_empty() {
        return Err(error::BFFError::ArgumentCount(0));
    }

    let tree = match cache::read_cache_file() {
        Ok(t) => t,
        Err(_) => {
            info!("no cache found");
            cache::get_file_tree()?
        }
    };

    let sum = checksum::gen_checksum()?;
    let old = checksum::read_checksum()?;

    if !checksum::check_checksum(&sum, &old) {
        info!("file tree changed, writing cache file");
        cache::write_cache_file(sum, &tree)?;
    }

    let mut res: Vec<String> = vec![];
    let mut full_match = false;

    for n in (0..=query.len()).rev() {
        for leaf in &tree.files {
            let match_size = largest_matching_subset_size(leaf, &query)?;
            info!(
                "checking file: {}, {} matches, n value of {}",
                leaf, match_size, n
            );

            if match_size == n && n > 0 {
                if strict && n == query.len() {
                    full_match = true;
                }

                // Skip partial matches
                if strict && n != query.len() {
                    continue;
                }

                info!("found file: {}, {} matches", leaf, n);

                res.push(strings::highlight_substr_plural(leaf, &query));

                if res.len() == count as usize {
                    return Ok(res);
                }
            }
        }
    }

    if strict && !full_match {
        return Err(error::BFFError::NoResult);
    }

    if res.is_empty() {
        Err(error::BFFError::NoResult)
    } else {
        Ok(res)
    }
}

pub fn largest_matching_subset_size(
    test: &str,
    query: &Vec<String>,
) -> Result<usize, error::BFFError> {
    for size in (1..=query.len()).rev() {
        for subset in query.iter().combinations(size) {
            if subset.iter().all(|q| test.contains(*q)) {
                return Ok(size);
            }
        }
    }

    Ok(0)
}
