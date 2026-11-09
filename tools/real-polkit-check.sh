#!/bin/sh
# Holds Trindle's question to the real polkit: whether polkit would ask
set -eu

here=$(cd "$(dirname "$0")/.." && pwd)
cd "$here"
unset DISPLAY WAYLAND_DISPLAY DBUS_SESSION_BUS_ADDRESS

if [ "$(id -u)" = 0 ]; then
    echo "Run this as an ordinary login: root is allowed everything, so nothing is told apart." >&2
    exit 2
fi
if ! command -v pkaction >/dev/null 2>&1; then
    echo "There is no polkit on this computer (no pkaction), so nothing was checked. Run this on a computer with polkit." >&2
    exit 2
fi

if ! command -v loginctl >/dev/null 2>&1; then
    echo "There is no loginctl on this computer, so whether this login is the one in use could not be found out and nothing was checked." >&2
    exit 2
fi
if ! said=$(loginctl show-session self -p Active -p Remote -p Class 2>/dev/null); then
    echo "This was not started from inside a login (the login service knows no session for it), so polkit would answer it as it answers a stranger and nothing was checked. Run it from a terminal at the computer itself." >&2
    exit 2
fi
for want in Active=yes Remote=no Class=user; do
    if ! printf '%s\n' "$said" | grep -qx "$want"; then
        echo "This login is not the one in use at the computer (wanted $want; the login service says: $(printf '%s' "$said" | tr '\n' ' ')), so polkit would answer it differently and nothing was checked. Run this from a terminal at the computer itself." >&2
        exit 2
    fi
done

exec cargo +1.85 test --release -j 4 --bin trindle -- --ignored --exact common::refused::tests::the_real_polkit_answers_as_the_stand_in_does --nocapture
