#! /bin/bash
set -o errexit -o pipefail

if [ "$FMT_UPDATE" = 'true' ]; then
  cargo_fmt_flag=()
elif [ "$FMT_UPDATE" = 'false' ] || [ "$FMT_UPDATE" = '' ]; then
  cargo_fmt_flag=('--check')
fi

exec cargo fmt -- "${cargo_fmt_flag[@]}"
