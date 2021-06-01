#! /bin/bash
set -o errexit -o pipefail -o nounset
exec cargo run --all-features --bin="$1" -- "${@:2}"
