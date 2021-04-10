#![no_implicit_prelude]

/// Create representation of a [directory](crate::FileSystemTree::Directory).
///
/// **Example:** Without commas
///
/// ```
/// use build_fs_tree::{FileSystemTree, dir, file};
///
/// let tree: FileSystemTree<&str, &str> = dir! {
///     "a" => file!("foo")
///     "b" => file!("bar")
///     "c" => dir! {
///         "x" => file!("baz")
///     }
/// };
///
/// # dbg!(&tree);
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get("a").unwrap().file_content().unwrap(),
///     &"foo",
/// );
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get("b").unwrap().file_content().unwrap(),
///     &"bar",
/// );
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get("c").unwrap().dir_content().unwrap()
///         .get("x").unwrap().file_content().unwrap(),
///     &"baz",
/// );
/// ```
///
/// **Example:** With commas
///
/// ```
/// use build_fs_tree::{FileSystemTree, dir, file};
///
/// let tree: FileSystemTree<&str, &str> = dir! {
///     "a" => file!("foo"),
///     "b" => file!("bar"),
///     "c" => dir! {
///         "x" => file!("baz"),
///     },
/// };
///
/// # dbg!(&tree);
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get("a").unwrap().file_content().unwrap(),
///     &"foo",
/// );
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get("b").unwrap().file_content().unwrap(),
///     &"bar",
/// );
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get("c").unwrap().dir_content().unwrap()
///         .get("x").unwrap().file_content().unwrap(),
///     &"baz",
/// );
/// ```
#[macro_export]
macro_rules! dir {
    ($($key:expr => $value:expr $(,)?)*) => {{
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
