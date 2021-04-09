mod dir_content;

pub use dir_content::*;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Representation of a filesystem which contains only [files](FileSystemTree::File)
/// and [directories](FileSystemTree::Directory).
///
/// The serialization of `FileContent` (content of file) and `BTreeMap<Path, Self>`
/// (children of directory) must not share the same type. That is, `FileContent` must
/// be serialized to things other than a dictionary.
///
/// **Note:** [`FileSystemTree::build`](crate::Build::build) cannot write over an existing
/// directory. Use [`MergeableFileSystemTree`] instead if you desire such behavior.
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
///
/// **Difference from [`FileSystemTree`]:**
/// [`FileSystemTree::build`](crate::Build::build) cannot write over an existing directory.
/// On the other hand, [`MergeableFileSystemTree::build`](crate::Build::build) either merges
/// the two directories if there's no conflict.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsMut, AsRef, Deref, DerefMut, From, Into,
)]
pub struct MergeableFileSystemTree<Path, FileContent>(FileSystemTree<Path, FileContent>)
where
    Path: Ord;

mod methods;
