#!/bin/bash
# Simulates apt-get dist-upgrade in a private apt root (no root, no chroot) with Trindle's shipped backports source and pin file; needs network.
set -euo pipefail
export LC_ALL=C
here=$(cd "$(dirname "$0")" && pwd)
pref=${1:-$here/../../system/apt/trindle-backports.pref}
src=$here/../../system/apt/trindle-backports.sources
root=${SIM_DIR:-$(mktemp -d)}
mkdir -p "$root"/{etc/apt/sources.list.d,etc/apt/preferences.d,etc/apt/apt.conf.d,var/lib/apt/lists/partial,var/lib/dpkg,var/cache/apt/archives/partial}
cat > "$root/etc/apt/sources.list.d/debian.sources" <<SOURCES
Types: deb
URIs: http://deb.debian.org/debian
Suites: trixie trixie-updates
Components: main non-free-firmware
Signed-By: /usr/share/keyrings/debian-archive-keyring.pgp
SOURCES
: > "$root/var/lib/dpkg/status"
apt_opts=(-o Dir="$root" -o Dir::Etc="$root/etc/apt" -o Dir::State="$root/var/lib/apt" -o Dir::State::status="$root/var/lib/dpkg/status"
   -o Dir::Cache="$root/var/cache/apt" -o Dir::Log="$root/log" -o APT::Architecture=amd64 -o Debug::NoLocking=1
   -o Acquire::Languages=none -o APT::Get::List-Cleanup=0 -o Dir::Bin::dpkg=/bin/true)

apt-get "${apt_opts[@]}" -qq update
want=(linux-image-amd64 linux-base firmware-linux firmware-linux-free firmware-linux-nonfree firmware-misc-nonfree firmware-amd-graphics)
apt-get "${apt_opts[@]}" -s install "${want[@]}" | sed -n 's/^Inst \([^ ]*\) (\([^ ]*\) .*/\1 \2/p' > "$root/inst.txt"
while read -r package version; do
  apt-cache "${apt_opts[@]}" show "$package=$version" | awk '/^$/{exit} {print}' | grep -v '^Description' | sed '1i Status: install ok installed'
  echo
done < "$root/inst.txt" > "$root/var/lib/dpkg/status"
echo "installed stand-in: $(wc -l < "$root/inst.txt") packages"

cp "$src" "$root/etc/apt/sources.list.d/trindle-backports.sources"
cp "$pref" "$root/etc/apt/preferences.d/trindle-backports.pref"
apt-get "${apt_opts[@]}" -qq update
out=$(apt-get "${apt_opts[@]}" -s dist-upgrade 2>&1) || true
echo "$out" | grep -E '^Inst (linux-|firmware-)' || true
kept=$(echo "$out" | sed -n '/have been kept back/,/^[0-9]* upgraded/p' | grep -E '^ ' | tr -s ' ' '\n' | grep -E '^(linux-base|firmware-linux|linux-image-)' || true)
if [ -n "$kept" ]; then
  echo "kept back: $kept"
  exit 1
fi
echo "linux-base, firmware-linux* and the kernel are not kept back"
