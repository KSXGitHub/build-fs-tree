# build-fs-tree

[![Test](https://github.com/KSXGitHub/build-fs-tree/workflows/Test/badge.svg)](https://github.com/KSXGitHub/build-fs-tree/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/build-fs-tree?logo=rust)](https://crates.io/crates/build-fs-tree)

Generate a filesystem tree from a macro or a YAML tree.

## Description

When I write integration tests, I often find myself needing to create temporary files and directories. Therefore, I created this crate which provides both a library to use in a Rust code and a CLI program that generates a filesystem tree according to a YAML structure.

## Usage Examples

### The Library

Go to [docs.rs](https://docs.rs/build-fs-tree/) for the full API reference.

#### `FileSystemTree`

`FileSystemTree::build` is faster than `MergeableFileSystemTree::build` but it does not write over an existing directory and it does not create parent directories when they don't exist.

```rust
use build_fs_tree::{FileSystemTree, Build, dir, file};
let tree: FileSystemTree<&str, &str> = dir! {
    "index.html" => file!(r#"
        <!DOCTYPE html>
        <link rel="stylesheet" href="styles/style.css" />
        <script src="scripts/main.js"></script>
    "#)
    "scripts" => dir! {
        "main.js" => file!(r#"document.write('Hello World')"#)
    }
    "styles" => dir! {
        "style.css" => file!(r#":root { color: red; }"#)
    }
};
tree.build(&"public".into()).unwrap();
```

#### `MergeableFileSystemTree`

Unlike `FileSystemTree::build`, `MergeableFileSystemTree::build` can write over an existing directory and create parent directories that were not exist before at the cost of performance.

You can convert a `FileSystemTree` into a `MergeableFileSystemTree` via `From::from`/`Into::into` and vice versa.

```rust
use build_fs_tree::{MergeableFileSystemTree, Build, dir, file};
let tree = MergeableFileSystemTree::<&str, &str>::from(dir! {
    "public" => dir! {
        "index.html" => file!(r#"
            <!DOCTYPE html>
            <link rel="stylesheet" href="styles/style.css" />
            <script src="scripts/main.js"></script>
        "#)
        "scripts/main.js" => file!(r#"document.write('Hello World')"#)
        "scripts/style.css" => file!(r#":root { color: red; }"#)
    }
});
tree.build(&".".into()).unwrap();
```

### The Program

The name of the command is `build-fs-tree`. It has 2 subcommands: [`create`](#create) and [`populate`](#populate).

#### `create`

This command reads YAML from stdin and creates a new filesystem tree. It is the CLI equivalent of [`FileSystemTree`](#filesystemtree).

_Create two text files in a new directory:_

```sh
echo '{ foo.txt: HELLO, bar.txt: WORLD }' | build-fs-tree create foo-and-bar
```

_Create a text file and its parent directories:_

```sh
echo '{ text-files: { foo.txt: HELLO } }' | build-fs-tree create files
```

_Create a new filesystem tree from a YAML file:_

```sh
build-fs-tree create root < fs-tree.yaml
```

#### `populate`

This command reads YAML from stdin and either creates a new filesystem tree or add files and directories to an already existing directories. It is the CLI equivalent of [`MergeableFileSystemTree`](#mergeablefilesystemtree).

_Create two text files in the current directory:_

```sh
echo '{ foo.txt: HELLO, bar.txt: WORLD }' | build-fs-tree populate .
```

_Create a text file and its parent directories:_

```sh
echo '{ files/text-files/foo.txt: HELLO }' | build-fs-tree populate .
```

_Populate the current directory with filesystem tree as described in a YAML file:_

```sh
build-fs-tree populate . < fs-tree.yaml
```

## Packaging Status

[![Packaging Status](https://repology.org/badge/vertical-allrepos/build-fs-tree.svg)](https://repology.org/project/build-fs-tree/versions)

## License

[MIT](https://git.io/JOkew) © [Hoàng Văn Khải](https://ksxgithub.github.io/).
