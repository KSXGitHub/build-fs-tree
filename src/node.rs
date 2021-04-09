use crate::{make_unmergeable_dir_content_mergeable, FileSystemTree, MergeableFileSystemTree};
use pipe_trait::Pipe;
use std::collections::BTreeMap;

/// Node of a filesystem tree.
pub trait Node {
    /// Content of the node if it is a file.
    type FileContent;
    /// Content of the node if it is a directory.
    type DirectoryContent;
    /// Read the content of the node.
    fn read(self) -> NodeContent<Self::FileContent, Self::DirectoryContent>;
}

/// Content of a node in the filesystem tree
///
/// **Generic parameters:**
/// * `FileContent`: Content of the node if it is a file.
/// * `DirectoryContent`: Content of the node if it is a directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeContent<FileContent, DirectoryContent> {
    /// The node is a file.
    File(FileContent),
    /// The node is a directory.
    Directory(DirectoryContent),
}

impl<Path, FileContent> Node for FileSystemTree<Path, FileContent>
where
    Path: Ord,
{
    type FileContent = FileContent;
    type DirectoryContent = BTreeMap<Path, Self>;

    fn read(self) -> NodeContent<FileContent, Self::DirectoryContent> {
        match self {
            FileSystemTree::File(content) => NodeContent::File(content),
            FileSystemTree::Directory(content) => NodeContent::Directory(content),
        }
    }
}
impl<Path, FileContent> Node for MergeableFileSystemTree<Path, FileContent>
where
    Path: Ord,
{
    type FileContent = FileContent;
    type DirectoryContent = BTreeMap<Path, Self>;

    fn read(self) -> NodeContent<FileContent, Self::DirectoryContent> {
        match self.into() {
            FileSystemTree::File(content) => NodeContent::File(content),
            FileSystemTree::Directory(content) => content
                .pipe(make_unmergeable_dir_content_mergeable)
                .pipe(NodeContent::Directory),
        }
    }
}
