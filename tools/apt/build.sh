#!/bin/sh
# Build the apt repository that computers running Trindle update from: a plain
set -eu

repo=${1:?where to write the repository}
shift
empty=${TRINDLE_APT_EMPTY:-}
[ "$#" -gt 0 ] || [ -n "$empty" ] || { echo "no packages given" >&2; exit 1; }
[ "$#" -eq 0 ] || [ -z "$empty" ] || { echo "TRINDLE_APT_EMPTY is set, yet packages were given" >&2; exit 1; }

here=$(cd "$(dirname "$0")/../.." && pwd)
key=$(sh "$here/tools/apt/key.sh" fingerprint)
keyring=$(sh "$here/tools/apt/key.sh" file)
suite=stable
component=main

icons=${TRINDLE_ICON_DEBS:-$here/../trindle-icons/dist/apt/pool/main}
if [ -z "$empty" ]; then
    for name in trindle-circle-icon-theme; do
        deb=$(ls -1 "$icons/${name}_"*_all.deb 2>/dev/null | sort -V | tail -1)
        [ -n "$deb" ] || { echo "no $name package in $icons: build trindle-icons, or set TRINDLE_ICON_DEBS" >&2; exit 1; }
        set -- "$@" "$deb"
    done
fi

mkdir -p "$repo/pool/$component"
for deb in "$@"; do
    cp "$deb" "$repo/pool/$component/"
done

for arch in amd64 arm64; do
    dir="$repo/dists/$suite/$component/binary-$arch"
    mkdir -p "$dir"
    (cd "$repo" && apt-ftparchive --arch "$arch" packages "pool/$component") > "$dir/Packages"
    gzip -9fkn "$dir/Packages"
    {
        echo "Archive: $suite"
        echo "Component: $component"
        echo "Origin: Trindle"
        echo "Label: Trindle"
        echo "Architecture: $arch"
    } > "$dir/Release"
done

(cd "$repo" && apt-ftparchive \
    -o "APT::FTPArchive::Release::Origin=Trindle" \
    -o "APT::FTPArchive::Release::Label=Trindle" \
    -o "APT::FTPArchive::Release::Suite=$suite" \
    -o "APT::FTPArchive::Release::Codename=$suite" \
    -o "APT::FTPArchive::Release::Architectures=amd64 arm64" \
    -o "APT::FTPArchive::Release::Components=$component" \
    -o "APT::FTPArchive::Release::Description=Trindle" \
    release "dists/$suite") > "$repo/dists/$suite/Release"

rm -f "$repo/dists/$suite/Release.gpg" "$repo/dists/$suite/InRelease"
gpg --batch --yes --local-user "$key!" --armor --detach-sign \
    -o "$repo/dists/$suite/Release.gpg" "$repo/dists/$suite/Release"
gpg --batch --yes --local-user "$key!" --clearsign \
    -o "$repo/dists/$suite/InRelease" "$repo/dists/$suite/Release"

cp "$keyring" "$repo/trindle-archive-keyring.gpg"
cp "$here/system/apt/trindle.sources" "$repo/trindle.sources"

sh "$here/tools/apt/verify.sh" "$repo" || { echo "$repo is not signed as it should be. Do not publish it." >&2; exit 1; }

echo "Repository written to $repo"
