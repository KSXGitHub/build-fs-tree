#![no_implicit_prelude]

/// Create representation of a [directory](crate::FileSystemTree::Directory).
#[macro_export]
macro_rules! dir {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut _map = ::std::collections::BTreeMap::new();
        $(
            let _ = ::std::collections::BTreeMap::insert(
                &mut _map,
                ::std::convert::From::from($key),
                $value
            );
        )*
        ::build_fs_tree::FileSystemTree::Directory(_map)
    }};
}

/// Create representation of a [file](crate::FileSystemTree::File).
#[macro_export]
macro_rules! file {
    ($content:expr) => {
        ::build_fs_tree::FileSystemTree::File(::std::convert::From::from($content))
    };
}
