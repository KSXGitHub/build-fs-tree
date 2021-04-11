#! /bin/bash
set -o errexit -o nounset

if [ -z "$RELEASE_TAG" ]; then
	echo '::error::Environment variable RELEASE_TAG is required but missing'
	exit 1
fi

echo "Publishing build-fs-tree@$RELEASE_TAG..."
cd src
exec cargo publish
