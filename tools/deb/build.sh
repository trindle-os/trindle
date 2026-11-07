#!/bin/sh
# Build the trindle Debian package from this folder.
set -eu

here=$(cd "$(dirname "$0")/../.." && pwd)
out=${1:-$here/target/deb}
version=$(sed -n 's/^version = "\(.*\)"/\1/p' "$here/Cargo.toml" | head -1)
version=$(echo "$version" | sed 's/-\(rc\|beta\|alpha\)\.\?/~\1/')
version="$version${TRINDLE_VERSION_SUFFIX:-}"
epoch=1
arch=$(dpkg --print-architecture)
[ -n "$version" ] || { echo "no version in Cargo.toml" >&2; exit 1; }
[ -x "$here/target/release/trindle" ] || { echo "build the program first: sh tools/build.sh" >&2; exit 1; }

stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT

TRINDLE_DESTDIR="$stage" sh "$here/tools/system-files.sh" "$here/target/release/trindle"

doc="$stage/usr/share/doc/trindle"
mkdir -p "$doc"
sh "$here/tools/licences.sh" "$doc/THIRD-PARTY.txt" >/dev/null
install -m644 "$here/tools/deb/copyright" "$doc/copyright"
install -m644 "$here/share/licences/sounds-and-wallpapers.txt" "$doc/sounds-and-wallpapers.txt"
chmod 644 "$doc/THIRD-PARTY.txt"

first_apps="alacritty"
"$here/target/release/trindle" boxed --gather "$stage/usr/share/trindle/first-apps" $first_apps

rm -rf "$stage/usr/bin"

size=$(du -sk "$stage" | cut -f1)
depends=$(sed -e 's/#.*//' "$here/packages.txt" | tr -s ' \n' ' ' | sed 's/^ //; s/ $//; s/ /, /g')
recommends=$(sed -e 's/#.*//' "$here/system/apt/hardware-packages.txt" | tr -s ' \n' ' ' | sed 's/^ //; s/ $//; s/ /, /g')
mkdir -p "$stage/DEBIAN"
sed -e "s/@VERSION@/$epoch:$version/" -e "s/@ARCH@/$arch/" -e "s/@SIZE@/$size/" -e "s/@DEPENDS@/$depends/" \
    -e "s/@RECOMMENDS@/$recommends/" \
    "$here/tools/deb/control" > "$stage/DEBIAN/control"
for f in preinst postinst prerm postrm; do
    install -m755 "$here/tools/deb/$f" "$stage/DEBIAN/$f"
done
(cd "$stage" && find etc -type f 2>/dev/null | sed 's|^|/|') > "$stage/DEBIAN/conffiles"

mkdir -p "$out"
deb="$out/trindle_${version}_${arch}.deb"
dpkg-deb --root-owner-group --build "$stage" "$deb" >/dev/null
echo "$deb"
