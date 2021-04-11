#![cfg(test)]
use crate::{run_main_subcommand, test_sample_tree, Temp, SAMPLE_YAML};
use pretty_assertions::assert_eq;

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
