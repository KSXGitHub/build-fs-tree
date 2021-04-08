use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Representation of a filesystem which contains only [files](FileSystemTree::File)
/// and [directories](FileSystemTree::Directory).
///
/// The serialization of `FileContent` (content of file) and `BTreeMap<Path, Self>`
/// (children of directory) must not share the same type. That is, `FileContent` must
/// be serialized to things other than a dictionary.
///
/// **Generic parameters:**
/// * `Path`: Reference to a file in the filesystem.
/// * `FileContent`: Content of a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileSystemTree<Path, FileContent>
where
    Path: Ord,
{
    /// Represents a file with its content.
    /// Its YAML representation must not have the same type as [`FileSystemTree::Directory`]'s.
    File(FileContent),
    /// Represents a directory with its children.
    /// It is a set of name-to-subtree mappings.
    /// Its YAML representation must not have the same type as [`FileSystemTree::File`]'s.
    Directory(BTreeMap<Path, Self>),
}

mod methods;

#[cfg(test)]
mod test;
