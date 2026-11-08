# machine translated (German), not yet reviewed by a person.

# Heading of the card when the PIN is the way in.
# SAFETY: needs a human review before it ships
session-lock-enter-pin = Gib deine PIN ein

# Heading of the card when the password is the way in.
session-lock-locked = Gesperrt

# Under the heading of the password card: what to do.
# SAFETY: needs a human review before it ships
session-lock-type-password = Gib dein Passwort ein und drücke die Eingabetaste

# Button under the PIN card that changes to typing the password.
# SAFETY: needs a human review before it ships
session-lock-use-password = Passwort verwenden

# Button under the password card that changes to typing a PIN.
# SAFETY: needs a human review before it ships
session-lock-use-pin = PIN verwenden

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = Handy verwenden

# Shown while the typed password or PIN is being checked.
session-lock-checking = Wird geprüft...

# Shown when the PIN has run out of time and the password is asked for instead.
# SAFETY: needs a human review before it ships
session-lock-pin-timed-out = Die PIN ist abgelaufen. Gib dein Passwort ein.

# Shown after too many wrong PINs, and the password is asked for instead.
# SAFETY: needs a human review before it ships
session-lock-too-many-pins = Zu viele falsche PINs. Gib dein Passwort ein.

# Shown after a wrong PIN, with how many tries are left (1 is one try left).
# SAFETY: needs a human review before it ships
session-lock-wrong-pin =
    { $n ->
        [one] Falsche PIN. { $n } Versuch übrig.
       *[other] Falsche PIN. { $n } Versuche übrig.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Dein Handy wird gefragt...

# Shown when the phone did not unlock the screen and the PIN may be typed.
# SAFETY: needs a human review before it ships
session-lock-phone-no-pin = Dein Handy hat nicht entsperrt. Verwende deine PIN.

# Shown when the phone did not unlock the screen and the password must be typed.
# SAFETY: needs a human review before it ships
session-lock-phone-no-password = Dein Handy hat nicht entsperrt. Gib dein Passwort ein.

# Shown when the password typed is not the person's password.
# SAFETY: needs a human review before it ships
session-lock-wrong-password = Dieses Passwort ist nicht richtig.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = Einen Moment bitte

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = Dieses Bild ist größer als dein Bildschirm und braucht zusätzlichen Speicher.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Bewegte Hintergründe brauchen mehr Speicher und Akku.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = größer als 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = größer als dein Bildschirm

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = über 20 MB (200 MB, wenn bewegt)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = über 20 MB

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = über 200 MB

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } oder { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count } ausgelassen: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = Das hat zu lange gedauert. Versuchen Sie es erneut.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = Die ganze Arbeitsoberfläche neu starten?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Alle offenen Fenster werden geschlossen, und alles darin Ungespeicherte geht verloren.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = Abbrechen

# Main button that restarts the whole desktop.
session-restart-go = Neu starten

