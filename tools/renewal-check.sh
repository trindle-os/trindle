#!/bin/bash
# renewal-check.sh: shows every long-running part become the new program
set -u
W=$(cd "$(dirname "$0")/.." && pwd)
if [ "${1:-}" != --inside ]; then
  unset DISPLAY WAYLAND_DISPLAY DBUS_SESSION_BUS_ADDRESS
  exec unshare -Urm bash "$0" --inside
fi
S=$(mktemp -d "${TMPDIR:-/tmp}/trindle-renewal.XXXXXX")
R=$(mktemp -d /tmp/trr.XXXXXX)
LABWC=${LABWC:-$(command -v labwc)}
GRIM=${GRIM:-$(command -v grim)}
L=$(dirname "$(dirname "$(dirname "$LABWC")")")
unset DISPLAY WAYLAND_DISPLAY DBUS_SESSION_BUS_ADDRESS GDK_BACKEND TRINDLE_COMPOSE_PID XDG_CONFIG_DIRS XDG_DATA_DIRS XDG_SESSION_ID XDG_VTNR XDG_SEAT SYSTEMD_EXEC_PID
export XDG_RUNTIME_DIR=$R
WAIT=${WAIT:-75}

ours() { cat /proc/$1/environ 2>/dev/null | tr "\\0" "\\n" | grep -qx "XDG_RUNTIME_DIR=$R"; }
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
    a0=$(tr '\0' ' ' < /proc/$pid/cmdline | cut -d' ' -f1-2)
    case "$a0" in *trindle*) ;; *) continue ;; esac
    printf '%s\t%s\t%s\n' "$pid" "$a0" "$(readlink /proc/$pid/exe 2>/dev/null || echo unreadable)"
  done | sort -k2
}

stop_all
mkdir -p $S/bin $S/home $S/shots && chmod 700 $R || { echo "could not make the scratch folders"; exit 1; }
for f in trindle trindle-compose trindle-unlock; do
  cp $W/target/release/$f $S/bin/$f && chmod 755 $S/bin/$f || { echo "could not copy $f from the build: build first"; exit 1; }
done
mount --bind $S/bin /usr/lib/trindle/bin || { echo "could not mount the private programs"; exit 1; }
mount -t tmpfs tmpfs /tmp/.X11-unix && chmod 1777 /tmp/.X11-unix
[ "$(stat -c %u /usr/lib/trindle/bin/trindle)" = 0 ] || { echo "the private program does not count as root's in here, so no part would renew"; exit 1; }
cmp -s /usr/lib/trindle/bin/trindle $W/target/release/trindle || { echo "the installed path is not the build"; exit 1; }
mkdir -p $S/home/.config/trindle
PIC=$(ls $W/share/wallpapers/*.jpg $W/share/wallpapers/*.png $W/share/wallpapers/*.webp 2>/dev/null | head -1)
echo "picture: $PIC"

BASEPATH=/usr/local/bin:/usr/bin:/bin
env -i PATH=$BASEPATH XDG_RUNTIME_DIR=$R HOME=$S/home WLR_BACKENDS=headless WLR_RENDERER=pixman WLR_HEADLESS_OUTPUTS=1 LD_LIBRARY_PATH=$L/usr/lib/x86_64-linux-gnu \
  setsid $LABWC -s "sh -c 'echo \$WAYLAND_DISPLAY > $S/labwc.sock; sleep 100000'" > $S/labwc.log 2>&1 < /dev/null &
i=0; until [ -s $S/labwc.sock ] || [ $i -gt 50 ]; do sleep 0.2; i=$((i+1)); done
[ -s $S/labwc.sock ] || { echo "labwc did not start"; cat $S/labwc.log | tail -5; stop_all; exit 1; }
env -i PATH=$BASEPATH XDG_RUNTIME_DIR=$R HOME=$S/home USER=root LOGNAME=root LANG=en_GB.UTF-8 SHELL=/bin/sh \
  WAYLAND_DISPLAY=$(cat $S/labwc.sock) RUST_LOG=info NO_COLOR=1 PIPEWIRE_REMOTE=/nonexistent PULSE_SERVER=/nonexistent \
  setsid dbus-run-session -- sh -c "exec /usr/lib/trindle/bin/trindle-compose --window --start trindle-autostart" > $S/compose.log 2>&1 < /dev/null &
i=0; until grep -aq 'Listening on wayland socket' $S/compose.log || [ $i -gt 50 ]; do sleep 0.2; i=$((i+1)); done
SOCK=$(grep -a -o 'Listening on wayland socket name="[^"]*"' $S/compose.log | sed 's/.*name=//; s/"//g' | head -1)
[ -n "$SOCK" ] || { echo "the compositor did not start"; tail -5 $S/compose.log; stop_all; exit 1; }
C=""
for p in /proc/[0-9]*; do pid=${p#/proc/}; ours $pid && [ "$(cat /proc/$pid/comm)" = trindle-compose ] && C=$pid; done
B=$(tr '\0' '\n' < /proc/$C/environ | grep '^DBUS_SESSION_BUS_ADDRESS=')
case "$B" in *"/run/user/"*|"") echo "REFUSED: not a private bus: $B"; stop_all; exit 1 ;; esac
echo "compositor $C socket $SOCK bus $B"
client() { env -i "$B" PATH=$BASEPATH XDG_RUNTIME_DIR=$R HOME=$S/home USER=root LANG=en_GB.UTF-8 WAYLAND_DISPLAY=$SOCK TRINDLE_COMPOSE_PID=$C PIPEWIRE_REMOTE=/nonexistent PULSE_SERVER=/nonexistent "$@"; }
sleep 10
if [ -n "$PIC" ]; then client trindle-wallpaper --set "$PIC"; sleep 2; fi
client setsid trindle-keyring > $S/keyring.log 2>&1 < /dev/null &
client gdbus call --session --dest org.freedesktop.impl.portal.desktop.trindle --object-path /org/freedesktop/portal/desktop \
  --method org.freedesktop.impl.portal.Settings.ReadAll "['org.freedesktop.appearance']" > $S/portal-before.txt 2>&1
echo "portal answered before: $(head -c 60 $S/portal-before.txt)"
printf 'kept across the update' | client setsid trindle-clipboard copy > /dev/null 2>&1 &
sleep 3
client gdbus call --session --dest org.freedesktop.secrets --object-path /org/freedesktop/secrets \
  --method org.freedesktop.Secret.Service.OpenSession plain "<''>" > $S/keyring-before.txt 2>&1
echo "keyring answered before: $(cat $S/keyring-before.txt)"
OWNER1=$(client gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus --method org.freedesktop.DBus.GetNameOwner org.freedesktop.secrets 2>&1)
echo "== before the update"
parts | tee $S/before.txt

for f in trindle trindle-compose trindle-unlock; do
  cp $W/target/release/$f $S/bin/$f.dpkg-new && chmod 755 $S/bin/$f.dpkg-new && mv -f $S/bin/$f.dpkg-new $S/bin/$f \
    || { echo "could not put the new $f in place, so nothing was updated"; stop_all; exit 1; }
done
echo "== updated at $(date +%T); waiting $WAIT seconds"
( n=0; bad=0; end=$(( $(date +%s) + WAIT - 5 ))
  while [ $(date +%s) -lt $end ]; do
    n=$((n+1))
    client gdbus call --session --timeout 5 --dest org.freedesktop.secrets --object-path /org/freedesktop/secrets \
      --method org.freedesktop.Secret.Service.SearchItems "{}" > /dev/null 2>$S/ask.err || { bad=$((bad+1)); cat $S/ask.err >> $S/ask-failed.txt; }
  done
  echo "$n $bad" > $S/asked.txt ) &
( i=0; end=$(( $(date +%s) + WAIT - 5 ))
  while [ $(date +%s) -lt $end ]; do
    i=$((i+1)); WAYLAND_DISPLAY=$SOCK LD_LIBRARY_PATH= $GRIM -t ppm $S/shots/$(printf %04d $i).ppm 2>/dev/null; sleep 0.15
  done ) &
sleep $WAIT
echo "== after the update"
parts | tee $S/after.txt
OWNER2=$(client gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus --method org.freedesktop.DBus.GetNameOwner org.freedesktop.secrets 2>&1)
echo "keyring's bus name before $OWNER1 after $OWNER2"
echo "keyring asked/failed: $(cat $S/asked.txt 2>/dev/null)"; [ -s $S/ask-failed.txt ] && head -3 $S/ask-failed.txt
echo "clipboard now: $(client trindle-clipboard paste 2>&1)"
echo "history: $(grep -c 'kept across the update' $S/home/.local/share/trindle/clipboard/index.json 2>&1)"
client gdbus call --session --dest org.freedesktop.impl.portal.desktop.trindle --object-path /org/freedesktop/portal/desktop \
  --method org.freedesktop.impl.portal.Settings.ReadAll "['org.freedesktop.appearance']" > $S/portal-after.txt 2>&1
echo "portal answered after: $(head -c 60 $S/portal-after.txt)"
grep -a "updated\|renew\|handed\|keyring" $S/compose.log $S/keyring.log 2>/dev/null | tail -12
python3 -I - "$S/shots" <<'EOF'
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
rm -rf $S $R
echo done
