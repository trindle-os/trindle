#!/bin/sh
# Puts the git sources of the toolkit and Help back into Cargo.lock.
set -eu

here=$(cd "$(dirname "$0")/.." && pwd)
rev_of() {
    rev=$(sed -n "s/^$1 = .*rev = \"\([0-9a-f]\{40\}\)\".*/\1/p" "$here/Cargo.toml" | head -1)
    [ -n "$rev" ] || { echo "Cargo.toml names no full commit for $1" >&2; exit 1; }
    echo "$rev"
}
toolkit=$(rev_of trindle-toolkit)
help=$(rev_of trindle-help)
awk -v tk="git+https://github.com/trindle-os/trindle-toolkit?rev=$toolkit#$toolkit" -v hp="git+https://github.com/trindle-os/trindle-help?rev=$help#$help" '
    { line[NR] = $0 }
    END {
        for (i = 1; i <= NR; i++) {
            print line[i]
            if (line[i] ~ /^name = "trindle-(toolkit|common|help)"$/) {
                src = line[i] ~ /help/ ? hp : tk
                print line[++i]
                if (line[i + 1] !~ /^source = /) print "source = \"" src "\""
            }
        }
    }' "$here/Cargo.lock" > "$here/Cargo.lock.new"
awk -v RS= -v ORS='\n\n' '!seen[$0]++ { gsub(/trindle-common 0\.9\.0-beta\.1( \(git[^)]*\))?"/, "trindle-common\""); print }' "$here/Cargo.lock.new" | sed '$d' > "$here/Cargo.lock"
rm "$here/Cargo.lock.new"
