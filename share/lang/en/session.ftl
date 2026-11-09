# Trindle's session: the lock screen's words (the password and PIN card, the
# phone check and what it says when it fails), the "One moment, please" card
# shown while the desktop comes up, and the note on wallpapers that cost
# more memory than they need.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The lock screen (src/session/lock.rs)

# Heading of the card when the PIN is the way in.
session-lock-enter-pin = Enter your PIN
# Heading of the card when the password is the way in.
session-lock-locked = Locked
# Under the heading of the password card: what to do.
session-lock-type-password = Type your password, then press Enter
# Button under the PIN card that changes to typing the password.
session-lock-use-password = Use password
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = Use PIN
# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = Use phone
# Shown while the typed password or PIN is being checked.
session-lock-checking = Checking...
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = The PIN has timed out; type your password.
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = Too many wrong PINs; type your password.
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [one] Wrong PIN. { $n } try left.
       *[other] Wrong PIN. { $n } tries left.
    }
# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Asking your phone...
# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Your phone did not unlock it. Use your PIN.
# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Your phone did not unlock it. Type your password.
# Shown when the password typed is not the person's password.
session-lock-wrong-password = That password is not right.

## One moment, please (src/session/starting.rs)

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = One moment, please

## Wallpapers that cost more than they need (src/session/wallpaper_limits.rs)

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = This picture is bigger than your screen and uses extra memory.
# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Moving wallpapers use more memory and battery.
# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = larger than 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = larger than your screen
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = over 20 MB (200 MB if moving)
# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = over 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = over 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } or { $second }
# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count } left out: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = That took too long. Try again.

## The question before everything is restarted (src/session/restart_ask.rs)

# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = Restart the whole desktop?
# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Every open window will close, and anything not saved in them will be lost.
# Tip of the round button that leaves everything as it is.
session-restart-cancel = Cancel
# Main button that restarts the whole desktop.
session-restart-go = Restart
