//! Components that make up the program that generates shell completions for the main program.
use super::main::Args;
use structopt_utilities::StructOptUtils;

/// Run the completions generator.
pub fn main() {
    Args::run_completion_generator("build-fs-tree-completions", "build-fs-tree");
}
