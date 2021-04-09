#![cfg(test)]
use crate::*;
use maplit::btreemap;
use mktemp::Temp;
use pipe_trait::Pipe;
use std::{env::set_current_dir, io::Error};
use text_block_macros::text_block_fnl;

use FileSystemTree::{Directory, File};

pub type Tree = FileSystemTree<String, String>;

pub const YAML: &str = text_block_fnl! {
    "---"
    "a:"
    "  abc: {}"
    "  def: content of a/def"
    "b:"
    "  foo:"
    "    bar: content of b/foo/bar"
};

pub fn tree() -> Tree {
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

/// Create a temporary folder and set it as working directory.
pub fn temp_workspace() -> Result<(), Error> {
    Temp::new_dir()?.pipe(set_current_dir)
}
