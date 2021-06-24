#!/bin/bash
export DIST="$1"
export SOURCE_ROOT="$2"

cd "$SOURCE_ROOT"
mkdir "$DIST"/.cargo
cargo vendor | sed 's/^directory = ".*"/directory = "vendor"/g' > $DIST/.cargo/config
# Move vendor into dist tarball directory
mv vendor "$DIST"

cd "$SOURCE_ROOT"/mod_repo
mkdir "$DIST"/mod_repo/.cargo
cargo vendor | sed 's/^directory = ".*"/directory = "vendor"/g' > $DIST/mod_repo/.cargo/config
# Move vendor into dist tarball directory
cp -r vendor "$DIST"
mv vendor "$DIST"/mod_repo/
