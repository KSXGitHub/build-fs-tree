use crate::{Build, FileSystemTree, MergeableFileSystemTree, Node};
use pipe_trait::Pipe;
use serde::de::DeserializeOwned;
use serde_yaml::from_reader;
use std::{fmt::Debug, io, path::PathBuf};

/// Read a YAML from stdin and build a filesystem tree.
pub fn run<Tree, Path>(target: &Path) -> Result<(), String>
where
    Tree: Build<Path, io::Error, Path = Path> + Node + DeserializeOwned,
    Tree::DirectoryContent: IntoIterator<Item = (Path, Tree)>,
    Path: Debug + Ord,
{
    io::stdin()
        .pipe(from_reader::<_, Tree>)
        .map_err(|error| format!("error: Read from stdin: {:?}", error))?
        .build(target)
        .map_err(|error| error.to_string())
}

/// Read a YAML from stdin and build a filesystem tree.
pub type Run = fn(&PathBuf) -> Result<(), String>;
/// Read a YAML from stdin and create a new filesystem tree.
pub const CREATE: Run = run::<FileSystemTree<PathBuf, String>, PathBuf>;
/// Read a YAML from stdin and pollute the target filesystem tree.
pub const POLLUTE: Run = run::<MergeableFileSystemTree<PathBuf, String>, PathBuf>;
