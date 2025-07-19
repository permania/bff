use bff::behavior::cache::FileTree;
use bff::behavior::search::search_in_tree;
use bff::cli::error;
use rand::Rng;

#[test]
fn stress_test_search_random_file() {
    let mut files = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        files.push(format!("file_{:07}.txt", i));
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..files.len());
    let random_file = files[random_index].clone();

    let tree = FileTree {
        files: files.into_boxed_slice(),
    };

    let query = vec![random_file.clone()];
    let result = search_in_tree(&tree, query, false, 1);

    match result {
        Ok(results) => {
            assert!(!results.is_empty(), "Expected some matches but got none");
            assert!(
                results.iter().any(|r| r.contains(&random_file)),
                "Results did not include the searched file"
            );
        }
        Err(e) => {
            if let error::BFFError::NoResult = e {
                panic!("Random file search returned no results unexpectedly");
            } else {
                panic!("Search failed unexpectedly: {:?}", e);
            }
        }
    }
}
