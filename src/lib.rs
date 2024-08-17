//! # [`FileSystemTree`]
//!
//! [`FileSystemTree::build`](Build::build) is faster than
//! [`MergeableFileSystemTree::build`](MergeableFileSystemTree) but it does not write over an existing
//! directory and it does not create parent directories when they don't exist.
//!
//! **Example:**
//!
//! ```no_run
//! use build_fs_tree::{FileSystemTree, Build, dir, file};
//! let tree: FileSystemTree<&str, &str> = dir! {
//!     "index.html" => file!(r#"
//!         <!DOCTYPE html>
//!         <link rel="stylesheet" href="styles/style.css" />
//!         <script src="scripts/main.js"></script>
//!     "#)
//!     "scripts" => dir! {
//!         "main.js" => file!(r#"document.write('Hello World')"#)
//!     }
//!     "styles" => dir! {
//!         "style.css" => file!(r#":root { color: red; }"#)
//!     }
//! };
//! tree.build("public").unwrap();
//! ```
//!
//! # [`MergeableFileSystemTree`]
//!
//! Unlike [`FileSystemTree::build`](FileSystemTree), [`MergeableFileSystemTree::build`](Build::build)
//! can write over an existing directory and create parent directories that were not exist before at the
//! cost of performance.
//!
//! You can convert a `FileSystemTree` into a `MergeableFileSystemTree` via [`From::from`]/[`Into::into`]
//! and vice versa.
//!
//! **Example:**
//!
//! ```no_run
//! use build_fs_tree::{MergeableFileSystemTree, Build, dir, file};
//! let tree = MergeableFileSystemTree::<&str, &str>::from(dir! {
//!     "public" => dir! {
//!         "index.html" => file!(r#"
//!             <!DOCTYPE html>
//!             <link rel="stylesheet" href="styles/style.css" />
//!             <script src="scripts/main.js"></script>
//!         "#)
//!         "scripts/main.js" => file!(r#"document.write('Hello World')"#)
//!         "scripts/style.css" => file!(r#":root { color: red; }"#)
//!     }
//! });
//! tree.build(".").unwrap();
//! ```
//!
//! # Serialization and Deserialization
//!
//! Both [`FileSystemTree`] and [`MergeableFileSystemTree`] implement [`serde::Deserialize`]
//! and [`serde::Serialize`].

#![deny(warnings)]

mod build;
mod macros;
mod node;
mod tree;

pub use build::*;
pub use node::*;
pub use tree::*;

#[cfg(feature = "cli")]
pub mod program;

pub use serde;
pub use serde_yaml;
