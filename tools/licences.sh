#!/bin/sh
# Writes the third-party notices: every package in Cargo.lock with its
# declared licence and the licence and copyright files found in its source.
#
#   sh tools/licences.sh [outfile]
#
# Runs offline from the cargo registry already on this computer. A package
# whose source has no licence file gets the standard text of the licence it
# declares (tools/licence-texts), MIT where it offers MIT, under its authors'
# names from its Cargo.toml. Identical texts are printed once.
# share/licences/THIRD-PARTY.txt is made by this, never edited by hand; a test
# fails when a package in Cargo.lock is missing from it.
set -eu
here=$(cd "$(dirname "$0")/.." && pwd)
out=${1:-$here/share/licences/THIRD-PARTY.txt}
cd "$here"
meta=$(mktemp)
seen=$(mktemp)
rows=$(mktemp)
trap 'rm -f "$meta" "$seen" "$rows"' EXIT
cargo +1.85 metadata --format-version 1 --offline $("$here/tools/locked.sh") > "$meta"
jq -r '.packages | sort_by(.name, .version)[]
       | [.name, .version, (.license // "unknown"), (.source // "local"), (.manifest_path | rtrimstr("/Cargo.toml")),
          ((.authors | join(", ")) as $a | if $a == "" then "the " + .name + " authors" else $a end)]
       | @tsv' "$meta" > "$rows"

{
echo "Third-party notices for Trindle"
echo
echo "Trindle is free software under the GNU General Public License, version 3 or"
echo "any later version. It is built from the packages below, each under its own"
echo "licence. This file is made from Cargo.lock by tools/licences.sh."
echo
while IFS='	' read -r name version licence source dir authors; do
    echo "================================================================"
    echo "$name $version"
    case "$name" in trindle-common|trindle-toolkit|trindle-help) source=local ;; esac
    if [ "$source" = local ]; then
        if [ "$name" = smithay ]; then
            echo "Licence: MIT"
            echo "Smithay, kept in vendor/smithay with Trindle's changes (see vendor/smithay/TRINDLE.md)."
        else
            echo "Licence: GPL-3.0-or-later"
            echo "Part of Trindle itself."
            echo
            continue
        fi
    else
        echo "Licence: $licence"
    fi
    found=0
    for f in "$dir"/LICEN[CS]E* "$dir"/COPYING* "$dir"/NOTICE* "$dir"/UNLICENSE* "$dir"/licen[cs]e*; do
        [ -f "$f" ] || continue
        found=1
        sum=$(sha256sum "$f" | cut -d' ' -f1)
        echo
        echo "--- $(basename "$f")"
        first=$(grep "^$sum " "$seen" | head -1 | cut -d' ' -f2- || true)
        if [ -n "$first" ]; then
            echo "Same text as the one printed under: $first"
        else
            echo "$sum $name $version ($(basename "$f"))" >> "$seen"
            cat "$f"
            echo
        fi
    done
    if [ "$found" = 0 ]; then
        echo
        case "$licence" in
            *MIT*)
                echo "--- MIT (the package holds no licence file; the standard text, under its authors' names)"
                echo "Copyright (c) $authors"
                echo
                cat "$here/tools/licence-texts/MIT" ;;
            *Apache-2.0*)
                echo "--- Apache-2.0 (the package holds no licence file; the standard text)"
                cat "$here/tools/licence-texts/Apache-2.0" ;;
            *)
                echo "The licence text could not be found in this package's source. It is declared as: $licence." ;;
        esac
    fi
    echo
done < "$rows"
} > "$out"
echo "$out"
