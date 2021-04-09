use crate::sample_tree;
use build_fs_tree::FileSystemTree;
use build_fs_tree::{dir, file};
use pretty_assertions::assert_eq;

type Tree = FileSystemTree<&'static str, &'static str>;

macro_rules! test_path {
    ($name:ident, $path:expr, Some $expected:expr $(,)?) => {
        #[test]
        fn $name() {
            let actual_tree: Tree = sample_tree();
            let path = $path;
            let mut path_iter = path.iter();
            let actual = actual_tree.path(&mut path_iter);
            let expected_tree: Tree = $expected;
            let expected: Option<&Tree> = Some(&expected_tree);
            assert_eq!(actual, expected);
        }
    };

    ($name:ident, $path:expr, None $(,)?) => {
        #[test]
        fn $name() {
            let actual_tree: Tree = sample_tree();
            let path = $path;
            let mut path_iter = path.iter();
            let actual = actual_tree.path(&mut path_iter);
            let expected: Option<&Tree> = None;
            assert_eq!(actual, expected);
        }
    };
}

test_path!(empty_path, [], Some sample_tree());
test_path!(to_a_dir, ["b", "foo"], Some dir! { "bar" => file!("content of b/foo/bar") });
test_path!(to_an_empty_dir, ["a", "abc"], Some dir! {});
test_path!(to_a_file, ["a", "def"], Some file!("content of a/def"));
test_path!(to_nothing_1, ["a", "abc", "not exist"], None);
test_path!(to_nothing_2, ["x"], None);
