use cargo_toml::Manifest;
use command_extra::CommandExtra;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use semver::Version;
use std::{fs, path::PathBuf, process::Command, str};

fn workspace_manifest() -> PathBuf {
    env!("CARGO")
        .pipe(Command::new)
        .with_arg("locate-project")
        .with_arg("--workspace")
        .with_arg("--message-format=plain")
        .output()
        .expect("cargo locate-project")
        .stdout
        .pipe_as_ref(str::from_utf8)
        .expect("convert stdout to UTF-8")
        .trim()
        .pipe(PathBuf::from)
}

fn rust_toolchain() -> String {
    workspace_manifest()
        .parent()
        .expect("get workspace dir")
        .join("rust-toolchain")
        .pipe(fs::read_to_string)
        .expect("read rust-toolchain")
}

fn rust_version() -> String {
    workspace_manifest()
        .parent()
        .expect("get workspace dir")
        .join("src")
        .join("Cargo.toml")
        .pipe(Manifest::from_path)
        .expect("load src/Cargo.toml")
        .package
        .expect("read package")
        .rust_version
        .expect("read rust_version")
        .get()
        .expect("read rust_version as string")
        .to_string()
}

#[test]
fn rust_version_matches_rust_toolchain() {
    let rust_toolchain = rust_toolchain()
        .trim()
        .pipe(Version::parse)
        .expect("parse rust-toolchain as semver");
    dbg!(&rust_toolchain);
    let rust_version = rust_version().trim().to_string();
    dbg!(&rust_version);

    let toolchain_without_patch = format!("{}.{}", rust_toolchain.major, rust_toolchain.minor);
    dbg!(&toolchain_without_patch);

    assert_eq!(&toolchain_without_patch, &rust_version);
}
