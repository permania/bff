use std::cmp::max;

use colored::Colorize;
use log::info;

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
    info!("highlighting patterns: {substrs:?}, in string: \"{s}\"");

    let ranges = find_pattern_indices(s, substrs);

    let mut result = String::new();
    let mut last = 0;

    for (start, end) in ranges {
        if start > last {
            result.push_str(&s[last..start]);
        }

        result.push_str(&s[start..end].yellow().bold().to_string());
        last = end;
    }

    if last < s.len() {
        result.push_str(&s[last..]);
    }

    result
}
