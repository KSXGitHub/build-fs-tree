use super::RuntimeError;
use crate::{Build, FileSystemTree, MergeableFileSystemTree, Node};
use pipe_trait::Pipe;
use serde::de::DeserializeOwned;
use serde_yaml::from_reader;
use std::{fmt::Debug, io, path::PathBuf};

/// Read a YAML from stdin and build a filesystem tree.
pub fn run<Tree, Path>(target: &Path) -> Result<(), RuntimeError<Path>>
where
    Tree: Build<Path, io::Error, Path = Path> + Node + DeserializeOwned,
    Tree::DirectoryContent: IntoIterator<Item = (Path, Tree)>,
    Path: Debug + Ord,
{
    io::stdin()
        .pipe(from_reader::<_, Tree>)?
        .build(target)?
        .pipe(Ok)
}

/// Read a YAML from stdin and build a filesystem tree.
pub type Run = fn(&PathBuf) -> Result<(), RuntimeError<PathBuf>>;
/// Read a YAML from stdin and create a new filesystem tree.
pub const CREATE: Run = run::<FileSystemTree<PathBuf, String>, PathBuf>;
/// Read a YAML from stdin and populate the target filesystem tree.
pub const POPULATE: Run = run::<MergeableFileSystemTree<PathBuf, String>, PathBuf>;
