#!/bin/sh
# Everything the Debian package holds, written into a staging folder.
set -eu

dest=${TRINDLE_DESTDIR:?a staging folder is needed}
here=$(cd "$(dirname "$0")/.." && pwd)
program=${1:-$here/target/release/trindle}
prefix=/usr/lib/trindle

[ -x "$program" ] || { echo "system-files.sh: no program at $program" >&2; exit 1; }

install -d "$dest$prefix/bin" "$dest$prefix/libexec" "$dest$prefix/share/icons/apps"
install -m755 "$program" "$dest$prefix/bin/trindle"
unlock=$(dirname "$program")/trindle-unlock
if [ -x "$unlock" ]; then install -m755 "$unlock" "$dest$prefix/bin/trindle-unlock"; fi
compose=$(dirname "$program")/trindle-compose
if [ -x "$compose" ]; then
    install -m755 "$compose" "$dest$prefix/bin/trindle-compose"
    install -d "$dest$prefix/xwayland-later"
    ln -sfn ../bin/trindle-compose "$dest$prefix/xwayland-later/Xwayland"
fi
(cd "$here/share" && tar chf - --exclude=./grub --exclude=./iso --exclude=./fonts --exclude=./licences .) \
    | (cd "$dest$prefix/share" && tar xf -)
[ -s "$dest$prefix/share/style.css" ] && [ -d "$dest$prefix/share/themes" ] \
    || { echo "share/style.css or share/themes did not reach the package" >&2; exit 1; }
ln -sfn ../bin/trindle "$dest$prefix/libexec/trindle-admin"
ln -sfn ../bin/trindle "$dest$prefix/libexec/trindle-dns"
ln -sfn ../bin/trindle "$dest$prefix/libexec/trindle-thunder"
ln -sfn ../bin/trindle "$dest$prefix/libexec/trindle-controller"
ln -sfn ../bin/trindle "$dest$prefix/libexec/trindle-usb-retry"
chmod -R go-w "$dest$prefix"

install -d "$dest/usr/bin"
for name in $("$dest$prefix/bin/trindle" --names); do
    ln -sfn "$prefix/bin/trindle" "$dest/usr/bin/$name"
done

for f in "$here"/share/applications/*.desktop; do
    case $(basename "$f") in
        trindle-*) install -Dm644 "$f" -t "$dest/usr/share/applications/" ;;
        *) install -Dm644 "$f" -t "$dest/usr/share/trindle/applications/" ;;
    esac
done
install -Dm644 "$here/system/xdg/trindle-mimeapps.list" "$dest/usr/share/applications/trindle-mimeapps.list"
install -Dm644 "$here/share/icons/trindle.svg" "$dest/usr/share/icons/hicolor/scalable/apps/trindle.svg"
install -d "$dest/usr/share/fonts/truetype/trindle-sans"
install -m644 "$here"/share/fonts/trindle-sans/*.ttf "$here"/share/fonts/trindle-sans/*.txt \
    "$dest/usr/share/fonts/truetype/trindle-sans/"
install -Dm644 "$here/share/icons/tailscale.svg" "$dest/usr/share/icons/hicolor/scalable/apps/tailscale.svg"
for mark in claude-desktop chatgpt nodejs docker; do
    install -Dm644 "$here/share/icons/$mark.svg" "$dest/usr/share/icons/hicolor/scalable/apps/$mark.svg"
done
install -Dm644 "$here/share/icons/trindle-keep-awake-symbolic.svg" \
    "$dest/usr/share/icons/hicolor/scalable/status/trindle-keep-awake-symbolic.svg"
install -Dm644 "$here/share/icons/github-symbolic.svg" \
    "$dest/usr/share/icons/hicolor/scalable/status/github-symbolic.svg"

skel=$dest/etc/skel/.config
install -d "$skel/trindle"
for f in alacritty/alacritty.toml xdg-terminals.list gtk-4.0/settings.ini gtk-3.0/settings.ini trindle/favourites; do
    install -Dm644 "$here/config/$f" "$skel/$f"
done

for f in "$here"/share/themes/*.css; do
    if [ -f "$f" ]; then
        install -Dm644 "$f" -t "$dest/usr/share/trindle/themes/"
    fi
done
install -Dm644 "$here/config/alacritty/trindle.toml" "$dest/usr/share/trindle/alacritty/trindle.toml"
install -d "$dest/usr/share/trindle/desktop"
install -m644 "$here"/config/desktop/shortcuts.xml "$here"/config/desktop/environment "$dest/usr/share/trindle/desktop/"
install -Dm644 "$here/system/xdg-desktop-portal/trindle-portals.conf" \
    "$dest/usr/share/xdg-desktop-portal/trindle-portals.conf"
install -Dm644 "$here/system/xdg-desktop-portal/trindle.portal" \
    "$dest/usr/share/xdg-desktop-portal/portals/trindle.portal"
install -Dm644 "$here/system/xdg-desktop-portal/org.freedesktop.impl.portal.desktop.trindle.service" \
    "$dest/usr/share/dbus-1/services/org.freedesktop.impl.portal.desktop.trindle.service"
install -Dm644 "$here/system/xdg-desktop-portal/trindle-keyring.portal" \
    "$dest/usr/share/xdg-desktop-portal/portals/trindle-keyring.portal"
install -Dm644 "$here/system/pam-configs/trindle-keyring" "$dest/usr/share/pam-configs/trindle-keyring"

install -Dm644 "$here/system/greetd/greetd.toml" "$dest/etc/trindle/greetd.toml"
install -Dm644 "$here/system/systemd/greetd-trindle.conf" \
    "$dest/usr/lib/systemd/system/greetd.service.d/trindle.conf"
install -Dm644 "$here/system/tmpfiles/trindle-reader.conf" "$dest/usr/lib/tmpfiles.d/trindle-reader.conf"
install -Dm644 "$here/system/systemd/trindle-session.target" "$dest/usr/lib/systemd/user/trindle-session.target"
for unit in trindle-boxed-patch.service trindle-boxed-patch.timer; do
    install -Dm644 "$here/system/systemd/$unit" "$dest/usr/lib/systemd/user/$unit"
done
install -d "$dest/usr/lib/systemd/user/timers.target.wants"
ln -sfn ../trindle-boxed-patch.timer "$dest/usr/lib/systemd/user/timers.target.wants/trindle-boxed-patch.timer"
install -Dm644 "$here/system/wayland-sessions/trindle.desktop" "$dest/usr/share/wayland-sessions/trindle.desktop"
install -Dm644 "$here/system/polkit/org.trindle.admin.policy" "$dest/usr/share/polkit-1/actions/org.trindle.admin.policy"
install -Dm644 "$here/system/polkit/60-trindle.rules" "$dest/usr/share/polkit-1/rules.d/60-trindle.rules"
install -Dm644 "$here/system/pam.d/trindle-lock" "$dest/etc/pam.d/trindle-lock"

theme=$dest/boot/grub/themes/trindle
install -d "$theme"
install -m644 "$here"/share/grub/trindle/*.png "$here"/share/grub/trindle/theme.txt "$theme"/
if command -v grub-mkfont >/dev/null 2>&1; then
    font=$here/share/fonts/trindle-sans/TrindleSans-Variable.ttf
    bold=$(fc-scan --format '%{index} %{style[0]}\n' "$font" | awk '$2 == "Bold" { print $1; exit }')
    for spec in Regular:14 Regular:16 Regular:18 Bold:18 Bold:30; do
        style=${spec%:*}; size=${spec#*:}
        if [ "$style" = Bold ]; then index=${bold:-0}; name="Trindle Sans Bold"; else index=0; name="Trindle Sans"; fi
        grub-mkfont --index="$index" --name="$name" -s "$size" -o "$theme/trindle-sans-$style-$size.pf2" "$font" 2>/dev/null
    done
fi
install -Dm644 "$here/system/grub/trindle.cfg" "$dest/etc/default/grub.d/trindle.cfg"
install -d "$dest/etc/grub.d"
ln -sfn "$prefix/bin/trindle" "$dest/etc/grub.d/41_trindle_windows"
ln -sfn "$prefix/bin/trindle" "$dest/etc/grub.d/42_trindle_snapshots"
install -m755 "$here/system/grub/06_trindle_quiet" "$dest/etc/grub.d/06_trindle_quiet"

if [ -x "$dest$prefix/bin/trindle-unlock" ]; then
    install -Dm755 "$here/system/initramfs/trindle-unlock-hook" \
        "$dest/etc/initramfs-tools/hooks/trindle-unlock"
    install -Dm644 "$here/system/initramfs/trindle-small.conf" \
        "$dest/usr/share/trindle/initramfs-small.conf"
    install -Dm755 "$here/system/initramfs/trindle-spin" \
        "$dest/etc/initramfs-tools/scripts/init-top/trindle-spin"
    install -Dm755 "$here/system/initramfs/trindle-spin-panic" \
        "$dest/etc/initramfs-tools/scripts/panic/trindle-spin"
    install -Dm644 "$here/system/systemd/trindle-spin.service" \
        "$dest/usr/lib/systemd/system/trindle-spin.service"
    install -d "$dest/usr/lib/systemd/system/sysinit.target.wants"
    ln -sfn ../trindle-spin.service "$dest/usr/lib/systemd/system/sysinit.target.wants/trindle-spin.service"
    install -Dm644 "$here/system/systemd/trindle-spin-closing.service" \
        "$dest/usr/lib/systemd/system/trindle-spin-closing.service"
    for t in poweroff reboot halt; do
        install -d "$dest/usr/lib/systemd/system/$t.target.wants"
        ln -sfn ../trindle-spin-closing.service "$dest/usr/lib/systemd/system/$t.target.wants/trindle-spin-closing.service"
    done
    install -Dm644 "$here/system/systemd/getty-after-login-screen.conf" \
        "$dest/usr/lib/systemd/system/getty@tty1.service.d/trindle.conf"
    for shell in emergency rescue; do
        install -Dm644 "$here/system/systemd/trindle-spin-stop.conf" \
            "$dest/usr/lib/systemd/system/$shell.service.d/trindle-spin.conf"
    done
fi

install -Dm644 "$here/system/glib/90_trindle.gschema.override" \
    "$dest/usr/share/glib-2.0/schemas/90_trindle.gschema.override"

install -Dm755 "$here/system/systemd/trindle-touch-after-sleep" "$dest/usr/lib/systemd/system-sleep/trindle-touch"

install -Dm644 "$here/system/systemd/user-stop-quickly.conf" "$dest/usr/lib/systemd/user.conf.d/trindle.conf"

for unit in pipewire.socket pipewire.service pipewire-pulse.socket pipewire-pulse.service \
            wireplumber.service filter-chain.service; do
    install -Dm644 "$here/system/systemd/sound-not-for-login-screen.conf" \
        "$dest/usr/lib/systemd/user/$unit.d/trindle-not-login-screen.conf"
done

install -Dm755 "$here/system/initramfs/trindle-snapshot" \
    "$dest/etc/initramfs-tools/scripts/local-bottom/trindle-snapshot"
install -Dm755 "$here/system/initramfs/trindle-snapshot-hook" \
    "$dest/etc/initramfs-tools/hooks/trindle-snapshot"
install -Dm644 "$here/system/systemd/trindle-snapshots-grub.path" \
    "$dest/usr/lib/systemd/system/trindle-snapshots-grub.path"
install -Dm644 "$here/system/systemd/trindle-snapshots-grub.service" \
    "$dest/usr/lib/systemd/system/trindle-snapshots-grub.service"
install -Dm644 "$here/system/systemd/trindle-snapshots-grub-after-restore.service" \
    "$dest/usr/lib/systemd/system/trindle-snapshots-grub-after-restore.service"
install -d "$dest/usr/lib/systemd/system/multi-user.target.wants"
ln -sfn ../trindle-snapshots-grub-after-restore.service \
    "$dest/usr/lib/systemd/system/multi-user.target.wants/trindle-snapshots-grub-after-restore.service"
for unit in trindle-home-snapshots.service trindle-home-snapshots.timer; do
    install -Dm644 "$here/system/systemd/$unit" "$dest/usr/lib/systemd/system/$unit"
done
install -d "$dest/usr/lib/systemd/system/timers.target.wants"
ln -sfn ../trindle-home-snapshots.timer "$dest/usr/lib/systemd/system/timers.target.wants/trindle-home-snapshots.timer"
for unit in trindle-read-only.service trindle-read-only.timer; do
    install -Dm644 "$here/system/systemd/$unit" "$dest/usr/lib/systemd/system/$unit"
done
install -Dm644 "$here/system/dpkg/trindle-read-only" "$dest/etc/dpkg/dpkg.cfg.d/trindle-read-only"
install -Dm644 "$here/system/systemd/trindle-charge-limit.service" \
    "$dest/usr/lib/systemd/system/trindle-charge-limit.service"
install -Dm644 "$here/system/systemd/trindle-firewall.service" \
    "$dest/usr/lib/systemd/system/trindle-firewall.service"
ln -sfn ../trindle-firewall.service "$dest/usr/lib/systemd/system/multi-user.target.wants/trindle-firewall.service"
for unit in trindle-sharing.path trindle-sharing.service; do
    install -Dm644 "$here/system/systemd/$unit" "$dest/usr/lib/systemd/system/$unit"
    ln -sfn ../$unit "$dest/usr/lib/systemd/system/multi-user.target.wants/$unit"
done
install -Dm644 "$here/system/systemd/trindle-dns.service" \
    "$dest/usr/lib/systemd/system/trindle-dns.service"
install -Dm644 "$here/system/systemd/trindle-shares@.service" \
    "$dest/usr/lib/systemd/system/trindle-shares@.service"
install -Dm644 "$here/system/sysctl/90-trindle-games.conf" \
    "$dest/usr/lib/sysctl.d/90-trindle-games.conf"
install -Dm644 "$here/system/sysctl/90-trindle-hardening.conf" \
    "$dest/usr/lib/sysctl.d/90-trindle-hardening.conf"
install -Dm644 "$here/system/systemd/limits/90-trindle-games.conf" \
    "$dest/usr/lib/systemd/system.conf.d/90-trindle-games.conf"
install -Dm644 "$here/system/systemd/limits/90-trindle-games.conf" \
    "$dest/usr/lib/systemd/user.conf.d/90-trindle-games.conf"
install -Dm644 "$here/system/udev/70-trindle-game-controllers.rules" \
    "$dest/usr/lib/udev/rules.d/70-trindle-game-controllers.rules"
install -Dm644 "$here/system/systemd/trindle-thunder.service" \
    "$dest/usr/lib/systemd/system/trindle-thunder.service"
install -Dm644 "$here/system/udev/90-trindle-thunder.rules" \
    "$dest/usr/lib/udev/rules.d/90-trindle-thunder.rules"
for unit in trindle-controller.socket trindle-controller.service; do
    install -Dm644 "$here/system/systemd/$unit" "$dest/usr/lib/systemd/system/$unit"
done
install -d "$dest/usr/lib/systemd/system/sockets.target.wants"
ln -sfn ../trindle-controller.socket \
    "$dest/usr/lib/systemd/system/sockets.target.wants/trindle-controller.socket"
install -Dm644 "$here/system/systemd/trindle-usb-retry.service" \
    "$dest/usr/lib/systemd/system/trindle-usb-retry.service"
install -d "$dest/usr/lib/systemd/system/multi-user.target.wants"
ln -sfn ../trindle-usb-retry.service \
    "$dest/usr/lib/systemd/system/multi-user.target.wants/trindle-usb-retry.service"
install -Dm644 "$here/system/systemd/trindle-usb-quiet.service" \
    "$dest/usr/lib/systemd/system/trindle-usb-quiet.service"
for t in reboot kexec; do
    install -d "$dest/usr/lib/systemd/system/$t.target.wants"
    ln -sfn ../trindle-usb-quiet.service \
        "$dest/usr/lib/systemd/system/$t.target.wants/trindle-usb-quiet.service"
done
install -Dm644 "$here/system/sysusers/trindle-dns.conf" \
    "$dest/usr/lib/sysusers.d/trindle-dns.conf"
install -Dm644 "$here/system/modprobe/trindle-charge.conf" \
    "$dest/usr/lib/modprobe.d/trindle-charge.conf"

install -Dm644 /dev/stdin "$dest/usr/lib/systemd/journald.conf.d/60-trindle.conf" <<'LOG'
[Journal]
Storage=persistent
SystemMaxUse=200M
SystemKeepFree=1G
LOG

install -Dm644 /dev/stdin "$dest/usr/lib/NetworkManager/conf.d/60-trindle-wifi-address.conf" <<'NM'
[device]
wifi.scan-rand-mac-address=yes

[connection]
wifi.cloned-mac-address=stable

# The drivers Debian's no-mac-addr-change.conf names cannot change it.
[connection-trindle-real-address]
match-device=driver:eagle_sdio,driver:wl
wifi.cloned-mac-address=preserve
NM

install -Dm644 /dev/stdin "$dest/etc/apt/apt.conf.d/20auto-upgrades" <<'APT'
APT::Periodic::Update-Package-Lists "1";
APT::Periodic::Unattended-Upgrade "0";
APT
install -Dm644 /dev/stdin "$dest/etc/apt/apt.conf.d/52trindle-security-updates" <<'APT'
APT::Periodic::Unattended-Upgrade "1";
#clear Unattended-Upgrade::Origins-Pattern;
Unattended-Upgrade::Origins-Pattern {
	"origin=Debian,codename=${distro_codename},label=Debian-Security";
	"origin=Debian,codename=${distro_codename}-security,label=Debian-Security";
};
// A security fix never takes a program away, not even one it no longer
// needs: tidying is Trindle's own, and it keeps screen readers.
Unattended-Upgrade::Remove-Unused-Dependencies "false";
Unattended-Upgrade::Remove-New-Unused-Dependencies "false";
APT

install -Dm644 "$here/system/protected-packages.txt" "$dest/usr/share/trindle/protected-packages.txt"

install -Dm644 "$here/system/apt/trindle.sources" "$dest/etc/apt/sources.list.d/trindle.sources"
install -Dm644 "$here/system/apt/trindle-archive-keyring.gpg" \
    "$dest/usr/share/keyrings/trindle-archive-keyring.gpg"

install -Dm644 "$here/system/apt/trindle-backports.sources" "$dest/etc/apt/sources.list.d/trindle-backports.sources"
install -Dm644 "$here/system/apt/trindle-backports.pref" "$dest/etc/apt/preferences.d/trindle-backports.pref"
install -Dm755 "$here/system/kernel/dkms-hook" "$dest/usr/lib/trindle/dkms-hook"

version=$(sed -n 's/^version = "\(.*\)"/\1/p' "$here/Cargo.toml" | head -n 1)
[ -n "$version" ] || { echo "system-files.sh: no version in Cargo.toml" >&2; exit 1; }
install -d "$dest/usr/lib"
sed "s/@VERSION@/$version/g" "$here/system/identity/os-release" > "$dest/usr/lib/os-release"
chmod 644 "$dest/usr/lib/os-release"
install -Dm644 "$here/system/identity/issue" "$dest/etc/issue"
install -Dm644 "$here/system/identity/issue.net" "$dest/etc/issue.net"
