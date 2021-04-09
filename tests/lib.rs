use build_fs_tree::*;
use maplit::btreemap;
use mktemp::Temp;
use pipe_trait::Pipe;
use std::{env::set_current_dir, io::Error};
use text_block_macros::text_block_fnl;

use FileSystemTree::{Directory, File};

/// Create a YAML representation of a sample tree.
pub const SAMPLE_YAML: &str = text_block_fnl! {
    "---"
    "a:"
    "  abc: {}"
    "  def: content of a/def"
    "b:"
    "  foo:"
    "    bar: content of b/foo/bar"
};

/// Create a sample tree.
pub fn sample_tree<Path, FileContent>() -> FileSystemTree<Path, FileContent>
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

/// Create a sample tree (but with `dir!` and `file!` macros).
#[macro_export]
macro_rules! sample_tree {
    () => {
        dir! {
            "a" => dir! {
                "abc" => dir! {},
                "def" => file!("content of a/def"),
            },
            "b" => dir! {
                "foo" => dir! {
                    "bar" => file!("content of b/foo/bar"),
                },
            },
        }
    };
}

/// Create a temporary folder and set it as working directory.
pub fn temp_workspace() -> Result<(), Error> {
    Temp::new_dir()?.pipe(set_current_dir)
}

mod macros;
mod tree;
