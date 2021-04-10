//! Components that make up the main program.

mod app;
mod args;
mod run;

pub use app::*;
pub use args::*;
pub use run::*;

pub fn main() -> ! {
    std::process::exit(match App::from_env().run() {
        Ok(()) => 0,
        Err(error_message) => {
            eprintln!("{}", error_message);
            1
        }
    })
}
