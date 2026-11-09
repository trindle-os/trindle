#!/bin/sh
# What Trindle costs while nothing is happening, and a check that it stays
set -u

ROOT=$(cd "$(dirname "$0")/.." && pwd)
BIN=${BIN:-$ROOT/target/release}
DEVLIBS=${DEVLIBS:-$ROOT/target/devlibs}
SECS=${SECS:-30}
CPU_LIMIT=${CPU_LIMIT:-0.5}
WAKE_LIMIT=${WAKE_LIMIT:-6}
CLOCK_WAKE_LIMIT=${CLOCK_WAKE_LIMIT:-12}
MOVIE_CPU_LIMIT=${MOVIE_CPU_LIMIT:-8}
MOVIE_WAKE_LIMIT=${MOVIE_WAKE_LIMIT:-80}
COMPOSE_CPU_LIMIT=${COMPOSE_CPU_LIMIT:-4}
T=${IDLE_DIR:-$ROOT/target/idle}
OUT=$T/result.txt
PARTS=${*:-alone session lock rest saver settings apps folders ring portal movie}

if [ "$PARTS" = live ]; then
  snap_live() {
    for pid in $(pgrep -u "$(id -u)" -f '(^|/)trindle'); do
      [ -r "/proc/$pid/stat" ] || continue
      ticks=$(sed 's/.*) //' "/proc/$pid/stat" 2>/dev/null | awk '{print $12+$13}')
      wakes=$(cat /proc/$pid/task/*/status 2>/dev/null | awk '/^voluntary_ctxt_switches/{n+=$2} END{print n+0}')
      pss=$(awk '/^Pss:/{print $2}' "/proc/$pid/smaps_rollup" 2>/dev/null)
      name=$({ tr '\0' ' ' < "/proc/$pid/cmdline"; } 2>/dev/null | sed 's/ *$//; s|^[^ ]*/||' | cut -c1-34)
      [ -n "$ticks" ] && [ -n "$name" ] && echo "$pid ${ticks:-0} ${wakes:-0} ${pss:-0} $name"
    done
  }
  A=$(mktemp) && B=$(mktemp) || exit 2
  snap_live > "$A"; sleep "$SECS"; snap_live > "$B"
  awk -v secs="$SECS" -v hz="$(getconf CLK_TCK)" '
    NR == FNR { ticks[$1] = $2; wakes[$1] = $3; next }
    ($1 in ticks) {
      short = $5; for (i = 6; i <= NF; i++) short = short " " $i
      if (short !~ /^trindle/) next
      printf "live      %-34s cpu %6.2f%%  wakes %7.1f/s  mem %6.1f MB\n", short, ($2 - ticks[$1]) * 100 / hz / secs, ($3 - wakes[$1]) / secs, $4 / 1024
    }' "$A" "$B" | sort -k2
  rm -f "$A" "$B"
  exit 0
fi

[ -x "$BIN/trindle" ] && [ -x "$BIN/trindle-compose" ] || { echo "no built programs in $BIN: build first"; exit 2; }
LABWC=$(command -v labwc || echo "$DEVLIBS/labwc/tree/usr/bin/labwc")
[ -x "$LABWC" ] || { echo "labwc is needed, installed or unpacked under $DEVLIBS/labwc/tree"; exit 2; }
WTYPE=$(command -v wtype || echo "$DEVLIBS/wtype/tree/usr/bin/wtype")
REAL_BUS=${DBUS_SESSION_BUS_ADDRESS:-}
unset DISPLAY WAYLAND_DISPLAY GDK_BACKEND TRINDLE_COMPOSE_PID DBUS_SESSION_BUS_ADDRESS

R=$(mktemp -d "${TMPDIR:-/tmp}/trindle-idle.XXXXXX") || exit 2
chmod 700 "$R"
mkdir -p "$T/bin" "$T/home/.config/trindle"
for name in $(grep -o '"trindle-[a-z-]*"' "$ROOT/src/main.rs" | tr -d '"' | sort -u) trindle; do
  ln -sf "$BIN/trindle" "$T/bin/$name"
done
ln -sf "$BIN/trindle-compose" "$T/bin/trindle-compose"
: > "$T/home/.config/trindle/welcome-done"
: > "$OUT"

export XDG_RUNTIME_DIR="$R" HOME="$T/home" PATH="$T/bin:$PATH" NO_COLOR=1
export PIPEWIRE_REMOTE=/nonexistent PULSE_SERVER=/nonexistent
unset XDG_SESSION_ID XDG_CONFIG_HOME XDG_DATA_HOME XDG_CACHE_HOME XDG_STATE_HOME

ours() { { tr '\0' '\n' < "/proc/$1/environ"; } 2>/dev/null | grep -qx "XDG_RUNTIME_DIR=$R"; }

mine() {
  all=""
  for p in /proc/[0-9]*; do
    pid=${p#/proc/}
    [ "$pid" = "$$" ] && continue
    ours "$pid" && all="$all $pid"
  done
  grew=1
  while [ "$grew" = 1 ]; do
    grew=0
    for p in /proc/[0-9]*; do
      pid=${p#/proc/}
      case " $all " in *" $pid "*) continue ;; esac
      parent=$(sed 's/.*) //' "$p/stat" 2>/dev/null | awk '{print $2}')
      case " $all " in *" $parent "*) [ "$pid" = "$$" ] || { all="$all $pid"; grew=1; } ;; esac
    done
  done
  for pid in $all; do echo "$pid"; done
}

stop() {
  for round in 1 2 3; do
    left=$(mine)
    [ -n "$left" ] || break
    for pid in $left; do
      [ "$round" = 3 ] && kill -9 "$pid" 2>/dev/null || kill "$pid" 2>/dev/null
    done
    sleep 0.5
  done
}

finish() {
  stop
  rm -rf "$R"
}
trap 'finish' EXIT
trap 'exit 130' INT TERM

bus() {
  setsid dbus-daemon --session --nofork --address="unix:path=$R/$1" > /dev/null 2>&1 < /dev/null &
  i=0; until [ -S "$R/$1" ] || [ $i -gt 50 ]; do sleep 0.1; i=$((i+1)); done
  [ -S "$R/$1" ]
}

start() {
  bus bus && bus system-bus || { echo "no private bus"; exit 2; }
  export DBUS_SESSION_BUS_ADDRESS="unix:path=$R/bus" DBUS_SYSTEM_BUS_ADDRESS="unix:path=$R/system-bus"
  [ "$DBUS_SESSION_BUS_ADDRESS" != "$REAL_BUS" ] || { echo "that is the real bus"; exit 2; }
  rm -f "$R/outer"
  WLR_BACKENDS=headless WLR_RENDERER=pixman WLR_HEADLESS_OUTPUTS=1 LD_LIBRARY_PATH="$DEVLIBS/labwc/tree/usr/lib/x86_64-linux-gnu" \
    setsid "$LABWC" -s "sh -c 'echo \$WAYLAND_DISPLAY > $R/outer; sleep 100000'" > "$T/labwc.log" 2>&1 < /dev/null &
  i=0; until [ -s "$R/outer" ] || [ $i -gt 50 ]; do sleep 0.2; i=$((i+1)); done
  [ -s "$R/outer" ] || { echo "labwc did not start; see $T/labwc.log"; exit 2; }
  WAYLAND_DISPLAY=$(cat "$R/outer") RUST_LOG=info setsid "$T/bin/trindle-compose" --window --start "$1" > "$T/compose.log" 2>&1 < /dev/null &
  i=0; until grep -aq 'Listening on wayland socket' "$T/compose.log" || [ $i -gt 50 ]; do sleep 0.2; i=$((i+1)); done
  INNER=$(grep -a -o 'Listening on wayland socket name="[^"]*"' "$T/compose.log" | sed 's/.*name=//; s/"//g')
  [ -n "$INNER" ] || { echo "the compositor did not start; see $T/compose.log"; exit 2; }
}

session() {
  start trindle-autostart; sleep 20
  key -k Escape; sleep 3
}

run() {
  log=$T/$(basename "$1").log
  env ${RUN_ENV:-} WAYLAND_DISPLAY="$INNER" setsid "$@" > "$log" 2>&1 < /dev/null &
}

shot() {
  [ -n "${SHOTS:-}" ] && [ -x "${GRIM:-}" ] || return 0
  mkdir -p "$SHOTS"
  WAYLAND_DISPLAY=$INNER LD_LIBRARY_PATH= timeout 10 "$GRIM" "$SHOTS/$1.png" 2>/dev/null
}

key() {
  [ -x "$WTYPE" ] && WAYLAND_DISPLAY=$INNER LD_LIBRARY_PATH= timeout 10 "$WTYPE" "$@"
}

snap() {
  for pid in $(mine); do
    [ -r "/proc/$pid/stat" ] || continue
    ticks=$(sed 's/.*) //' "/proc/$pid/stat" 2>/dev/null | awk '{print $12+$13}')
    wakes=$(cat /proc/$pid/task/*/status 2>/dev/null | awk '/^voluntary_ctxt_switches/{n+=$2} END{print n+0}')
    pss=$(awk '/^Pss:/{print $2}' "/proc/$pid/smaps_rollup" 2>/dev/null)
    name=$({ tr '\0' ' ' < "/proc/$pid/cmdline"; } 2>/dev/null | sed 's/ *$//; s|^[^ ]*/||' | cut -c1-60)
    [ -n "$ticks" ] && [ -n "$name" ] && echo "$pid ${ticks:-0} ${wakes:-0} ${pss:-0} $name"
  done
}

measure() {
  shot "$1-1"
  snap > "$R/before"
  sleep "$SECS"
  snap > "$R/after"
  shot "$1-2"
  awk -v secs="$SECS" -v hz="$(getconf CLK_TCK)" -v part="$1" -v cpu_limit="${2:-$CPU_LIMIT}" -v wake_limit="${3:-$WAKE_LIMIT}" -v compose_limit="${4:-$COMPOSE_CPU_LIMIT}" '
    NR == FNR { ticks[$1] = $2; wakes[$1] = $3; next }
    ($1 in ticks) {
      short = $5; for (i = 6; i <= NF; i++) short = short " " $i
      if (short !~ /^trindle/) next
      short = substr(short, 1, 34)
      cpu = ($2 - ticks[$1]) * 100 / hz / secs
      wk = ($3 - wakes[$1]) / secs
      verdict = (cpu > cpu_limit || wk > wake_limit) ? "OVER" : "ok"
      if (short ~ /^trindle-compose/) verdict = (cpu > compose_limit) ? "OVER" : "ok"
      printf "%-9s %-34s cpu %6.2f%%  wakes %7.1f/s  mem %6.1f MB  %s\n", part, short, cpu, wk, $4 / 1024, verdict
    }' "$R/before" "$R/after" | sort -k2 | tee -a "$OUT"
}

for part in $PARTS; do
  echo "wallpaper=none" > "$T/home/.config/trindle/trindle.conf"
  case $part in
    alone)
      start "sleep 100000"; sleep 5
      measure alone ;;
    lock)
      start "sleep 100000"; sleep 3
      run trindle-lock --now; sleep 6
      measure lock ;;
    rest)
      start "sleep 100000"; sleep 3
      run trindle-lock --now --resting; sleep 6
      measure rest "$CPU_LIMIT" "$CLOCK_WAKE_LIMIT" ;;
    saver)
      start "sleep 100000"; sleep 3
      run trindle-screensaver; sleep 6
      measure saver "$CPU_LIMIT" "$CLOCK_WAKE_LIMIT" ;;
    session)
      session
      measure session ;;
    settings)
      session
      run trindle-settings; sleep 10
      measure settings ;;
    apps)
      session
      run trindle-apps; sleep 30
      measure apps ;;
    folders)
      session
      run trindle-folders; sleep 10
      measure folders ;;
    ring)
      session
      run trindle-ring; sleep 5
      shot ring-open
      key -k Escape; sleep 8
      measure ring ;;
    movie)
      : > "$T/home/.config/trindle/trindle.conf"
      session
      measure movie "$MOVIE_CPU_LIMIT" "$MOVIE_WAKE_LIMIT" 40 ;;
    portal)
      session
      run trindle-portal; sleep 8
      measure portal ;;
    only:*)
      start "sleep 100000"; sleep 3
      run ${part#only:}; sleep 8
      measure only ;;
    with:*)
      session
      run ${part#with:}; sleep 10
      measure with ;;
    *) echo "unknown part $part"; exit 2 ;;
  esac
  stop
done

echo
if grep -q 'OVER$' "$OUT"; then
  echo "At rest, these use more than $CPU_LIMIT% of a processor or wake more than $WAKE_LIMIT times a second:"
  grep 'OVER$' "$OUT"
  exit 1
fi
echo "Every program is quiet at rest. The figures are in $OUT"
