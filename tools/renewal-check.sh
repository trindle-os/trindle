#!/bin/bash
# renewal-check.sh: shows every long-running part become the new program
set -u
export LC_ALL=C
here=$(cd "$(dirname "$0")/.." && pwd)
if [ "${1:-}" != --inside ]; then
  unset DISPLAY WAYLAND_DISPLAY DBUS_SESSION_BUS_ADDRESS
  exec unshare -Urm bash "$0" --inside
fi
scratch=$(mktemp -d "${TMPDIR:-/tmp}/trindle-renewal.XXXXXX")
run=$(mktemp -d /tmp/trr.XXXXXX)
LABWC=${LABWC:-$(command -v labwc)}
GRIM=${GRIM:-$(command -v grim)}
labwc_root=$(dirname "$(dirname "$(dirname "$LABWC")")")
unset DISPLAY WAYLAND_DISPLAY DBUS_SESSION_BUS_ADDRESS GDK_BACKEND TRINDLE_COMPOSE_PID XDG_CONFIG_DIRS XDG_DATA_DIRS XDG_SESSION_ID XDG_VTNR XDG_SEAT SYSTEMD_EXEC_PID
export XDG_RUNTIME_DIR=$run
WAIT=${WAIT:-75}

ours() { cat /proc/"$1"/environ 2>/dev/null | tr "\\0" "\\n" | grep -qx "XDG_RUNTIME_DIR=$run"; }
stop_all() {
  for p in /proc/[0-9]*; do
    pid=${p#/proc/}
    [ "$pid" = $$ ] && continue
    ours "$pid" && kill "$pid" 2>/dev/null
  done
}
parts() {
  for p in /proc/[0-9]*; do
    pid=${p#/proc/}
    ours "$pid" || continue
    a0=$(tr '\0' ' ' < /proc/"$pid"/cmdline | cut -d' ' -f1-2)
    case "$a0" in *trindle*) ;; *) continue ;; esac
    printf '%s\t%s\t%s\n' "$pid" "$a0" "$(readlink /proc/"$pid"/exe 2>/dev/null || echo unreadable)"
  done | sort -k2
}

stop_all
mkdir -p "$scratch"/bin "$scratch"/home "$scratch"/shots && chmod 700 "$run" || { echo "could not make the scratch folders"; exit 1; }
for f in trindle trindle-compose trindle-unlock; do
  cp "$here"/target/release/$f "$scratch"/bin/$f && chmod 755 "$scratch"/bin/$f || { echo "could not copy $f from the build: build first"; exit 1; }
done
mount --bind "$scratch"/bin /usr/lib/trindle/bin || { echo "could not mount the private programs"; exit 1; }
mount -t tmpfs tmpfs /tmp/.X11-unix && chmod 1777 /tmp/.X11-unix
[ "$(stat -c %u /usr/lib/trindle/bin/trindle)" = 0 ] || { echo "the private program does not count as root's in here, so no part would renew"; exit 1; }
cmp -s /usr/lib/trindle/bin/trindle "$here"/target/release/trindle || { echo "the installed path is not the build"; exit 1; }
mkdir -p "$scratch"/home/.config/trindle
picture=$(ls "$here"/share/wallpapers/*.jpg "$here"/share/wallpapers/*.png "$here"/share/wallpapers/*.webp 2>/dev/null | head -1)
echo "picture: $picture"

base_path=/usr/local/bin:/usr/bin:/bin
env -i PATH=$base_path XDG_RUNTIME_DIR="$run" HOME="$scratch"/home WLR_BACKENDS=headless WLR_RENDERER=pixman WLR_HEADLESS_OUTPUTS=1 LD_LIBRARY_PATH="$labwc_root"/usr/lib/x86_64-linux-gnu \
  setsid "$LABWC" -s "sh -c 'echo \$WAYLAND_DISPLAY > $scratch/labwc.sock; sleep 100000'" > "$scratch"/labwc.log 2>&1 < /dev/null &
i=0; until [ -s "$scratch"/labwc.sock ] || [ $i -gt 50 ]; do sleep 0.2; i=$((i+1)); done
[ -s "$scratch"/labwc.sock ] || { echo "labwc did not start"; cat "$scratch"/labwc.log | tail -5; stop_all; exit 1; }
env -i PATH=$base_path XDG_RUNTIME_DIR="$run" HOME="$scratch"/home USER=root LOGNAME=root LANG=en_GB.UTF-8 SHELL=/bin/sh \
  WAYLAND_DISPLAY="$(cat "$scratch/labwc.sock")" RUST_LOG=info NO_COLOR=1 PIPEWIRE_REMOTE=/nonexistent PULSE_SERVER=/nonexistent \
  setsid dbus-run-session -- sh -c "exec /usr/lib/trindle/bin/trindle-compose --window --start trindle-autostart" > "$scratch"/compose.log 2>&1 < /dev/null &
i=0; until grep -aq 'Listening on wayland socket' "$scratch"/compose.log || [ $i -gt 50 ]; do sleep 0.2; i=$((i+1)); done
sock=$(grep -a -o 'Listening on wayland socket name="[^"]*"' "$scratch"/compose.log | sed 's/.*name=//; s/"//g' | head -1)
[ -n "$sock" ] || { echo "the compositor did not start"; tail -5 "$scratch"/compose.log; stop_all; exit 1; }
compose_pid=""
for p in /proc/[0-9]*; do pid=${p#/proc/}; ours "$pid" && [ "$(cat /proc/"$pid"/comm)" = trindle-compose ] && compose_pid=$pid; done
bus=$(tr '\0' '\n' < /proc/"$compose_pid"/environ | grep '^DBUS_SESSION_BUS_ADDRESS=')
case "$bus" in *"/run/user/"*|"") echo "REFUSED: not a private bus: $bus"; stop_all; exit 1 ;; esac
echo "compositor $compose_pid socket $sock bus $bus"
client() { env -i "$bus" PATH=$base_path XDG_RUNTIME_DIR="$run" HOME="$scratch"/home USER=root LANG=en_GB.UTF-8 WAYLAND_DISPLAY="$sock" TRINDLE_COMPOSE_PID="$compose_pid" PIPEWIRE_REMOTE=/nonexistent PULSE_SERVER=/nonexistent "$@"; }
sleep 10
if [ -n "$picture" ]; then client trindle-wallpaper --set "$picture"; sleep 2; fi
client setsid trindle-keyring > "$scratch"/keyring.log 2>&1 < /dev/null &
client gdbus call --session --dest org.freedesktop.impl.portal.desktop.trindle --object-path /org/freedesktop/portal/desktop \
  --method org.freedesktop.impl.portal.Settings.ReadAll "['org.freedesktop.appearance']" > "$scratch"/portal-before.txt 2>&1
echo "portal answered before: $(head -c 60 "$scratch"/portal-before.txt)"
printf 'kept across the update' | client setsid trindle-clipboard copy > /dev/null 2>&1 &
sleep 3
client gdbus call --session --dest org.freedesktop.secrets --object-path /org/freedesktop/secrets \
  --method org.freedesktop.Secret.Service.OpenSession plain "<''>" > "$scratch"/keyring-before.txt 2>&1
echo "keyring answered before: $(cat "$scratch"/keyring-before.txt)"
owner_before=$(client gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus --method org.freedesktop.DBus.GetNameOwner org.freedesktop.secrets 2>&1)
echo "== before the update"
parts | tee "$scratch"/before.txt

for f in trindle trindle-compose trindle-unlock; do
  cp "$here"/target/release/$f "$scratch"/bin/$f.dpkg-new && chmod 755 "$scratch"/bin/$f.dpkg-new && mv -f "$scratch"/bin/$f.dpkg-new "$scratch"/bin/$f \
    || { echo "could not put the new $f in place, so nothing was updated"; stop_all; exit 1; }
done
echo "== updated at $(date +%T); waiting $WAIT seconds"
( n=0; bad=0; end=$(( $(date +%s) + WAIT - 5 ))
  while [ "$(date +%s)" -lt "$end" ]; do
    n=$((n+1))
    client gdbus call --session --timeout 5 --dest org.freedesktop.secrets --object-path /org/freedesktop/secrets \
      --method org.freedesktop.Secret.Service.SearchItems "{}" > /dev/null 2>"$scratch"/ask.err || { bad=$((bad+1)); cat "$scratch"/ask.err >> "$scratch"/ask-failed.txt; }
  done
  echo "$n $bad" > "$scratch"/asked.txt ) &
( i=0; end=$(( $(date +%s) + WAIT - 5 ))
  while [ "$(date +%s)" -lt "$end" ]; do
    i=$((i+1)); WAYLAND_DISPLAY=$sock LD_LIBRARY_PATH='' $GRIM -t ppm "$scratch/shots/$(printf %04d $i).ppm" 2>/dev/null; sleep 0.15
  done ) &
sleep "$WAIT"
echo "== after the update"
parts | tee "$scratch"/after.txt
owner_after=$(client gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus --method org.freedesktop.DBus.GetNameOwner org.freedesktop.secrets 2>&1)
echo "keyring's bus name before $owner_before after $owner_after"
echo "keyring asked/failed: $(cat "$scratch"/asked.txt 2>/dev/null)"; [ -s "$scratch"/ask-failed.txt ] && head -3 "$scratch"/ask-failed.txt
echo "clipboard now: $(client trindle-clipboard paste 2>&1)"
echo "history: $(grep -c 'kept across the update' "$scratch"/home/.local/share/trindle/clipboard/index.json 2>&1)"
client gdbus call --session --dest org.freedesktop.impl.portal.desktop.trindle --object-path /org/freedesktop/portal/desktop \
  --method org.freedesktop.impl.portal.Settings.ReadAll "['org.freedesktop.appearance']" > "$scratch"/portal-after.txt 2>&1
echo "portal answered after: $(head -c 60 "$scratch"/portal-after.txt)"
grep -a "updated\|renew\|handed\|keyring" "$scratch"/compose.log "$scratch"/keyring.log 2>/dev/null | tail -12
python3 -I - "$scratch/shots" <<'EOF'
import sys, os
d = sys.argv[1]
worst = None
names = sorted(os.listdir(d))
for n in names:
    f = open(os.path.join(d, n), 'rb'); data = f.read(); f.close()
    parts = data.split(b'\n', 3)
    if len(parts) < 4 or parts[0] != b'P6': continue
    w, h = map(int, parts[1].split()); px = parts[3]
    seen = set()
    for y in range(h - 120, h - 20, 7):
        for x in range(w - 300, w - 20, 7):
            o = (y * w + x) * 3
            seen.add(px[o:o + 3])
    if worst is None or len(seen) < worst[0]: worst = (len(seen), n)
print("screenshots:", len(names), "fewest colours in the wallpaper's corner:", worst)
EOF
stop_all
sleep 1
stop_all
rm -rf "$scratch" "$run"
echo "done"
