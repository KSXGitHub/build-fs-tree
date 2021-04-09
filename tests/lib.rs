use build_fs_tree::*;
use maplit::btreemap;

use FileSystemTree::{Directory, File};

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

mod macros;
