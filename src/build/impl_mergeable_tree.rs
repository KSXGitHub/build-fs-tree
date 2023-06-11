use crate::{Build, MergeableFileSystemTree};
use std::{
    fs::{create_dir_all, write},
    io::Error,
    path::{Path, PathBuf},
};

impl<Name, FileContent> Build<Name, Error> for MergeableFileSystemTree<Name, FileContent>
where
    Name: AsRef<Path> + Ord,
    FileContent: AsRef<[u8]>,
{
    type BorrowedPath = Path;
    type OwnedPath = PathBuf;

    fn join(prefix: &Self::BorrowedPath, name: &Name) -> Self::OwnedPath {
        prefix.join(name)
    }

    fn write_file(path: &Self::BorrowedPath, content: &Self::FileContent) -> Result<(), Error> {
        if let Some(dir) = path.parent() {
            create_dir_all(dir)?;
        }
        write(path, content)
    }

    fn create_dir(path: &Self::BorrowedPath) -> Result<(), Error> {
        create_dir_all(path)
    }
}
