#![cfg(test)]
use crate::{assert_dir, assert_file, create_temp_dir, sample_tree, string_set, test_sample_tree};
use build_fs_tree::{dir, file, Build, MergeableFileSystemTree};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

type MergeableTree = MergeableFileSystemTree<&'static str, &'static str>;

macro_rules! test_case {
    ($name:ident, $root:expr, $key:ty, $value:ty $(,)?) => {
        #[test]
        fn $name() {
            let temp = create_temp_dir();
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

/// Error message when attempting to create a directory at location of a file.
const DIR_ON_FILE_ERROR_SUFFIX: &str = if cfg!(windows) {
    "Cannot create a file when that file already exists. (os error 183)"
} else {
    "File exists (os error 17)"
};

/// Error message when attempting to create a file at location of a directory.
const FILE_ON_DIR_ERROR_SUFFIX: &str = if cfg!(windows) {
    "Access is denied. (os error 5)"
} else {
    "Is a directory (os error 21)"
};

#[test]
fn unmergeable_build() {
    let temp = create_temp_dir();
    let target = temp.join("root");
    sample_tree::<&str, &str>()
        .build(&target)
        .expect("build for the first time");
    test_sample_tree(&target);
    let actual_error = sample_tree::<&str, &str>()
        .build(&target)
        .expect_err("build for the second time")
        .to_string();
    let expected_error = format!("create_dir {:?}: {}", &target, DIR_ON_FILE_ERROR_SUFFIX);
    assert_eq!(actual_error, expected_error);
}

#[test]
fn mergeable_build() {
    let temp = create_temp_dir();
    let target = temp.join("root");
    sample_tree::<&str, &str>()
        .build(&target)
        .expect("build for the first time");
    test_sample_tree(&target);
    sample_tree::<&str, &str>()
        .pipe(MergeableTree::from)
        .build(&target)
        .expect("build for the second time");
    test_sample_tree(&target);
    MergeableTree::from(dir! {
        "a" => dir! {
            "ghi" => dir! {
                "0" => dir! {},
                "1" => file!("content of a/ghi/1"),
            },
        },
        "z" => dir! {
            "x" => dir! {
                "c" => file!("content of z/x/c"),
            },
        },
    })
    .build(&target)
    .expect("build for the third time: add some items");
    eprintln!("Check new files...");
    assert_dir!(&target, string_set!("a", "b", "z"));
    assert_dir!(target.join("a"), string_set!("abc", "def", "ghi"));
    assert_dir!(target.join("a").join("ghi"), string_set!("0", "1"));
    assert_dir!(target.join("a").join("ghi").join("0"), string_set!());
    assert_file!(target.join("a").join("ghi").join("1"), "content of a/ghi/1");
    assert_dir!(target.join("z"), string_set!("x"));
    assert_dir!(target.join("z").join("x"), string_set!("c"));
    assert_file!(target.join("z").join("x").join("c"), "content of z/x/c");
    eprintln!("Check old files...");
    assert_dir!(target.join("a").join("abc"), string_set!());
    assert_file!(target.join("a").join("def"), "content of a/def");
    assert_dir!(target.join("b"), string_set!("foo"));
    assert_dir!(target.join("b").join("foo"), string_set!("bar"));
    assert_file!(
        target.join("b").join("foo").join("bar"),
        "content of b/foo/bar",
    );
}

#[test]
fn mergeable_build_conflict_file_on_dir() {
    let temp = create_temp_dir();
    let target = temp.join("root");
    sample_tree::<&str, &str>()
        .build(&target)
        .expect("build for the first time");
    test_sample_tree(&target);
    let actual_error = MergeableTree::from(dir! {
        "a" => file!("should not exist"),
    })
    .build(&target)
    .expect_err("build for the second time")
    .to_string();
    let expected_error = format!(
        "write_file {:?}: {}",
        target.join("a"),
        FILE_ON_DIR_ERROR_SUFFIX,
    );
    assert_eq!(actual_error, expected_error);
}

#[test]
fn mergeable_build_conflict_dir_on_file() {
    let temp = create_temp_dir();
    let target = temp.join("root");
    sample_tree::<&str, &str>()
        .build(&target)
        .expect("build for the first time");
    test_sample_tree(&target);
    let actual_error = MergeableTree::from(dir! {
        "a" => dir! {
            "def" => dir! {
                "b" => file!("should not exist"),
            },
        },
    })
    .build(&target)
    .expect_err("build for the second time")
    .to_string();
    let expected_error = format!(
        "create_dir {:?}: {}",
        target.join("a").join("def"),
        DIR_ON_FILE_ERROR_SUFFIX,
    );
    assert_eq!(actual_error, expected_error);
}
#[test]
fn mergeable_build_ensure_dir_to_write_file() {
    let temp = create_temp_dir();
    MergeableTree::from(dir! {
        "a/b/c" => file!("a/b/c"),
        "d/e/f" => dir! {
            "foo" => file!("d/e/f/foo"),
            "bar/baz" => file!("d/e/f/bar/baz"),
        },
    })
    .build(&temp)
    .expect("build filesystem tree");
    assert_file!(temp.join("a").join("b").join("c"), "a/b/c");
    assert_file!(temp.join("d").join("e").join("f").join("foo"), "d/e/f/foo");
    assert_file!(
        temp.join("d").join("e").join("f").join("bar").join("baz"),
        "d/e/f/bar/baz",
    );
}
