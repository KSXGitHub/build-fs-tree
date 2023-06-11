use crate::BuildError;
use derive_more::{Display, Error, From};
use std::{fmt::Debug, io};

#[derive(Debug, Display, From, Error)]
pub enum RuntimeError<Path> {
    Yaml(serde_yaml::Error),
    Build(BuildError<Path, io::Error>),
}
