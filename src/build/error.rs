use std::fmt::{self, Debug, Display, Formatter};
use thiserror::Error;

/// Error caused by [`Build::build`](crate::Build::build).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("{} {:?}: {}", operation, path, error)]
pub struct BuildError<Path: Debug, Error: Display> {
    /// Operation that caused the error.
    pub operation: FailedOperation,
    /// Path where the error occurred.
    pub path: Path,
    /// The error.
    pub error: Error,
}

/// Operation that causes an error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailedOperation {
    /// The operation was to write a file.
    WriteFile,
    /// The operation was to create a directory.
    CreateDir,
}

impl FailedOperation {
    /// Convert to a string.
    pub const fn name(self) -> &'static str {
        use FailedOperation::*;
        match self {
            WriteFile => "write_file",
            CreateDir => "create_dir",
        }
    }
}

impl Display for FailedOperation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.name())
    }
}
