use crate::cli::error;
use colored::Colorize;
use log::info;
use std::cmp::max;

pub fn contains_count(test: &str, query: &Vec<String>) -> Result<u32, error::BFFError> {
    let count = query.into_iter().filter(|q| test.contains(*q)).count() as u32;

    Ok(count)
}

pub fn contains_count_check(
    test: &str,
    query: &Vec<String>,
    target_count: u32,
) -> Result<bool, error::BFFError> {
    info!(
        "checking if string \"{}\" contains any of {:?} at least {} times",
        test, query, target_count
    );

    let count = contains_count(test, query)?;
    if count >= target_count {
        info!("string contains pattern");
        Ok(true)
    } else {
        info!("string does contains pattern");
        Ok(false)
    }
}

pub fn find_pattern_indices(full_string: &str, substrs: &Vec<String>) -> Vec<(usize, usize)> {
    if substrs.is_empty() {
        return vec![];
    }

    let mut indices: Vec<(usize, usize)> = vec![];
    for substr in substrs {
        let matches: Box<[(usize, &str)]> = full_string.match_indices(substr).collect();

        for m in matches {
            indices.push((m.0, m.0 + substr.len()));
        }
    }

    sort_and_merge(indices)
}

fn sort_and_merge(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.0);
    let mut merged = Vec::new();
    let mut current = ranges[0];

    for &range in &ranges[1..] {
        if range.0 <= current.1 {
            current.1 = max(current.1, range.1);
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);
    merged
}

pub fn highlight_substr_plural(s: &str, substrs: &Vec<String>) -> String {
    info!("highlighting patterns: {:?}, in string: \"{}\"", substrs, s);

    let ranges = find_pattern_indices(s, substrs);

    let mut result = String::new();
    let mut last = 0;

    for (start, end) in ranges {
        if start > last {
            result.push_str(&s[last..start]);
        }

        result.push_str(&s[start..end].bright_blue().bold().to_string());
        last = end;
    }

    if last < s.len() {
        result.push_str(&s[last..]);
    }

    result
}

pub fn count_occurrences(haystack: &str, needlestack: &Vec<String>) -> usize {
    info!(
        "counting occurences of {:?} in string: \"{}\"",
        needlestack, haystack
    );

    let mut res: usize = 0;

    for needle in needlestack {
        res = haystack.match_indices(needle).count();
    }

    info!("string contains {} occurences of pattern", res);

    return res;
}
