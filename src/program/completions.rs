//! Components that make up the program that generates shell completions for the main program.
use super::main::Args;
use clap_utilities::CommandFactoryExtra;
use std::process::ExitCode;

/// Run the completions generator.
pub fn main() -> ExitCode {
    Args::run_completion_generator()
}
