use itertools::Itertools;
use log::info;

use crate::behavior::cache;
use crate::behavior::checksum;
use crate::behavior::strings;
use crate::cli::error;

pub fn search(
    query: Vec<String>,
    strict: bool,
    count: u32,
) -> Result<Vec<String>, error::BFFError> {
    info!(
        "begin {} search with terms {:?}",
        if strict { "strict" } else { "soft" },
        query
    );

    let qlen = query.len();

    if query.is_empty() {
        return Err(error::BFFError::ArgumentCount(0));
    }

    let sum = checksum::gen_checksum()?;
    let old = checksum::read_checksum()?;

    let tree = if !checksum::check_checksum(&sum, &old) {
        info!("cache is out of date");
        let tree = cache::get_file_tree()?;
        info!("file tree changed, writing cache file");
        cache::write_cache_file(&sum, &tree)?;
        tree
    } else {
        cache::read_cache_file()?
    };

    let mut res: Vec<String> = vec![];
    let mut full_match = false;

    for n in (0..=qlen).rev() {
        for leaf in &tree.files {
            let match_size = largest_matching_subset_size(leaf, &query)?;
            info!("checking file: {leaf}, {match_size} matches, n value of {n}");

            if match_size == n && n > 0 {
                if strict && n == qlen {
                    full_match = true;
                }

                // Skip partial matches
                if strict && n != qlen {
                    continue;
                }

                info!("found file: {leaf}, {n} matches");

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

#[allow(dead_code)]
pub fn search_in_tree(
    tree: &cache::FileTree,
    query: Vec<String>,
    strict: bool,
    count: u32,
) -> Result<Vec<String>, error::BFFError> {
    let qlen = query.len();

    if query.is_empty() {
        return Err(error::BFFError::ArgumentCount(0));
    }

    let mut res: Vec<String> = vec![];
    let mut full_match = false;

    for n in (0..=qlen).rev() {
        for leaf in &tree.files {
            let match_size = largest_matching_subset_size(leaf, &query)?;
            info!("checking file: {leaf}, {match_size} matches, n value of {n}");

            if match_size == n && n > 0 {
                if strict && n == qlen {
                    full_match = true;
                }

                // Skip partial matches
                if strict && n != qlen {
                    continue;
                }

                info!("found file: {leaf}, {n} matches");

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
    query: &[String],
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
