use std::path::{
    Component::{CurDir, Normal, ParentDir, Prefix, RootDir},
    Path,
};

use ascii_tree::{
    write_tree,
    Tree::{self, Leaf, Node},
};

use crate::cli::error::BFFError;

pub fn path_to_tree(path: &str) -> Result<String, BFFError> {
    let path = Path::new(path);
    let mut res: Vec<&str> = vec![];
    for c in path.components() {
        match c {
            CurDir => res.push("."),
            Normal(p) => res.push(p.to_str().ok_or(BFFError::NoUTF8)?),
            Prefix(_) => todo!("if you reach this, leave an issue! this feature isn't properly implemented for Windows yet."),
            RootDir => unreachable!("if you reached this you found an unknown bug! leave an issue. bug code 0"),
            ParentDir => unreachable!("if you reached this you found an unknown bug! leave an issue. bug code 1"),
        }
    }

    let tree = build_tree_path(&res);

    let mut buf = String::new();
    write_tree(&mut buf, &tree)?;

    Ok(trim_lines(&buf))
}

fn trim_lines(s: &str) -> String {
    let mut buf = String::new();
    for line in s.lines().skip(1) {
        buf.push_str(line.char_indices().nth(1).map_or("", |(i, _)| &line[i..]));
        buf.push('\n');
    }
    buf
}

fn build_tree_path(path: &[&str]) -> Tree {
    if path.is_empty() {
        unreachable!("this shouldn't happen! leave an issue if it does")
    } else if path.len() == 1 {
        Leaf(vec![path[0].to_string()])
    } else {
        Node(path[0].to_string(), vec![build_tree_path(&path[1..])])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_segment() {
        let path = "foo";
        let result = path_to_tree(path).unwrap();
        println!("{result}");
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_segment_path() {
        let path = "./bar/baz";
        let result = path_to_tree(path).unwrap();
        let expected = r"└─ bar
   └─ baz
";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dot_path() {
        let path = ".";
        let result = path_to_tree(path).unwrap();
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "this shouldn't happen! leave an issue if it does")]
    fn test_empty_path() {
        let path = "";
        let result = path_to_tree(path).unwrap();
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "this shouldn't happen! leave an issue if it does")]
    fn test_build_tree_path_empty() {
        let _ = build_tree_path(&[]);
    }
}
