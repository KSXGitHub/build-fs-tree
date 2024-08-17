use crate::{rust_toolchain, rust_version};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use semver::Version;

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
