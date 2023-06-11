mod error;
mod impl_mergeable_tree;
mod impl_tree;

pub use error::*;

use crate::{Node, NodeContent};
use std::fmt::{Debug, Display};

/// Applying [`FileSystemTree`](crate::FileSystemTree) to the filesystem.
///
/// **Generic parameters:**
/// * `Name`: Identification of a child item.
/// * `Error`: Error type used by the other functions.
pub trait Build<Name, Error>: Node + Sized
where
    Self::DirectoryContent: IntoIterator<Item = (Name, Self)>,
    Error: Display,
{
    /// Build target.
    type BorrowedPath: Debug + ToOwned<Owned = Self::OwnedPath> + ?Sized;
    /// Locations of the items in the filesystem.
    type OwnedPath: Debug + Clone + AsRef<Self::BorrowedPath>;
    /// Add prefix to the root of the tree.
    fn join(prefix: &Self::BorrowedPath, name: &Name) -> Self::OwnedPath;
    /// Write content to a file.
    fn write_file(path: &Self::BorrowedPath, content: &Self::FileContent) -> Result<(), Error>;
    /// Create a directory at root.
    fn create_dir(path: &Self::BorrowedPath) -> Result<(), Error>;

    /// Build the tree into the filesystem.
    fn build<Path>(self, path: Path) -> Result<(), BuildError<Self::OwnedPath, Error>>
    where
        Path: AsRef<Self::BorrowedPath>,
    {
        let path = path.as_ref();

        let children = match self.read() {
            NodeContent::File(content) => {
                return Self::write_file(path, &content).map_err(|error| BuildError {
                    operation: FailedOperation::WriteFile,
                    path: path.to_owned(),
                    error,
                })
            }
            NodeContent::Directory(children) => children,
        };

        Self::create_dir(path).map_err(|error| BuildError {
            operation: FailedOperation::CreateDir,
            path: path.to_owned(),
            error,
        })?;

        for (name, child) in children {
            child.build(Self::join(path, &name))?;
        }

        Ok(())
    }
}
