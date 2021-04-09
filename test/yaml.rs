#![cfg(test)]
use crate::{sample_tree, SAMPLE_YAML};
use build_fs_tree::{FileSystemTree, MergeableFileSystemTree};
use pipe_trait::Pipe;
use serde_yaml::{from_str, to_string, Value};

type Tree = FileSystemTree<String, String>;
type MergeableTree = MergeableFileSystemTree<String, String>;

#[test]
fn serialize() {
    let actual: Tree = from_str(SAMPLE_YAML).expect("parse YAML as FileSystemTree");
    let expected: Tree = sample_tree();
    dbg!(&actual, &expected);
    assert_eq!(actual, expected);
}

#[test]
fn deserialize() {
    let actual = sample_tree()
        .pipe(|x: Tree| x)
        .pipe_ref(to_string)
        .expect("stringify a FileSystemTree as YAML");
    let expected = SAMPLE_YAML;
    eprintln!("\nACTUAL:\n{}\n", &actual);
    eprintln!("\nEXPECTED:\n{}\n", expected);
    macro_rules! parse {
        ($yaml:expr) => {
            from_str::<Value>($yaml).expect(concat!(
                "parse ",
                stringify!($yaml),
                " as serde_yaml::Value",
            ))
        };
    }
    assert_eq!(parse!(&actual), parse!(expected));
}

#[test]
fn serialize_mergeable() {
    let actual: MergeableTree =
        from_str(SAMPLE_YAML).expect("parse YAML as MergeableFileSystemTree");
    let expected: MergeableTree = sample_tree().into();
    dbg!(&actual, &expected);
    assert_eq!(actual, expected);
}

#[test]
fn deserialize_mergeable() {
    let actual = sample_tree()
        .pipe(MergeableTree::from)
        .pipe_ref(to_string)
        .expect("stringify a MergeableFileSystemTree as YAML");
    let expected = SAMPLE_YAML;
    eprintln!("\nACTUAL:\n{}\n", &actual);
    eprintln!("\nEXPECTED:\n{}\n", expected);
    macro_rules! parse {
        ($yaml:expr) => {
            from_str::<Value>($yaml).expect(concat!(
                "parse ",
                stringify!($yaml),
                " as serde_yaml::Value",
            ))
        };
    }
    assert_eq!(parse!(&actual), parse!(expected));
}
