use crate::{FileSystemTree, MergeableFileSystemTree};
use std::{collections::BTreeMap, mem::transmute};

macro_rules! map_type {
    ($(#[$attribute:meta])* $name:ident = $tree:ident) => {
        $(#[$attribute])*
        pub type $name<Path, FileContent> = BTreeMap<Path, $tree<Path, FileContent>>;
    };
}

map_type!(
    #[doc = "Directory content of [`FileSystemTree`]."]
    DirectoryContent = FileSystemTree
);

map_type!(
    #[doc = "Directory content of [`MergeableFileSystemTree`]."]
    MergeableDirectoryContent = MergeableFileSystemTree
);

macro_rules! function {
    ($(#[$attribute:meta])* $name:ident :: $input:ident -> $output:ident) => {
        $(#[$attribute])*
        pub fn $name<Path, FileContent>(map: $input<Path, FileContent>) -> $output<Path, FileContent>
        where
            Path: Ord,
        {
            unsafe { transmute(map) }
        }
    };
}

function!(
    #[doc = "Transmute a [`DirectoryContent`] into a [`MergeableDirectoryContent`]."]
    make_unmergeable_dir_content_mergeable :: DirectoryContent -> MergeableDirectoryContent
);

function!(
    #[doc = "Transmute a [`MergeableDirectoryContent`] into a [`DirectoryContent`]."]
    make_mergeable_dir_content_unmergeable :: MergeableDirectoryContent -> DirectoryContent
);
