#!/bin/sh
# Check that a built repository is signed by Trindle's archive key, as a
set -eu

repo=${1:?which repository to check}
dist="$repo/dists/stable"
tools=$(cd "$(dirname "$0")" && pwd)
keyring=$(sh "$tools/key.sh" file)
want=$(sh "$tools/key.sh" fingerprint)

expires=$(sh "$tools/key.sh" expires)
if [ "$expires" -gt 0 ] && [ "$expires" -lt "$(( $(date +%s) + 5184000 ))" ]; then
    echo "the archive key expires within 60 days (or has): renew it first, see docs/apt-key-rotation.md" >&2
    exit 1
fi

for f in Release Release.gpg InRelease; do
    [ -f "$dist/$f" ] || { echo "$dist/$f is missing" >&2; exit 1; }
done

tmp=$(mktemp -d)
trap 'gpgconf --homedir "$tmp/keys" --kill all >/dev/null 2>&1 || true; rm -rf "$tmp"' EXIT
mkdir -m 700 "$tmp/keys"
only() {
    gpg --homedir "$tmp/keys" --batch --no-tty --quiet "$@"
}
only --import "$keyring" 2>/dev/null || { echo "the public key at $keyring could not be read" >&2; exit 1; }

good() {
    awk -v want="$want" '$1 == "[GNUPG:]" && $2 == "VALIDSIG" && $NF == want { ok = 1 } END { exit !ok }' "$1"
}

only --status-fd 3 --verify "$dist/Release.gpg" "$dist/Release" 3>"$tmp/detached" 2>/dev/null || true
good "$tmp/detached" || { echo "Release.gpg is not a good signature of Release by the archive key $want" >&2; exit 1; }

only --status-fd 3 --output "$tmp/signed" --decrypt "$dist/InRelease" 3>"$tmp/inline" 2>/dev/null || true
good "$tmp/inline" || { echo "InRelease is not signed by the archive key $want" >&2; exit 1; }
cmp -s "$tmp/signed" "$dist/Release" || { echo "InRelease does not sign the Release beside it" >&2; exit 1; }

echo "Signed by the archive key $want"
