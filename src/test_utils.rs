#![cfg(test)]
use mktemp::Temp;
use pipe_trait::Pipe;
use std::{env::set_current_dir, io::Error};

/// Create a temporary folder and set it as working directory.
pub fn temp_workspace() -> Result<(), Error> {
    Temp::new_dir()?.pipe(set_current_dir)
}
