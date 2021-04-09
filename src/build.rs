mod error;
mod impl_mergeable_tree;
mod impl_tree;

pub use error::*;

use crate::{Node, NodeContent};

/// Applying [`FileSystemTree`](crate::FileSystemTree) to the filesystem.
///
/// **Generic parameters:**
/// * `Name`: Identification of a child item.
/// * `Error`: Error type used by the other functions.
pub trait Build<Name, Error>: Node + Sized
where
    Self::DirectoryContent: IntoIterator<Item = (Name, Self)>,
{
    /// Locations of the items in the filesystem.
    type Path: Clone;
    /// Add prefix to the root of the tree.
    fn join(prefix: &Self::Path, name: &Name) -> Self::Path;
    /// Write content to a file.
    fn write_file(path: &Self::Path, content: &Self::FileContent) -> Result<(), Error>;
    /// Create a directory at root.
    fn create_dir(path: &Self::Path) -> Result<(), Error>;

    /// Build the tree into the filesystem.
    fn build(self, path: &Self::Path) -> Result<(), BuildError<Self::Path, Error>> {
        let children = match self.read() {
            NodeContent::File(content) => {
                return Self::write_file(&path, &content).map_err(|error| BuildError {
                    operation: FailedOperation::WriteFile,
                    path: path.clone(),
                    error,
                })
            }
            NodeContent::Directory(children) => children,
        };

        Self::create_dir(&path).map_err(|error| BuildError {
            operation: FailedOperation::CreateDir,
            path: path.clone(),
            error,
        })?;

        for (name, child) in children {
            child.build(&Self::join(&path, &name))?;
        }

        Ok(())
    }
}
