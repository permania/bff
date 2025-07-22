use itertools::Itertools;
use log::info;

use super::tree::path_to_tree;
use crate::behavior::cache;
use crate::behavior::checksum;
use crate::behavior::strings;
use crate::cli::arg_parser::SearchArgs;
use crate::cli::error::BFFError::{self, ArgumentCount, NoResult};
use crate::config::schema::TreeConfig;
use crate::parser::alias_expansion::ExpandAlias;

pub fn search(query: Vec<String>, strict: bool, count: u32) -> Result<Vec<String>, BFFError> {
    info!(
        "begin {} search with terms {:?}",
        if strict { "strict" } else { "soft" },
        query
    );

    let qlen = query.len();

    if query.is_empty() {
        return Err(ArgumentCount(0));
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
        return Err(NoResult);
    }

    if res.is_empty() {
        Err(NoResult)
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
) -> Result<Vec<String>, BFFError> {
    let qlen = query.len();

    if query.is_empty() {
        return Err(ArgumentCount(0));
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
        return Err(NoResult);
    }

    if res.is_empty() {
        Err(NoResult)
    } else {
        Ok(res)
    }
}

pub fn largest_matching_subset_size(test: &str, query: &[String]) -> Result<usize, BFFError> {
    for size in (1..=query.len()).rev() {
        for subset in query.iter().combinations(size) {
            if subset.iter().all(|q| test.contains(*q)) {
                return Ok(size);
            }
        }
    }

    Ok(0)
}

pub fn run_search(obj: SearchArgs, conf: TreeConfig) -> Result<(), BFFError> {
    info!("searching for files");

    let expd = obj.terms.expand(conf);

    info!("before alias expansion: {:?}", obj.terms);
    info!("after alias expansion: {expd:?}");

    let count = obj.count.unwrap_or(if obj.all { u32::MAX } else { 1 });
    let ss = search(expd, obj.strict, count)?;
    for s in ss {
        println!("{s}");
        if obj.tree {
            println!("{}", path_to_tree(&s)?);
        };
    }

    Ok(())
}

#[cfg(test)]
mod search_tests {
    use once_cell::sync::Lazy;

    use super::search_in_tree;
    use crate::behavior::cache::FileTree;

    static FILES: Lazy<Box<[String]>> = Lazy::new(|| {
        r#"
/docs/report_final.docx
/docs/project_notes.txt
/docs/meeting_minutes_07-15.pdf
/docs/draft_v2.docx
/docs/readme.md
/spreadsheets/budget_2024.xlsx
/spreadsheets/data.csv
/presentations/presentation.pptx
/presentations/logo_design.ai
/images/photo_001.jpg
/scripts/script.sh
/scripts/app.py
/backups/archive_backup.zip
/backups/backup_2023.tar.gz
/notes/todo_list.md
/notes/notes_personal.txt
/invoices/invoice_1234.pdf
/logs/error_log_20250718.log
/results/results_final.json
"#
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Box<[String]>>()
    });

    #[test]
    fn strict_search() {
        let tree: FileTree = FileTree {
            files: FILES.clone(),
        };
        let r = search_in_tree(
            &tree,
            vec!["t".to_string(), "s".to_string(), "o".to_string()],
            true,
            3,
        )
        .unwrap();

        let p: Vec<String> = vec![
	    "/d\u{1b}[1;33mo\u{1b}[0mc\u{1b}[1;33ms\u{1b}[0m/rep\u{1b}[1;33mo\u{1b}[0mr\u{1b}[1;33mt\u{1b}[0m_final.d\u{1b}[1;33mo\u{1b}[0mcx".to_string(),
	    "/d\u{1b}[1;33mo\u{1b}[0mc\u{1b}[1;33ms\u{1b}[0m/pr\u{1b}[1;33mo\u{1b}[0mjec\u{1b}[1;33mt\u{1b}[0m_n\u{1b}[1;33mot\u{1b}[0me\u{1b}[1;33ms\u{1b}[0m.\u{1b}[1;33mt\u{1b}[0mx\u{1b}[1;33mt\u{1b}[0m".to_string(),
	    "/d\u{1b}[1;33mo\u{1b}[0mc\u{1b}[1;33ms\u{1b}[0m/mee\u{1b}[1;33mt\u{1b}[0ming_minu\u{1b}[1;33mt\u{1b}[0me\u{1b}[1;33ms\u{1b}[0m_07-15.pdf".to_string(),
	];

        assert_eq!(r, p);
    }

    #[test]
    fn soft_search() {
        let tree: FileTree = FileTree {
            files: FILES.clone(),
        };
        let r = search_in_tree(
            &tree,
            vec![
                "final".to_string(),
                "back".to_string(),
                "docs".to_string(),
                "read".to_string(),
            ],
            false,
            3,
        )
        .unwrap();

        let p: Vec<String> = vec![
            "/\u{1b}[1;33mdocs\u{1b}[0m/report_\u{1b}[1;33mfinal\u{1b}[0m.docx".to_string(),
            "/\u{1b}[1;33mdocs\u{1b}[0m/\u{1b}[1;33mread\u{1b}[0mme.md".to_string(),
            "/\u{1b}[1;33mdocs\u{1b}[0m/project_notes.txt".to_string(),
        ];

        assert_eq!(r, p);
    }
}
