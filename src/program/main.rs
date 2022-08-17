//! Components that make up the main program.

mod app;
mod args;
mod error;
mod run;

pub use app::*;
pub use args::*;
pub use error::*;
pub use run::*;

use std::process::ExitCode;

/// The main program.
pub fn main() -> ExitCode {
    match App::from_env().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error_message) => {
            eprintln!("{}", error_message);
            ExitCode::FAILURE
        }
    }
}
