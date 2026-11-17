# Settings page "Display": screens, resolution, size, rotation, arrangement and night light.

## Page (src/settings/panels/display.rs)

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Display
# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = screen monitor resolution size scale zoom bigger smaller rotate refresh night light warm blue light evening eyes
# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = Changes apply at once. If the screen goes wrong, do nothing. It goes back by itself after 15 seconds unless you press Keep. Night light makes the screen warmer in the evening.
# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = Keep this arrangement?
# Sentence at the top of the page while the displays are being found.
settings-display-looking = Checking the displays…
# Sentence at the top of the page when no display was found.
settings-display-none-found = Can't find a display.

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
# Heading of the size line.
settings-display-size = Size
# Heading of the rotation line.
settings-display-rotation = Rotation
# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Use this display
# Small words under that line for the only display in use.
settings-display-use-only = You can't switch off the only display in use.
# Words behind the i of a display's card.
settings-display-card-about = Changes apply at once. If the screen goes wrong, do nothing. It goes back by itself after 15 seconds unless you press Keep. The recommended resolution is the one the screen is made for. The standard size is the one Trindle chose for this screen.

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
settings-display-arrangement-about = Set where the displays sit beside each other, so the pointer can move across the edge they share.
# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = You need two displays to place one beside the other.

## Changes and keeping

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Couldn't change that.
# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
        [one] The displays have changed. They go back in { $seconds } second unless you press Keep.
       *[other] The displays have changed. They go back in { $seconds } seconds unless you press Keep.
    }
# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Couldn't undo the change. { $why }
# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = The arrangement is in use now, but Trindle couldn't save it for the next login.
# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Nothing was changed
# Small words under that line.
settings-display-refused-change = Couldn't make that change.
# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Changes apply at once

## Night light

# Title of the night light card.
settings-display-night-title = Night light
# Words behind the i of that card.
settings-display-night-about = The screen turns warmer in the evening, which is easier on the eyes. It follows sunset and sunrise, worked out from your time zone, or times you set yourself. If Trindle can't tell where you are from your time zone, it uses the times you set.
# Heading of the night light line, and name of its switch.
settings-display-night = Night light
# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Sunset and sunrise in { $zone }.
# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle is using set times because it doesn't know when sunset is.
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
        [one] { $n } display
       *[other] { $n } displays, as they are arranged
    }
# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = No display
# Name of the large picture of the displays.
settings-display-your-displays = Your displays
# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle found no displays.
# Title of the card shown when no display was found.
settings-display-none-title = No displays found
# Heading of the line in that card.
settings-display-none-heading = No displays found
# Title of the card about the graphics card.
settings-display-graphics-title = Graphics
# Words behind the i of that card.
settings-display-graphics-about = This computer has an NVIDIA graphics card that uses the open driver. It works, but a card from about 2022 onwards will feel slow, because the open driver can't raise the card's speed. You can install NVIDIA's own driver instead.
# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = NVIDIA card with the open driver
# Small words under that line.
settings-display-nvidia-about = Cards from 2022 on may run slowly.
# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = What to do about it
