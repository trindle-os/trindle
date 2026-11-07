#!/bin/bash
# Check: apt-get -s dist-upgrade in a private apt root (no root, no chroot) with Trindle's shipped backports source and pin file; needs network.
# Usage: simulate-upgrade.sh [pref-file]   (default: the shipped system/apt/trindle-backports.pref)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
pref=${1:-$here/../../system/apt/trindle-backports.pref}
src=$here/../../system/apt/trindle-backports.sources
W=${SIM_DIR:-$(mktemp -d)}
mkdir -p "$W"/{etc/apt/sources.list.d,etc/apt/preferences.d,etc/apt/apt.conf.d,var/lib/apt/lists/partial,var/lib/dpkg,var/cache/apt/archives/partial}
cat > "$W/etc/apt/sources.list.d/debian.sources" <<S
Types: deb
URIs: http://deb.debian.org/debian
Suites: trixie trixie-updates
Components: main non-free-firmware
Signed-By: /usr/share/keyrings/debian-archive-keyring.pgp
S
: > "$W/var/lib/dpkg/status"
A=(-o Dir="$W" -o Dir::Etc="$W/etc/apt" -o Dir::State="$W/var/lib/apt" -o Dir::State::status="$W/var/lib/dpkg/status"
   -o Dir::Cache="$W/var/cache/apt" -o Dir::Log="$W/log" -o APT::Architecture=amd64 -o Debug::NoLocking=1
   -o Acquire::Languages=none -o APT::Get::List-Cleanup=0 -o Dir::Bin::dpkg=/bin/true)
# Phase 1: stand in for an installed Debian stable system (stable kernel and firmware metas).
apt-get "${A[@]}" -qq update
want="linux-image-amd64 linux-base firmware-linux firmware-linux-free firmware-linux-nonfree firmware-misc-nonfree firmware-amd-graphics"
apt-get "${A[@]}" -s install $want | sed -n 's/^Inst \([^ ]*\) (\([^ ]*\) .*/\1 \2/p' > "$W/inst.txt"
while read -r p v; do
  apt-cache "${A[@]}" show "$p=$v" | awk '/^$/{exit} {print}' | grep -v '^Description' | sed '1i Status: install ok installed'
  echo
done < "$W/inst.txt" > "$W/var/lib/dpkg/status"
echo "installed stand-in: $(wc -l < "$W/inst.txt") packages"
# Phase 2: add the shipped backports source and pin file, then simulate.
cp "$src" "$W/etc/apt/sources.list.d/trindle-backports.sources"
cp "$pref" "$W/etc/apt/preferences.d/trindle-backports.pref"
apt-get "${A[@]}" -qq update
out=$(apt-get "${A[@]}" -s dist-upgrade 2>&1) || true
echo "$out" | grep -E '^Inst (linux-|firmware-)' || true
kept=$(echo "$out" | sed -n '/have been kept back/,/^[0-9]* upgraded/p' | grep -E '^ ' | tr -s ' ' '\n' | grep -E '^(linux-base|firmware-linux|linux-image-)' || true)
if [ -n "$kept" ]; then echo "RESULT: kept back: $kept"; exit 1; fi
echo "RESULT: linux-base, firmware-linux* and the kernel are not kept back"
