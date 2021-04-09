#![cfg(test)]
use crate::{sample_tree, temp_workspace, test_sample_tree};
use build_fs_tree::Build;
use std::path::PathBuf;

#[test]
fn string_string() {
    let _temp = temp_workspace().unwrap();
    let target = PathBuf::from("root");
    sample_tree::<String, String>().build(&target).unwrap();
    test_sample_tree(&target);
}
