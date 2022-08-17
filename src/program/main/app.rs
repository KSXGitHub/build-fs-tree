use super::{Args, Command, RuntimeError, CREATE, POPULATE};
use clap::Parser;
use derive_more::{From, Into};
use std::path::PathBuf;

/// The main application.
#[derive(Debug, From, Into)]
pub struct App {
    /// Parse result of CLI arguments.
    pub args: Args,
}

impl App {
    /// Initialize the application from environment parameters.
    pub fn from_env() -> Self {
        Args::parse().into()
    }

    /// Run the application.
    pub fn run(self) -> Result<(), RuntimeError<PathBuf>> {
        match self.args.command {
            Command::Create { target } => CREATE(&target),
            Command::Populate { target } => POPULATE(&target),
        }
    }
}
