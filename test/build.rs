#![cfg(test)]
use crate::{sample_tree, temp_workspace, test_sample_tree};
use build_fs_tree::Build;
use std::path::PathBuf;

macro_rules! test_case {
    ($name:ident, $root:expr, $key:ty, $value:ty $(,)?) => {
        #[test]
        fn $name() {
            let temp = temp_workspace().unwrap();
            let target = PathBuf::from(temp.join($root));
            sample_tree::<$key, $value>().build(&target).unwrap();
            test_sample_tree(&target);
        }
    };
}

test_case!(string_string, "root", String, String);
test_case!(str_slice_string, "root", &'static str, String);
test_case!(string_str_slice, "root", String, &'static str);
test_case!(str_slice_str_slice, "root", &'static str, &'static str);
test_case!(path_buf_str_slice, "root", PathBuf, &'static str);
test_case!(path_buf_u8_vec, "root", PathBuf, ::std::vec::Vec<u8>);
