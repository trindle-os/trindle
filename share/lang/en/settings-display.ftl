# Settings page "Display": screens, resolution, size, rotation, arrangement and night light.

## Page (src/settings/panels/display.rs)

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Display
# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = screen monitor resolution size scale zoom bigger smaller rotate refresh night light warm blue light evening eyes
# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = Changes apply at once. If the screen goes wrong, do nothing: it goes back by itself after 15 seconds unless you press Keep. Night light turns the screen warmer in the evening.
# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = Keep this arrangement?
# Sentence at the top of the page while the displays are being found.
settings-display-looking = Looking at the displays…
# Sentence at the top of the page when no display was found.
settings-display-none-found = No displays were found.

## Top sentence

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = No display is in use
# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = One display in use
# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] { $n } display in use
       *[other] { $n } displays in use
    }
# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = night light is off
# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = night light warms the screen from { $from } to { $to }
# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = night light warms the screen from sunset to sunrise
# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

## Lists

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Normal
# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Rotated right
# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Rotated left
# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = Upside down
# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = Right of
# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = Left of
# Choice in the list of where a display sits: above the other.
settings-display-place-above = Above
# Choice in the list of where a display sits: below the other.
settings-display-place-below = Below

## Resolutions and sizes

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }
# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height } at { $hz } Hz
# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (recommended)
# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz
# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (standard)
# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Resolution of { $display }
# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Size on { $display }
# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Rotation of { $display }
# Heading of the resolution line.
settings-display-resolution = Resolution
# Small words under that line.
settings-display-resolution-about = How many dots it shows, how often.
# Heading of the size line.
settings-display-size = Size
# Small words under that line.
settings-display-size-about = How large everything looks on this screen.
# Heading of the rotation line.
settings-display-rotation = Rotation
# Small words under that line.
settings-display-rotation-about = For a screen stood on its side.
# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Use this display
# Small words under that line for the only display in use.
settings-display-use-only = The only display in use cannot be turned off.
# Small words under that line otherwise.
settings-display-use-off = Off, nothing is shown on it.
# Words behind the i of a display's card.
settings-display-card-about = Changes apply at once. If the screen goes wrong, do nothing: it goes back by itself after 15 seconds unless you press Keep. The recommended resolution is the screen's own; standard size is the one Trindle chose for this screen.

## Arrangement

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Place { $display }
# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Place it
# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } relative to { $other }
# Title of the card about where displays sit.
settings-display-arrangement-title = Arrangement
# Words behind the i of that card.
settings-display-arrangement-about = Where the displays sit beside each other, so the pointer moves from one to the next across the edge they share.
# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Use two displays to place one beside the other.

## Changes and keeping

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = The desktop refused it.
# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
        [one] The displays have changed. They go back in { $seconds } second unless you press Keep.
       *[other] The displays have changed. They go back in { $seconds } seconds unless you press Keep.
    }
# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = The change could not be undone. { $why }
# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = The arrangement is in use now, but could not be kept for the next login.
# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Nothing was changed
# Small words under that line.
settings-display-refused-change = The desktop refused the change.
# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Changes apply at once
# Small words under that line; { $seconds } is how long Keep is asked for.
settings-display-keep-asked =
    { $seconds ->
        [one] Keep is asked at the foot of the window for { $seconds } second.
       *[other] Keep is asked at the foot of the window for { $seconds } seconds.
    }

## Night light

# Title of the night light card.
settings-display-night-title = Night light
# Words behind the i of that card.
settings-display-night-about = The screen turns warmer in the evening, which is easier on the eyes. It follows sunset and sunrise, worked out from the time zone, or set times of your own; when the time zone does not say where this computer is, the set times are used.
# Heading of the night light line, and name of its switch.
settings-display-night = Night light
# Small words under that line.
settings-display-night-about-line = Warmer colours in the evening.
# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Sunset and sunrise in { $zone }.
# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = The time zone gives no place: set times.
# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Sunset to sunrise
# Choice in the When list: between times the person sets.
settings-display-when-times = Set times
# Heading of the When line, and name of its list.
settings-display-when = When
# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = From
# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = To
# Heading of the warmth line, and name of its slider.
settings-display-warmth = Warmth
# Small words under the warmth line.
settings-display-warmth-about = How warm the screen turns at night.
# Small word under the left end of the warmth slider.
settings-display-little = A little
# Small word under the right end of the warmth slider.
settings-display-very = Very warm
# Spoken description of the strip of colours night light runs through.
settings-display-strip = From a little warm to very warm

## Pictures and cards

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = No display in use
# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }
# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] { $n } display, side by side as they are arranged
       *[other] { $n } displays, side by side as they are arranged
    }
# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = No display
# Name of the large picture of the displays.
settings-display-your-displays = Your displays
# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = The desktop listed no displays.
# Title of the card shown when no display was found.
settings-display-none-title = No displays found
# Heading of the line in that card.
settings-display-none-heading = Nothing to show on
# Title of the card about the graphics card.
settings-display-graphics-title = Graphics
# Words behind the i of that card.
settings-display-graphics-about = This computer has an NVIDIA card, using the open driver. Everything works. On a card from about 2022 onwards it will feel slow, because the open driver cannot raise the card's speed. NVIDIA's own driver can be installed.
# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = An NVIDIA card, on the open driver
# Small words under that line.
settings-display-nvidia-about = Works; a card from 2022 on feels slow.
# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = What to do about it
