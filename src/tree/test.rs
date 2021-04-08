#![cfg(test)]

use crate::*;
use maplit::btreemap;
use pipe_trait::Pipe;
use serde_yaml::{from_str, to_string, Value};

use FileSystemTree::{Directory, File};

type Tree = FileSystemTree<String, String>;

const YAML: &str = concat! {
    "---\n",
    "a:\n",
    "  abc: {}\n",
    "  def: content of a/def\n",
    "b:\n",
    "  foo:\n",
    "    bar: content of b/foo/bar\n",
};

fn tree() -> Tree {
    Directory(btreemap! {
        "a".to_string() => Directory(btreemap! {
            "abc".to_string() => Directory(btreemap! {}),
            "def".to_string() => File("content of a/def".to_string()),
        }),
        "b".to_string() => Directory(btreemap! {
            "foo".to_string() => Directory(btreemap! {
                "bar".to_string() => File("content of b/foo/bar".to_string()),
            }),
        }),
    })
}

#[test]
fn serialize() {
    let actual: Tree = from_str(YAML).expect("parse YAML as FileSystemTree");
    let expected = tree();
    dbg!(&actual, &expected);
    assert_eq!(actual, expected);
}

#[test]
fn deserialize() {
    let actual = tree()
        .pipe_ref(to_string)
        .expect("stringify a FileSystemTree as YAML");
    let expected = YAML;
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
