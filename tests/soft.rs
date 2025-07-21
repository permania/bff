use bff::behavior::cache::FileTree;
use bff::behavior::search::{self, search_in_tree};

#[test]
fn soft_search_test() {
    let files: Box<[String]> = r#"
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
    .map(|s| s.to_string())
    .collect::<Box<[String]>>();

    let tree: FileTree = FileTree { files };
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
