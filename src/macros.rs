#![no_implicit_prelude]

/// Create representation of a [directory](crate::FileSystemTree::Directory).
///
/// **NOTES:**
/// * **Syntax:** The syntax used by this macro is similar to the syntax used by
///   [maplit](https://docs.rs/maplit/1.0.2/maplit/) except that in this macro, commas are
///   optional.
/// * **Typings:** The types of `Path` and `FileContent` generic parameter isn't required to be
///   the types provided by the expressions that users wrote as long as they implement
///   [`From<X>`](::std::convert::From) where `X` is the types of the aforementioned user
///   provided expressions.
///
/// # Syntax
///
/// The syntax used by this macro is similar to the syntax used by
/// [maplit](https://docs.rs/maplit/1.0.2/maplit/) except that in this macro, commas are optional.
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
///
/// # Typings
///
/// The types of `Path` and `FileContent` generic parameter isn't required to be the types
/// provided by the expressions that users wrote as long as they implement
/// [`From<X>`](::std::convert::From) where `X` is the types of the aforementioned user
/// provided expressions.
///
/// **Example:** Where `Path` is a `String`
///
/// ```
/// use build_fs_tree::{FileSystemTree, dir, file};
///
/// let tree: FileSystemTree<String, &str> = dir! {
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
/// **Example:** Where `Path` is a `PathBuf` and `FileContent` is a `Vec<u8>`
///
/// ```
/// use build_fs_tree::{FileSystemTree, dir, file};
/// use std::path::PathBuf;
///
/// let tree: FileSystemTree<PathBuf, Vec<u8>> = dir! {
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
///         .get(&PathBuf::from("a")).unwrap().file_content().unwrap(),
///     &Vec::from("foo"),
/// );
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get(&PathBuf::from("b")).unwrap().file_content().unwrap(),
///     &Vec::from("bar"),
/// );
/// assert_eq!(
///     tree.dir_content().unwrap()
///         .get(&PathBuf::from("c")).unwrap().dir_content().unwrap()
///         .get(&PathBuf::from("x")).unwrap().file_content().unwrap(),
///     &Vec::from("baz"),
/// );
/// ```
#[macro_export]
macro_rules! dir {
    ($($key:expr => $value:expr $(,)?)*) => {{
        let mut __directory_content = ::std::collections::BTreeMap::new();
        $(
            let _ = ::std::collections::BTreeMap::insert(
                &mut __directory_content,
                ::std::convert::From::from($key),
                $value
            );
        )*
        $crate::FileSystemTree::Directory(__directory_content)
    }};
}

/// Create representation of a [file](crate::FileSystemTree::File).
///
/// # Syntax
///
/// **Example:**
///
/// ```
/// use build_fs_tree::{FileSystemTree, file};
/// let file: FileSystemTree<&str, &str> = file!("CONTENT OF THE FILE");
/// assert_eq!(file, FileSystemTree::File("CONTENT OF THE FILE"));
/// ```
///
/// # Typings
///
/// This macro calls [`From::from`](::std::convert::From::from) under the hood.
///
/// **Example:** Where `FileContent` is a `String`
///
/// ```
/// use build_fs_tree::{FileSystemTree, file};
/// let file: FileSystemTree<&str, String> = file!("CONTENT OF THE FILE");
/// assert_eq!(file, FileSystemTree::File("CONTENT OF THE FILE".to_string()));
/// ```
///
/// **Example:** Where `FileContent` is a `Vec<u8>`
///
/// ```
/// use build_fs_tree::{FileSystemTree, file};
/// let file: FileSystemTree<&str, Vec<u8>> = file!("CONTENT OF THE FILE");
/// assert_eq!(file, FileSystemTree::File("CONTENT OF THE FILE".into()));
/// ```
#[macro_export]
macro_rules! file {
    ($content:expr) => {
        $crate::FileSystemTree::File(::std::convert::From::from($content))
    };
}
