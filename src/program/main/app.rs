use super::{Args, Command, CREATE, POLLUTE};
use derive_more::{From, Into};
use structopt_utilities::StructOptUtils;

/// The main application.
#[derive(Debug, From, Into)]
pub struct App {
    /// Parse result of CLI arguments.
    pub args: Args,
}

impl App {
    /// Initialize the application from environment parameters.
    pub fn from_env() -> Self {
        Args::strict_from_args().into()
    }

    /// Run the application.
    pub fn run(self) -> Result<(), String> {
        match self.args.command {
            Command::Create { target } => CREATE(&target),
            Command::Pollute { target } => POLLUTE(&target),
        }
    }
}
