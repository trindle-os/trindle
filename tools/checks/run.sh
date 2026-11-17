#!/bin/sh
# Repository checks beyond cargo: dead docs/ names, help naming a label no screen has, and the English rules (Vale).
set -u
export LC_ALL=C
here=$(cd "$(dirname "$0")/../.." && pwd)
checks="$here/tools/checks"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
found=0

copies=$(sed -n 's/^ *("\([^"]*\)", "[^"]*"),$/:!\1/p' "$here/src/common/from_the_toolkit.rs")
[ -n "$copies" ] || { echo "docs: no toolkit copies read from src/common/from_the_toolkit.rs"; found=1; }
git -C "$here" grep -ohE "docs/[A-Za-z0-9_./-]+\.md" -- . $copies | sort -u > "$work/docs"
while read -r doc; do
    [ -f "$here/$doc" ] || { echo "docs: $doc is named but does not exist"; found=1; }
done < "$work/docs"

cat "$here"/share/lang/en/*.ftl | grep -E '^[a-z][a-zA-Z0-9_-]* =' | sed -E 's/^([a-zA-Z0-9_-]+) = ?/\1\t/' > "$work/all.txt"
grep -v -E '^[^	]*-keywords	' "$work/all.txt" > "$work/prose.txt"
cut -f2 "$work/prose.txt" > "$work/en.txt"

python3 -I "$checks/help_names.py" "$work/all.txt" > "$work/help" || { echo "help: help_names.py failed"; found=1; }
sed 's/^/help: /' "$work/help"
[ -s "$work/help" ] && found=1

vale=${VALE:-$(command -v vale || true)}
if [ -n "$vale" ] && [ -x "$vale" ]; then
    (cd "$checks/vale" && "$vale" --no-wrap --output=line "$work/en.txt") > "$work/vale"
    case $? in 0) ;; 1) found=1 ;; *) echo "vale: did not run"; found=1 ;; esac
    awk -F: 'NR==FNR{split($0,kv,"\t"); key[FNR]=kv[1]; next} {line=$2; sub(/^[^:]*:[^:]*:[^:]*:/, ""); print "vale: " key[line] ": " $0}' "$work/prose.txt" "$work/vale"
elif [ -n "${VALE:-}" ]; then
    echo "vale: $VALE is not a program"; found=1
else
    echo "vale: not installed, English rules skipped"
fi
exit "$found"
