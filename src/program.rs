//! Components of the CLI programs.

#[cfg(feature = "cli-completions")]
pub mod completions;
pub mod main;

pub use clap;
pub use clap_utilities;
pub use clap_utilities::clap_complete;
