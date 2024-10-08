use derive_more::{Display, Error};
use std::fmt::{self, Debug, Formatter};

/// Error caused by [`Build::build`](crate::Build::build).
#[derive(Debug, Display, Error)]
#[display("{operation} {path:?}: {error}")]
#[display(bound(Path: Debug, Error: Display))]
pub struct BuildError<Path, Error> {
    /// Operation that caused the error.
    pub operation: FailedOperation,
    /// Path where the error occurred.
    pub path: Path,
    /// The error.
    pub error: Error,
}

/// Operation that causes an error.
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

impl fmt::Display for FailedOperation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.name())
    }
}
