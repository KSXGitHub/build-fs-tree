use crate::FileSystemTree::{self, *};
use std::collections::BTreeMap;

macro_rules! get_content {
    ($variant:ident, $source:expr) => {
        if let $variant(content) = $source {
            Some(content)
        } else {
            None
        }
    };
}

impl<Path, FileContent> FileSystemTree<Path, FileContent>
where
    Path: Ord,
{
    /// Get immutable reference to the file content.
    pub fn file_content(&self) -> Option<&'_ FileContent> {
        get_content!(File, self)
    }

    /// Get immutable reference to the directory content.
    pub fn dir_content(&self) -> Option<&'_ BTreeMap<Path, Self>> {
        get_content!(Directory, self)
    }

    /// Get immutable reference to a descendant of any level.
    pub fn path<'a>(&'a self, path: &'a mut impl Iterator<Item = &'a Path>) -> Option<&'a Self> {
        if let Some(current) = path.next() {
            self.dir_content()?.get(current)?.path(path)
        } else {
            Some(self)
        }
    }

    /// Get mutable reference to the file content.
    pub fn file_content_mut(&mut self) -> Option<&'_ mut FileContent> {
        get_content!(File, self)
    }

    /// Get mutable reference to the directory content.
    pub fn dir_content_mut(&mut self) -> Option<&'_ mut BTreeMap<Path, Self>> {
        get_content!(Directory, self)
    }

    /// Get mutable reference to a descendant of any level.
    pub fn path_mut<'a>(
        &'a mut self,
        path: &'a mut impl Iterator<Item = &'a Path>,
    ) -> Option<&'a mut Self> {
        if let Some(current) = path.next() {
            self.dir_content_mut()?.get_mut(current)?.path_mut(path)
        } else {
            Some(self)
        }
    }
}
