use bff::behavior::cache::FileTree;
use bff::behavior::search::search_in_tree;

#[test]
fn strict_search_test() {
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
