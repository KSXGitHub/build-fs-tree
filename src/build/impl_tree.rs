use crate::{Build, FileSystemTree};
use std::{
    fs::{create_dir, write},
    io::Error,
    path::{Path, PathBuf},
};

impl<Name, FileContent> Build<Name, Error> for FileSystemTree<Name, FileContent>
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
        write(path, content)
    }

    fn create_dir(path: &Self::BorrowedPath) -> Result<(), Error> {
        create_dir(path)
    }
}
