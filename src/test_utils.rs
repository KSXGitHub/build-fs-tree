#![cfg(test)]
use crate::*;
use maplit::btreemap;
use mktemp::Temp;
use pipe_trait::Pipe;
use std::{env::set_current_dir, io::Error};
use text_block_macros::text_block_fnl;

use FileSystemTree::{Directory, File};

pub const YAML: &str = text_block_fnl! {
    "---"
    "a:"
    "  abc: {}"
    "  def: content of a/def"
    "b:"
    "  foo:"
    "    bar: content of b/foo/bar"
};

pub fn tree<Path, FileContent>() -> FileSystemTree<Path, FileContent>
where
    Path: Ord,
    &'static str: Into<Path> + Into<FileContent>,
{
    Directory(btreemap! {
        "a".into() => Directory(btreemap! {
            "abc".into() => Directory(btreemap! {}),
            "def".into() => File("content of a/def".into()),
        }),
        "b".into() => Directory(btreemap! {
            "foo".into() => Directory(btreemap! {
                "bar".into() => File("content of b/foo/bar".into()),
            }),
        }),
    })
}

/// Create a temporary folder and set it as working directory.
pub fn temp_workspace() -> Result<(), Error> {
    Temp::new_dir()?.pipe(set_current_dir)
}
