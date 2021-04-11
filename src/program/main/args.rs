use std::path::PathBuf;
use structopt::StructOpt;
use text_block_macros::text_block;

/// Parse result of CLI arguments.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "build-fs-tree",

    long_about = text_block! {
        "Create a filesystem tree from YAML"
        ""
        "Source: https://github.com/KSXGitHub/build-fs-tree"
        "Issues: https://github.com/KSXGitHub/build-fs-tree/issues"
        "Donate: https://patreon.com/khai96_"
    },

    after_help = text_block! {
        "EXAMPLES:"
        "    Create two text files in a new directory"
        "    $ echo '{ foo.txt: HELLO, bar.txt: WORLD }' | build-fs-tree create foo-and-bar"
        ""
        "    Create two text files in the current directory"
        "    $ echo '{ foo.txt: HELLO, bar.txt: WORLD }' | build-fs-tree pollute ."
        ""
        "    Create a new filesystem tree from a YAML file"
        "    $ build-fs-tree create root < fs-tree.yaml"
        ""
        "    Pollute the current directory with filesystem tree as described in a YAML file"
        "    $ build-fs-tree pollute . < fs-tree.yaml"
    },
)]
pub struct Args {
    /// Command to execute.
    #[structopt(subcommand)]
    pub command: Command,
}

/// Subcommands of the program.
#[derive(Debug, StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    about = "Create a filesystem tree from YAML"
)]
pub enum Command {
    /// Invoke [`FileSystemTree::build`](crate::FileSystemTree).
    #[structopt(
        about = concat!(
            "Read YAML from stdin and create a new filesystem tree at <TARGET>. ",
            "Merged paths are not allowed",
        ),

        after_help = text_block! {
            "EXAMPLES:"
            "    Create two text files in a new directory"
            "    $ echo '{ foo.txt: HELLO, bar.txt: WORLD }' | build-fs-tree create foo-and-bar"
            ""
            "    Create a text file and its parent directories"
            "    $ echo '{ text-files: { foo.txt: HELLO } }' | build-fs-tree create files"
            ""
            "    Create a new filesystem tree from a YAML file"
            "    $ build-fs-tree create root < fs-tree.yaml"
        },
    )]
    Create {
        #[structopt(name = "TARGET")]
        target: PathBuf,
    },

    /// Invoke [`MergeableFileSystemTree::build`](crate::MergeableFileSystemTree).
    #[structopt(
        about = concat!(
            "Read YAML from stdin and pollute an existing filesystem tree at <TARGET>. ",
            "Parent directories would be created if they are not already exist",
        ),

        after_help = text_block! {
            "EXAMPLES:"
            "    Create two text files in the current directory"
            "    $ echo '{ foo.txt: HELLO, bar.txt: WORLD }' | build-fs-tree pollute ."
            ""
            "    Create a text file and its parent directories"
            "    $ echo '{ files/text-files/foo.txt: HELLO }' | build-fs-tree pollute ."
            ""
            "    Pollute the current directory with filesystem tree as described in a YAML file"
            "    $ build-fs-tree pollute . < fs-tree.yaml"
        },
    )]
    Pollute {
        #[structopt(name = "TARGET")]
        target: PathBuf,
    },
}
