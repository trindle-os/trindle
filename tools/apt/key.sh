#!/bin/sh
# The public half of Trindle's archive key, and its whole fingerprint.
set -eu

here=$(cd "$(dirname "$0")/../.." && pwd)
if [ -n "${TRINDLE_APT_KEYRING:-}" ] && [ "${TRINDLE_APT_TEST:-}" != 1 ]; then
    echo "TRINDLE_APT_KEYRING is set, which stands another key in for the one Trindle ships. It is for the test of these scripts alone: unset it." >&2
    exit 1
fi
keyring=${TRINDLE_APT_KEYRING:-$here/system/apt/trindle-archive-keyring.gpg}
[ -f "$keyring" ] || { echo "no public key at $keyring" >&2; exit 1; }

case ${1:-} in
file)
    echo "$keyring"
    ;;
fingerprint)
    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"' EXIT
    prints=$(gpg --homedir "$tmp" --batch --no-tty --show-keys --with-colons "$keyring" 2>/dev/null |
        awk -F: '$1 == "pub" { own = 1 } own && $1 == "fpr" { print $10; own = 0 }')
    [ "$(printf '%s\n' "$prints" | grep -c .)" -eq 1 ] || { echo "$keyring must hold exactly one key" >&2; exit 1; }
    [ "${#prints}" -eq 40 ] || { echo "$keyring gives no whole fingerprint" >&2; exit 1; }
    echo "$prints"
    ;;
expires)
    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"' EXIT
    gpg --homedir "$tmp" --batch --no-tty --show-keys --with-colons "$keyring" 2>/dev/null |
        awk -F: '$1 == "pub" { print $7 + 0; exit }'
    ;;
*)
    echo "usage: sh tools/apt/key.sh file|fingerprint|expires" >&2
    exit 1
    ;;
esac
