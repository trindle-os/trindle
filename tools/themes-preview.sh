#!/bin/sh
# Pictures of Trindle in its own look and in each proposed theme, for
set -eu

here=$(cd "$(dirname "$0")/.." && pwd)
out=$here/target/themes-preview
cd "$here"
unset DISPLAY WAYLAND_DISPLAY DBUS_SESSION_BUS_ADDRESS THEMES_PREVIEW

case "$here" in *" "*) echo "The project's folder has a space in its path, which this script does not handle: $here" >&2; exit 2 ;; esac
for name in "$@"; do
    case "$name" in
        ""|-*|*[!a-z0-9-]*) echo "Not a theme's name: $name (small letters, digits and hyphens only)" >&2; exit 2 ;;
    esac
    [ -f "share/themes/$name.css" ] || [ -f "themes-proposed/$name.css" ] || { echo "No theme is called $name: there is no share/themes/$name.css or themes-proposed/$name.css" >&2; exit 2; }
done
mkdir -p "$out/.empty-config"
export XDG_CONFIG_HOME="$out/.empty-config"
if [ $# -gt 0 ]; then
    themes="trindle $*"
else
    themes="trindle"
    for f in share/themes/*.css themes-proposed/*.css; do
        [ -f "$f" ] || continue
        name=$(basename "$f" .css)
        case "$name" in ""|-*|*[!a-z0-9-]*) echo "Skipping $f: not a theme's name" >&2; continue ;; esac
        themes="$themes $name"
    done
fi

run() {
    with=$1
    shift
    env $with cargo +1.85 test --release -j 4 --bin trindle -- --ignored --test-threads=1 "$@" >"$out/.log" 2>&1 || { tail -40 "$out/.log"; exit 1; }
}

cargo +1.85 test --release -j 4 --bin trindle --no-run >"$out/.log" 2>&1 || { tail -40 "$out/.log"; exit 1; }

for theme in $themes; do
    raw="$out/.raw-$theme"
    rm -rf "$raw" "$out/$theme"
    mkdir -p "$raw" "$out/$theme"
    if [ "$theme" = trindle ]; then
        unset TRINDLE_TEST_THEME
    else
        f="$here/share/themes/$theme.css"; [ -f "$f" ] || f="$here/themes-proposed/$theme.css"
        export TRINDLE_TEST_THEME="$f"
    fi
    export TRINDLE_SHOTS="$raw" LOCK_SHOTS="$raw" TRINDLE_SHOT_SIZE=1280x800
    run "TRINDLE_SHOT_PAGES=appearance,network,power" shoot_the_layout_of_every_page
    run "TRINDLE_SHOT_PAGES=appearance TRINDLE_SHOT_TRIAL=1 TRINDLE_SHOT_PREFIX=trying-" shoot_the_layout_of_every_page
    run "A=1" shoot_every_apps_page shoot_ring_open shoot_the_lock_screen shoot_the_password_window shoot_the_gallery
    run "TRINDLE_SHOT_ACCENT=#2295f1 TRINDLE_SHOT_TAIL=-blue" shoot_the_gallery
    n=0
    for pair in "settings-appearance-tall:appearance" "trying-appearance:trying-a-theme" "settings-network:network" "settings-power:power" \
                "apps-discover:apps-discover" "apps-app-installed-update-waiting:apps-one-app" "ring-open:ring" \
                "lock-password-1280:lock-screen" "polkit-1280:password-question" "dropdown-open:dropdown-open" "controls:every-control" \
                "controls-blue:every-control-blue-highlight"; do
        n=$((n + 1))
        cp "$raw/${pair%%:*}.png" "$out/$theme/$(printf '%02d' $n)-${pair##*:}.png"
    done
    if [ "$theme" = trindle ]; then
        run "A=1" shoot_the_login_screen
        cp "$raw/greeter-1280.png" "$out/$theme/14-login-screen.png"
    fi
    rm -rf "$raw"
    echo "$theme: $(find "$out/$theme" -name '*.png' | wc -l) pictures"
done

unset TRINDLE_TEST_THEME
cargo +1.85 test --release -j 4 --bin trindle print_the_contrast_of_the_themes -- --ignored --nocapture 2>/dev/null | grep '^CONTRAST|' >"$out/contrast.txt"
run "THEMES_PREVIEW=$out" write_the_themes_page
rm -rf "$out/.empty-config" "$out/.log"
echo "$out/index.html"
