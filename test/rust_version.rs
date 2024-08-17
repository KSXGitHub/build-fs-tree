use crate::{RUST_TOOLCHAIN, RUST_VERSION};
use pretty_assertions::assert_eq;

#[test]
fn rust_version_matches_rust_toolchain() {
    let rust_toolchain = &*RUST_TOOLCHAIN;
    dbg!(rust_toolchain);
    let rust_version = &*RUST_VERSION;
    dbg!(rust_version);

    let toolchain_without_patch = format!("{}.{}", rust_toolchain.major, rust_toolchain.minor);
    dbg!(&toolchain_without_patch);

    assert_eq!(&toolchain_without_patch, rust_version);
}
