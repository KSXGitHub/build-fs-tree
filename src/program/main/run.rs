use super::RuntimeError;
use crate::{Build, FileSystemTree, MergeableFileSystemTree, Node};
use pipe_trait::Pipe;
use serde::de::DeserializeOwned;
use serde_yaml::from_reader;
use std::{
    io,
    path::{Path, PathBuf},
};

/// Read a YAML from stdin and build a filesystem tree.
pub fn run<Tree, Path>(target: &Path) -> Result<(), RuntimeError<Path::Owned>>
where
    Path: ToOwned + AsRef<Path> + ?Sized,
    Path::Owned: AsRef<Path>,
    Tree: Build<Path::Owned, io::Error, BorrowedPath = Path, OwnedPath = Path::Owned>
        + Node
        + DeserializeOwned,
    Tree::DirectoryContent: IntoIterator<Item = (Path::Owned, Tree)>,
    Path: Ord,
{
    io::stdin()
        .pipe(from_reader::<_, Tree>)?
        .build(target)?
        .pipe(Ok)
}

/// Read a YAML from stdin and build a filesystem tree.
pub type Run = fn(&Path) -> Result<(), RuntimeError<PathBuf>>;
/// Read a YAML from stdin and create a new filesystem tree.
pub const CREATE: Run = run::<FileSystemTree<PathBuf, String>, Path>;
/// Read a YAML from stdin and populate the target filesystem tree.
pub const POPULATE: Run = run::<MergeableFileSystemTree<PathBuf, String>, Path>;
