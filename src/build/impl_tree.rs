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
    type Path = PathBuf;

    fn join(prefix: &Self::Path, name: &Name) -> Self::Path {
        prefix.join(name)
    }

    fn write_file(path: &Self::Path, content: &Self::FileContent) -> Result<(), Error> {
        write(path, content)
    }

    fn create_dir(path: &Self::Path) -> Result<(), Error> {
        create_dir(path)
    }
}
