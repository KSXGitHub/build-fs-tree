use crate::BuildError;
use derive_more::{Display, Error, From};
use std::{fmt::Debug, io};

/// Error when execute the [`run`](super::run::run) function.
#[derive(Debug, Display, From, Error)]
pub enum RuntimeError<Path> {
    /// Failed to parse YAML from stdin.
    Yaml(serde_yaml::Error),
    /// Failed to create the filesystem tree.
    Build(BuildError<Path, io::Error>),
}
