use build_fs_tree::*;
use command_extra::CommandExtra;
use derive_more::{AsRef, Deref};
use maplit::btreemap;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    collections,
    env::temp_dir,
    ffi::OsString,
    fs::{create_dir, read_dir, read_to_string, remove_dir_all},
    io::{Error, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};
use text_block_macros::text_block_fnl;

use FileSystemTree::{Directory, File};

/// Name of the directory that stores all compilation artifacts.
pub const TARGET_DIR: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

/// Absolute path to the directory that stores all compilation artifacts.
pub fn target_dir() -> PathBuf {
    env!("CARGO_MANIFEST_DIR")
        .pipe(PathBuf::from)
        .parent()
        .expect("parent of $CARGO_MANIFEST_DIR")
        .join("target")
        .join(TARGET_DIR)
}

/// The main command
pub fn main_command() -> Command {
    target_dir()
        .join("build-fs-tree")
        .pipe(Command::new)
        .with_stdin(Stdio::piped())
        .with_stdout(Stdio::piped())
        .with_stderr(Stdio::piped())
}

/// Run a subcommand of the main command
pub fn run_main_subcommand(
    working_directory: &Temp,
    command: &'static str,
    target: &'static str,
    input: &'static str,
) -> (bool, Option<i32>, String, String) {
    let mut child = main_command()
        .with_current_dir(working_directory.as_path())
        .with_arg(command)
        .with_arg(target)
        .spawn()
        .expect("spawn the main command");
    child
        .stdin
        .as_mut()
        .expect("get stdin")
        .write_all(input.as_bytes())
        .expect("write input to stdin");
    let Output {
        status,
        stdout,
        stderr,
    } = child
        .wait_with_output()
        .expect("get the output of the command");
    (
        status.success(),
        status.code(),
        String::from_utf8(stdout).expect("decode stdout as utf-8"),
        String::from_utf8(stderr).expect("decode stdout as utf-8"),
    )
}

/// Representation of a temporary filesystem item.
///
/// **NOTE:** Delete this once https://github.com/samgiles/rs-mktemp/issues/8 is resolved.
#[derive(Debug, AsRef, Deref)]
pub struct Temp(PathBuf);

impl Temp {
    /// Create a temporary directory.
    pub fn new_dir() -> Result<Self, Error> {
        let path = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect::<String>()
            .pipe(|name| temp_dir().join(name));
        if path.exists() {
            return Self::new_dir();
        }
        create_dir(&path)?;
        path.pipe(Temp).pipe(Ok)
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let path = &self.0;
        if let Err(error) = remove_dir_all(path) {
            eprintln!("warning: Failed to delete {:?}: {}", path, error);
        }
    }
}

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
                "abc" => dir! {}
                "def" => file!("content of a/def")
            }
            "b" => dir! {
                "foo" => dir! {
                    "bar" => file!("content of b/foo/bar")
                }
            }
        }
    };
}

/// Create a temporary folder.
pub fn create_temp_dir() -> Temp {
    Temp::new_dir().expect("create a temporary directory")
}

/// Create a set of `String` from `str` slices.
#[macro_export]
macro_rules! string_set {
    ($($element:expr),* $(,)?) => {
        ::maplit::btreeset! { $(::std::string::String::from($element)),* }
    };
}

/// List names of children of a directory.
pub fn list_children_names(path: impl AsRef<Path>) -> collections::BTreeSet<String> {
    read_dir(path)
        .expect("read_dir")
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name())
        .map(OsString::into_string)
        .filter_map(Result::ok)
        .collect()
}

/// Read content of a text file.
pub fn read_text_file(path: impl AsRef<Path>) -> String {
    read_to_string(path).expect("read_to_string")
}

/// Assert that a directory has a only has certain children.
#[macro_export]
macro_rules! assert_dir {
    ($path:expr, $expected:expr $(,)?) => {
        match (crate::list_children_names($path), $expected) {
            (actual, expected) => {
                eprintln!("CASE: {} => {}", stringify!($path), stringify!($expected));
                dbg!(&actual, &expected);
                assert_eq!(
                    actual,
                    expected,
                    "{} => {}",
                    stringify!($path),
                    stringify!($expected),
                );
            }
        }
    };
}

/// Assert that content of a file is a certain text.
#[macro_export]
macro_rules! assert_file {
    ($path:expr, $expected:expr $(,)?) => {
        match (crate::read_text_file($path), $expected) {
            (actual, expected) => {
                eprintln!("CASE: {} => {}", stringify!($path), stringify!($expected));
                dbg!(&actual, &expected);
                assert_eq!(
                    actual,
                    expected,
                    "{} => {}",
                    stringify!($path),
                    stringify!($expected),
                );
            }
        }
    };
}

/// Test the structure of an actual filesystem tree
pub fn test_sample_tree(root: &Path) {
    assert_dir!(root, string_set!("a", "b"));
    assert_dir!(root.join("a"), string_set!("abc", "def"));
    assert_dir!(root.join("a").join("abc"), string_set!());
    assert_file!(root.join("a").join("def"), "content of a/def");
    assert_dir!(root.join("b"), string_set!("foo"));
    assert_dir!(root.join("b").join("foo"), string_set!("bar"));
    assert_file!(
        root.join("b").join("foo").join("bar"),
        "content of b/foo/bar",
    );
}

#[cfg(test)]
mod build;
#[cfg(test)]
mod macros;
#[cfg(test)]
mod program;
#[cfg(test)]
mod tree;
#[cfg(test)]
mod yaml;
