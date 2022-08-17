use crate::BuildError;
use derive_more::From;
use std::{fmt::Debug, io};
use thiserror::Error;

#[derive(Debug, From, Error)]
pub enum RuntimeError<Path: Debug> {
    #[error("{}", _0)]
    Yaml(serde_yaml::Error),
    #[error("{}", _0)]
    Build(BuildError<Path, io::Error>),
}
