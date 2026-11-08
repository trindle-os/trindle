#!/bin/sh
# Says "--locked" when cargo may be held to Cargo.lock. A checkout that patches
here=$(cd "$(dirname "$0")/.." && pwd)
if [ -f "$here/.cargo/config.toml" ] && grep -q '^\[patch' "$here/.cargo/config.toml"; then
    echo "tools/locked.sh: the toolkit is patched to a local folder, so this build is not held to Cargo.lock" >&2
else
    echo "--locked"
fi
