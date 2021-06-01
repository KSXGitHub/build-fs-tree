//! Components of the CLI programs.

#[cfg(feature = "cli-completions")]
pub mod completions;
pub mod main;

pub use structopt;
pub use structopt::clap;
