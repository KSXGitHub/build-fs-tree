#![cfg(test)]
#![no_implicit_prelude]
pub mod utils;
pub use utils::*;

use ::build_fs_tree::{dir, file};

macro_rules! test_case {
    ($name:ident, $key:ty, $value:ty) => {
        #[test]
        fn $name() {
            type Tree = ::build_fs_tree::FileSystemTree<$key, $value>;
            let actual: Tree = sample_tree!();
            let expected: Tree = sample_tree();
            ::std::assert_eq!(actual, expected);
        }
    };
}

test_case!(string_string, ::std::string::String, ::std::string::String);
test_case!(str_slice_string, &'static str, ::std::string::String);
test_case!(string_str_slice, ::std::string::String, &'static str);
test_case!(str_slice_str_slice, &'static str, &'static str);
test_case!(path_buf_str_slice, ::std::path::PathBuf, &'static str);
test_case!(path_buf_u8_vec, ::std::path::PathBuf, ::std::vec::Vec<u8>);
