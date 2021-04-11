#![cfg(test)]
use crate::{test_sample_tree, Temp, SAMPLE_YAML};
use command_extra::CommandExtra;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
};

/// Name of the directory that stores all compilation artifacts.
const TARGET_DIR: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

/// Run a subcommand of the main command.
fn run_main_subcommand(
    working_directory: &Temp,
    command: &'static str,
    target: &'static str,
    input: &'static str,
) -> (bool, Option<i32>, String, String) {
    let mut child = env!("CARGO_MANIFEST_DIR")
        .pipe(PathBuf::from)
        .parent()
        .expect("parent of $CARGO_MANIFEST_DIR")
        .join("target")
        .join(TARGET_DIR)
        .join("build-fs-tree")
        .pipe(Command::new)
        .with_stdin(Stdio::piped())
        .with_stdout(Stdio::piped())
        .with_stderr(Stdio::piped())
        .with_current_dir(working_directory.as_path())
        .with_arg(command)
        .with_arg(target)
        .spawn()
        .expect("spawn the main command");
    child
        .stdin
        .as_mut()
        .expect("get stdin")
        .write_all(input.as_bytes())
        .expect("write input to stdin");
    let Output {
        status,
        stdout,
        stderr,
    } = child
        .wait_with_output()
        .expect("get the output of the command");
    (
        status.success(),
        status.code(),
        String::from_utf8(stdout).expect("decode stdout as utf-8"),
        String::from_utf8(stderr).expect("decode stdout as utf-8"),
    )
}

#[test]
fn create() {
    let working_directory = Temp::new_dir().expect("create temporary directory");

    eprintln!("FIRST RUN");
    let output = run_main_subcommand(&working_directory, "create", "TARGET", SAMPLE_YAML);
    assert_eq!(output, (true, Some(0), "".to_string(), "".to_string()));
    test_sample_tree(&working_directory.join("TARGET"));

    eprintln!("SECOND RUN");
    let output = run_main_subcommand(&working_directory, "create", "TARGET", SAMPLE_YAML);
    assert_eq!(
        output,
        (
            false,
            Some(1),
            "".to_string(),
            String::from(if cfg!(windows) {
                "create_dir \"TARGET\": Cannot create a file when that file already exists. (os error 183)\n"
            } else {
                "create_dir \"TARGET\": File exists (os error 17)\n"
            }),
        ),
    );
    test_sample_tree(&working_directory.join("TARGET"));
}

#[test]
fn pollute() {
    let working_directory = Temp::new_dir().expect("create temporary directory");

    eprintln!("FIRST RUN");
    let output = run_main_subcommand(&working_directory, "pollute", "TARGET", SAMPLE_YAML);
    assert_eq!(output, (true, Some(0), "".to_string(), "".to_string()));
    test_sample_tree(&working_directory.join("TARGET"));

    eprintln!("SECOND RUN");
    let output = run_main_subcommand(&working_directory, "pollute", "TARGET", SAMPLE_YAML);
    assert_eq!(output, (true, Some(0), "".to_string(), "".to_string()));
    test_sample_tree(&working_directory.join("TARGET"));
}
