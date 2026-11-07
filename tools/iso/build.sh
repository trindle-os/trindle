#!/bin/sh
# Build the Trindle installation image.
set -eu

here=$(cd "$(dirname "$0")/../.." && pwd)
work=${1:-/var/tmp/trindle-iso}
prog="$here/target/release/trindle"
icons=${TRINDLE_ICON_DEBS:-$here/../trindle-icons/dist/apt/pool/main}
own_debs="$here/../trindle-folders/target/deb/trindle-folders_*.deb $here/../trindle-help/target/deb/trindle-help_*.deb $here/../trindle-toolkit/target/deb/trindle-toolkit-lang_*.deb $here/../trindle-crash-reports/packaging/out/trindle-crash-reports_*_amd64.deb $here/../trindle-voice_*_amd64.deb"

[ "$(id -u)" -eq 0 ] || { echo "run this as root" >&2; exit 1; }
[ -x "$prog" ] || { echo "build the program first: cargo build --release" >&2; exit 1; }
[ -d "$icons" ] || { echo "the Trindle icon packages are needed: build the trindle-icons project beside this folder (../trindle-icons/dist/apt/pool/main), or set TRINDLE_ICON_DEBS to where they are" >&2; exit 1; }
command -v lb >/dev/null || { echo "install live-build first" >&2; exit 1; }

icon_sets="trindle-circle-icon-theme"
newest() { ls -1 "$icons/${1}_"*_all.deb 2>/dev/null | sort -V | tail -1; }
for name in $icon_sets; do
    [ -n "$(newest "$name")" ] || { echo "no $name package in $icons: build trindle-icons, or set TRINDLE_ICON_DEBS" >&2; exit 1; }
done

mkdir -p "$work"
cd "$work"
lb clean --purge >/dev/null 2>&1 || true
rm -rf config

lb config \
    --distribution trixie \
    --archive-areas "main non-free non-free-firmware" \
    --apt-source-archives false \
    --binary-images iso-hybrid \
    --debian-installer none \
    --apt-recommends false \
    --apt-options "--yes -o Dpkg::Options::=--force-confold -o Dpkg::Options::=--force-confdef" \
    --apt-indices false \
    --memtest none \
    --firmware-chroot false \
    --updates true \
    --security true \
    --iso-application "Trindle" \
    --iso-publisher "Trindle; https://trindle.org" \
    --iso-volume "Trindle" \
    --bootappend-live "boot=live components quiet splash vt.global_cursor_default=0 username=trindle user-fullname=Trindle hostname=trindle"

cp -r "$here/tools/iso/config/." config/

mkdir -p config/archives
src="$here/system/apt/trindle-backports.sources"
printf 'deb %s %s %s\n' \
    "$(sed -n 's/^URIs: //p' "$src")" "$(sed -n 's/^Suites: //p' "$src")" "$(sed -n 's/^Components: //p' "$src")" \
    > config/archives/trindle-backports.list.chroot
cp "$here/system/apt/trindle-backports.pref" config/archives/trindle-backports.pref.chroot
sed -e 's/#.*//' "$here/system/apt/hardware-packages.txt" | tr -s ' \t' '\n\n' | sed '/^$/d' \
    > config/package-lists/hardware.list.chroot

mkdir -p config/includes.chroot/opt/trindle/target/release
cp -r "$here/share" "$here/config" "$here/system" "$here/tools" "$here/packages.txt" "$here/Cargo.toml" \
    config/includes.chroot/opt/trindle/
cp "$prog" config/includes.chroot/opt/trindle/target/release/trindle
suffix=${TRINDLE_VERSION_SUFFIX:-}
if [ -z "$suffix" ] && [ "${TRINDLE_ISO_TEST:-}" = yes ]; then
    suffix="+test$(date +%Y%m%d%H%M)"
fi
printf '%s' "$suffix" > config/includes.chroot/opt/trindle/version-suffix
for extra in trindle-unlock trindle-compose; do
    if [ -x "$here/target/release/$extra" ]; then
        cp "$here/target/release/$extra" config/includes.chroot/opt/trindle/target/release/
    fi
done
mkdir -p config/includes.chroot/opt/trindle/debs
for name in $icon_sets; do
    cp "$(newest "$name")" config/includes.chroot/opt/trindle/debs/
done
for pattern in $own_debs; do
    deb=$(ls -1 $pattern 2>/dev/null | sort -V | tail -1)
    [ -n "$deb" ] || { echo "no package matching $pattern: build it in its own repository" >&2; exit 1; }
    cp "$deb" config/includes.chroot/opt/trindle/debs/
done

main_deb=${TRINDLE_DEB:-$(ls -1 "$here"/target/deb/trindle_*_amd64.deb 2>/dev/null | sort -V | tail -1)}
[ -f "$main_deb" ] || { echo "build the Trindle package first: sh target/pkg.sh <number> (or set TRINDLE_DEB)" >&2; exit 1; }
cp "$main_deb" config/includes.chroot/opt/trindle/debs/

voice_files=${TRINDLE_VOICE_FILES:-/var/tmp/trindle-voice-files}
voice_sums=${TRINDLE_VOICE_SUMS:-$here/../trindle-voice/SHA256SUMS}
[ -f "$voice_sums" ] || { echo "no checksums for the voice files at $voice_sums" >&2; exit 1; }
[ -f "$here/tools/iso/voice-files.txt" ] || { echo "no tools/iso/voice-files.txt: the addresses of the voice files" >&2; exit 1; }
mkdir -p "$voice_files"
while IFS="$(printf '\t')" read -r file address; do
    [ -n "$file" ] || continue
    want=$(awk -v f="$file" '$2 == f { print $1 }' "$voice_sums")
    [ -n "$want" ] || { echo "$file is not in $voice_sums" >&2; exit 1; }
    have=$(sha256sum "$voice_files/$file" 2>/dev/null | cut -d' ' -f1 || true)
    if [ "$have" != "$want" ]; then
        curl -fL --retry 3 -o "$voice_files/$file.part" "$address"
        mv "$voice_files/$file.part" "$voice_files/$file"
    fi
done < "$here/tools/iso/voice-files.txt"
(cd "$voice_files" && sha256sum -c "$voice_sums" >/dev/null) || { echo "the voice files do not match their checksums" >&2; exit 1; }
mkdir -p config/includes.chroot/usr/share/trindle-voice
cp "$voice_files"/supertonic-* config/includes.chroot/usr/share/trindle-voice/

if [ "${TRINDLE_ISO_TEST:-}" = yes ] && [ -f "${TRINDLE_TEST_KEY:-}" ]; then
    echo "openssh-server" > config/package-lists/test.list.chroot
    mkdir -p config/includes.chroot/etc/skel/.ssh
    cp "$TRINDLE_TEST_KEY" config/includes.chroot/etc/skel/.ssh/authorized_keys
    chmod 600 config/includes.chroot/etc/skel/.ssh/authorized_keys
    printf '#!/bin/sh\nset -e\nmkdir -p /home/trindle/.ssh\ncp /etc/skel/.ssh/authorized_keys /home/trindle/.ssh/\nchown -R trindle:trindle /home/trindle/.ssh\nchmod 700 /home/trindle/.ssh\nsystemctl enable ssh\n' \
        > config/hooks/normal/0400-test-ssh.hook.chroot
    chmod +x config/hooks/normal/0400-test-ssh.hook.chroot
    echo "NOTE: this image has a test way in over the network. Do not publish it."
fi

if [ "${TRINDLE_ISO_TEST:-}" != yes ]; then
    if [ -e config/hooks/normal/0400-test-ssh.hook.chroot ] \
        || [ -e config/includes.chroot/etc/skel/.ssh ] \
        || [ -e config/package-lists/test.list.chroot ]; then
        echo "testing parts are still in $work/config; not building" >&2
        exit 1
    fi
fi

lb build

iso=$(ls -1 live-image-amd64.hybrid.iso 2>/dev/null || true)
[ -n "$iso" ] || { echo "no image was made" >&2; exit 1; }
name="trindle-$(date +%Y%m%d)-amd64.iso"
mv "$iso" "$name"
sha256sum "$name" > "$name.sha256"
echo "Made $work/$name"
