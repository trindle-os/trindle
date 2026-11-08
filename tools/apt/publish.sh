#!/bin/sh
# Put the apt repository up, where computers running Trindle look for updates.
set -eu

repo=${1:?which folder to publish}
key=${TRINDLE_APT_KEY_FILE:?the upload key file, as TRINDLE_APT_KEY_FILE}
rclone=${RCLONE:-$(command -v rclone || echo rclone)}

if [ -n "${TRINDLE_APT_KEYRING:-}" ] && [ "${TRINDLE_APT_TEST:-}" != 1 ]; then
    echo "TRINDLE_APT_KEYRING is set, which stands another key in for the one Trindle ships. It is for the test of these scripts alone: unset it. Nothing was put up." >&2
    exit 1
fi
[ -f "$repo/dists/stable/InRelease" ] || { echo "$repo is not a built repository" >&2; exit 1; }
sh "$(dirname "$0")/verify.sh" "$repo" || { echo "$repo is not signed by Trindle's archive key. Nothing was put up." >&2; exit 1; }
if [ -n "${TRINDLE_APT_KEYRING:-}" ]; then
    echo "$repo is signed by the stand-in key it was checked against, for the test of these scripts. Nothing was put up." >&2
    exit 1
fi
[ -f "$key" ] || { echo "no key at $key" >&2; exit 1; }
[ -x "$rclone" ] || { echo "no rclone at $rclone" >&2; exit 1; }

value() {
    sed -n "s/^$1=//p" "$key" | tr -d "\"' \r"
}
id=$(value access_key_id)
secret=$(value secret_access_key)
endpoint=$(value endpoint)
bucket=$(value bucket)
bucket=${bucket:-trindle-apt}
[ -n "$id" ] && [ -n "$secret" ] && [ -n "$endpoint" ] || { echo "key file is missing a value" >&2; exit 1; }

host=${APT_HOST:-apt.trindle.org}

conf=$(mktemp)
trap 'rm -f "$conf"' EXIT
chmod 600 "$conf"
cat > "$conf" <<EOF
[r2]
type = s3
provider = Cloudflare
access_key_id = $id
secret_access_key = $secret
endpoint = $endpoint
acl = private
no_check_bucket = true
EOF

put() {
    echo "  $1"
    "$rclone" --config "$conf" copyto "$repo/$1" "r2:$bucket/$1"
}

echo "packages"
found=0
for deb in "$repo"/pool/main/*.deb; do
    [ -f "$deb" ] || continue
    put "pool/main/$(basename "$deb")"
    found=$((found + 1))
done
[ "$found" -gt 0 ] || [ -n "${TRINDLE_APT_EMPTY:-}" ] || { echo "no packages in $repo/pool/main" >&2; exit 1; }

echo "the two files a new computer needs first"
put trindle-archive-keyring.gpg
put trindle.sources

echo "indexes"
for arch in amd64 arm64; do
    for f in Packages Packages.gz Release; do
        put "dists/stable/main/binary-$arch/$f"
    done
done

echo "signatures, last"
put dists/stable/Release
put dists/stable/Release.gpg
put dists/stable/InRelease

echo
echo "Published. Checking what a computer would fetch from $host:"
wrong=0
tmp=$(mktemp)
for p in \
    dists/stable/InRelease \
    dists/stable/Release \
    dists/stable/Release.gpg \
    dists/stable/main/binary-amd64/Packages \
    dists/stable/main/binary-amd64/Packages.gz \
    dists/stable/main/binary-arm64/Packages \
    dists/stable/main/binary-arm64/Packages.gz \
    trindle-archive-keyring.gpg \
    trindle.sources
do
    code=$(curl -s -o "$tmp" -w '%{http_code}' -m 60 "https://$host/$p" || echo 000)
    if [ "$code" = 200 ] && cmp -s "$tmp" "$repo/$p"; then
        printf '  %-46s %s  same\n' "$p" "$code"
    else
        printf '  %-46s %s  DOES NOT MATCH\n' "$p" "$code"
        wrong=$((wrong + 1))
    fi
done
rm -f "$tmp"
if [ "$wrong" -gt 0 ]; then
    echo
    echo "$wrong of them are not what is in $repo. Do not announce this release." >&2
    exit 1
fi
