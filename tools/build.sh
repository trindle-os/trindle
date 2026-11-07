#!/bin/sh
# Build Trindle with the folder names of the machine it was built on kept out of the finished program.
set -eu

here=$(cd "$(dirname "$0")/.." && pwd)
cargo_home=${CARGO_HOME:-$HOME/.cargo}
maps="--remap-path-prefix=$HOME=/src --remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$here=/trindle --remap-path-prefix=$here/target=/out --remap-path-prefix=$here/target/release/build=/generated"
RUSTFLAGS="$maps${RUSTFLAGS:+ $RUSTFLAGS}" cargo build --release $("$here/tools/locked.sh") "$@"
