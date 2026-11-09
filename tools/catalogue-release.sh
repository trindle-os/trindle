#!/bin/bash
# Makes, checks and signs Trindle's plugin catalogue for a release.
set -eu
umask 077

here=$(cd "$(dirname "$0")" && pwd)
sums_file=$here/catalogue-release.sums
tmp=$(mktemp -d "${TMPDIR:-/tmp}/trindle-catalogue.XXXXXX")
trap 'rm -rf "$tmp"' EXIT

say() { printf '%s\n' "$*"; }
stop() { say "catalogue-release: $*" >&2; exit 1; }

publisher() {
    [ -n "${TRINDLE_PUBLISHER:-}" ] || stop "TRINDLE_PUBLISHER is not set (a trindle program built with --features publisher)."
    [ -x "$TRINDLE_PUBLISHER" ] || stop "TRINDLE_PUBLISHER is not an executable file."
    ln -sf "$(readlink -f "$TRINDLE_PUBLISHER")" "$tmp/trindle-plugin"
    "$tmp/trindle-plugin" "$@"
}

in_repository() {
    local d
    d=$(readlink -f "$(dirname "$1")")
    while [ "$d" != / ]; do
        [ -e "$d/.git" ] && return 0
        d=$(dirname "$d")
    done
    return 1
}

check_key() {
    local k=$1
    [ -f "$k" ] && [ ! -L "$k" ] || stop "the key is not a plain file."
    [ "$(stat -c %u "$k")" = "$(id -u)" ] || stop "the key file is not owned by this account."
    local mode
    mode=$(stat -c %a "$k")
    case $mode in
        400|600) ;;
        *) stop "the key file can be read by others (mode $mode). Make it 600 first." ;;
    esac
    ! in_repository "$k" || stop "the key file is inside a repository. A private key is never kept in one."
}

check_sums() {
    local dir=$1 f
    [ -f "$dir/SHA256SUMS" ] || stop "$dir has no SHA256SUMS."
    for f in "$dir"/*.tar.zst "$dir"/*.tar.gz "$dir"/*.tar.xz "$dir"/*.tar; do
        [ -e "$f" ] || continue
        grep -q "^[0-9a-f]\{64\}  $(basename "$f")\$" "$dir/SHA256SUMS" || stop "$(basename "$f") is not listed in SHA256SUMS, so it is not published."
    done
    (cd "$dir" && sha256sum --strict -c SHA256SUMS >/dev/null 2>&1) || stop "an archive does not match its SHA-256 in $dir/SHA256SUMS."
}

gather() {
    local to=$1 sum file from
    [ ! -e "$to" ] || [ -d "$to" ] || stop "$to is not a folder."
    mkdir -p "$to"
    : > "$tmp/gathered"
    while read -r sum file from; do
        case $sum in ''|'#'*) continue ;; esac
        if [ ! -f "$from" ]; then
            say "skipped (not built): $file"
            continue
        fi
        [ "$(sha256sum "$from" | cut -d' ' -f1)" = "$sum" ] || stop "$from is not the archive pinned in catalogue-release.sums (SHA-256 differs). Check what was rebuilt, then change the pinned line."
        cp -- "$from" "$to/$file"
        local base=${from%.tar.*} id=${file%.tar.*} ext
        for ext in .svg .png .description.txt .txt .md; do
            [ -f "$base$ext" ] && cp -- "$base$ext" "$to/$id$ext"
        done
        printf '%s  %s\n' "$sum" "$file" >> "$tmp/gathered"
        say "gathered: $file"
    done < "$sums_file"
    [ -s "$tmp/gathered" ] || stop "no plugin archive was found."
    cp "$tmp/gathered" "$to/SHA256SUMS"
}

make() {
    local in=$1 out=$2 key=$3 public=${4:-} f said
    [ -d "$in" ] || stop "$in is not a folder."
    [ ! -e "$out" ] || stop "$out is there already. Give a new folder, so nothing stale is published."
    check_sums "$in"
    mkdir -p "$out"
    for f in "$in"/*; do
        case $(basename "$f") in
            SHA256SUMS) ;;
            *.tar.zst|*.tar.gz|*.tar.xz|*.tar|*.svg|*.png|*.txt|*.md) cp -- "$f" "$out/" ;;
        esac
    done
    cp "$in/SHA256SUMS" "$out/SHA256SUMS"
    check_sums "$out"
    publisher catalogue "$out" || stop "the catalogue could not be made; see above."
    rm -f "$out/SHA256SUMS"
    said=$(publisher sign "$out/catalogue.ini" "$key") || stop "signing failed: $said"
    say "$said"
    if [ -z "$public" ]; then
        case $said in *"This Trindle accepts that key."*) ;; *) stop "the key is not one built into this Trindle (KEYS in src/plugins/catalogue.rs). Nothing is to be published." ;; esac
        publisher verify "$out" || stop "the finished folder does not check against the built-in keys."
    else
        publisher verify "$out" "$public" || stop "the finished folder does not check."
    fi
    say "Upload these files together to the release, catalogue.ini and catalogue.ini.sig last:"
    (cd "$out" && ls -1 | grep -v '^catalogue.ini' ; ls -1 catalogue.ini catalogue.ini.sig)
}

test_mode() {
    local t=$tmp/t made public
    mkdir -p "$t/work/org.example.Fake/bin" "$t/in"
    printf '[Plugin]\nInterface=1\nId=org.example.Fake\nName=Fake\nVersion=1.0\nSummary=A made-up plugin.\nPermissions=notifications\n\n[Ring]\nIcon=icon.svg\nLabel=Fake\nRun=bin/go\n' > "$t/work/org.example.Fake/plugin.ini"
    printf "<svg xmlns='http://www.w3.org/2000/svg'/>" > "$t/work/org.example.Fake/icon.svg"
    printf '#!/bin/sh\n' > "$t/work/org.example.Fake/bin/go"
    chmod 755 "$t/work/org.example.Fake/bin/go"
    tar --zstd -cf "$t/in/org.example.Fake.tar.zst" -C "$t/work" org.example.Fake
    (cd "$t/in" && sha256sum org.example.Fake.tar.zst > SHA256SUMS)
    made=$(publisher new-key "$t/throwaway.key") || stop "no throwaway key could be made."
    public=$(printf '%s\n' "$made" | tail -n 1)
    check_key "$t/throwaway.key"
    say "-- a good release"
    make "$t/in" "$t/out" "$t/throwaway.key" "$public"
    say "-- a key anyone can read is refused"
    cp "$t/throwaway.key" "$t/open.key" && chmod 644 "$t/open.key"
    ( check_key "$t/open.key" ) 2>/dev/null && stop "TEST FAILED: an open key was accepted."
    say "refused, as it should be"
    say "-- an archive changed after its checksum was written is refused"
    cp -r "$t/in" "$t/in2" && printf 'x' >> "$t/in2/org.example.Fake.tar.zst"
    ( make "$t/in2" "$t/out2" "$t/throwaway.key" "$public" ) 2>/dev/null >/dev/null && stop "TEST FAILED: a changed archive was published."
    [ ! -e "$t/out2/catalogue.ini.sig" ] || stop "TEST FAILED: a changed archive was signed."
    say "refused, as it should be"
    say "-- an archive not listed is refused"
    cp -r "$t/in" "$t/in3" && cp "$t/in/org.example.Fake.tar.zst" "$t/in3/org.example.Other.tar.zst"
    ( make "$t/in3" "$t/out3" "$t/throwaway.key" "$public" ) 2>/dev/null >/dev/null && stop "TEST FAILED: an unlisted archive was published."
    say "refused, as it should be"
    say "-- a published archive or catalogue changed afterwards is refused"
    printf 'x' >> "$t/out/org.example.Fake.tar.zst"
    publisher verify "$t/out" "$public" >/dev/null && stop "TEST FAILED: a changed published archive was accepted."
    sed -i 's/^Name=Fake/Name=Evil/' "$t/out/catalogue.ini"
    publisher verify "$t/out" "$public" >/dev/null && stop "TEST FAILED: a changed catalogue was accepted."
    say "refused, as it should be"
    say "TEST PASSED"
}

case ${1:-} in
    gather)
        [ $# = 2 ] || stop "usage: catalogue-release.sh gather FOLDER"
        gather "$2" ;;
    make)
        [ $# = 3 ] || stop "usage: TRINDLE_PUBLISHER=... TRINDLE_CATALOGUE_KEY=... catalogue-release.sh make FOLDER OUT"
        [ -n "${TRINDLE_CATALOGUE_KEY:-}" ] || stop "TRINDLE_CATALOGUE_KEY is not set (the path of the private key file)."
        check_key "$TRINDLE_CATALOGUE_KEY"
        make "$2" "$3" "$TRINDLE_CATALOGUE_KEY" ;;
    test)
        test_mode ;;
    *)
        stop "usage: catalogue-release.sh gather FOLDER | make FOLDER OUT | test" ;;
esac
