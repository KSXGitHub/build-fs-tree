#![cfg(test)]
use crate::sample_tree;
use build_fs_tree::FileSystemTree;
use build_fs_tree::{dir, file};
use pretty_assertions::assert_eq;

type Tree = FileSystemTree<&'static str, &'static str>;

#[test]
fn mutate() {
    let mut tree: Tree = sample_tree();
    let path = ["a", "def"];
    let value = || -> Tree {
        dir! {
            "ghi" => file!("content of a/def/ghi")
        }
    };
    *tree.path_mut(&mut path.iter()).unwrap() = value();
    let expected: Tree = dir! {
        "a" => dir! {
            "abc" => dir! {}
            "def" => value()
        }
        "b" => dir! {
            "foo" => dir! {
                "bar" => file!("content of b/foo/bar")
            }
        }
    };
    assert_eq!(tree, expected);
}

#[test]
fn to_nothing() {
    let mut tree: Tree = sample_tree();
    let path = ["a", "def", "not exist"];
    assert_eq!(tree.path_mut(&mut path.iter()), None);
}
