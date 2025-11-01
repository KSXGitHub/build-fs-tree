#! /bin/bash
set -o errexit -o nounset -o pipefail

installer_url=https://sh.rustup.rs
installer_file=$(mktemp -d)/install-rustup

echo "info: Fetching install script from $installer_url to $installer_file..." >&2
curl --proto '=https' --tlsv1.2 -sSf $installer_url >"$installer_file"

echo "info: Executing the install script at $installer_file..." >&2
bash "$installer_file" --default-toolchain "$(cat rust-toolchain)" --component clippy,rustfmt -y
