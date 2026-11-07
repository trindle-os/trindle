# machine translated (Swahili), not yet reviewed by a person.

# Heading of the card when the PIN is the way in.
# SAFETY: needs a human review before it ships
session-lock-enter-pin = Weka PIN yako

# Heading of the card when the password is the way in.
session-lock-locked = Imefungwa

# Under the heading of the password card: what to do.
# SAFETY: needs a human review before it ships
session-lock-type-password = Andika nenosiri lako, kisha bonyeza Enter

# Button under the PIN card that changes to typing the password.
# SAFETY: needs a human review before it ships
session-lock-use-password = Tumia nenosiri

# Button under the password card that changes to typing a PIN.
# SAFETY: needs a human review before it ships
session-lock-use-pin = Tumia PIN

# Name shown beside the phone button, to unlock with the person's phone.
# SAFETY: needs a human review before it ships
session-lock-use-phone = Tumia simu

# Shown while the typed password or PIN is being checked.
session-lock-checking = Inakagua...

# Shown when the PIN has run out of time and the password is asked for instead.
# SAFETY: needs a human review before it ships
session-lock-pin-timed-out = Muda wa PIN umeisha; andika nenosiri lako.

# Shown after too many wrong PINs, and the password is asked for instead.
# SAFETY: needs a human review before it ships
session-lock-too-many-pins = PIN nyingi mno zisizo sahihi; andika nenosiri lako.

# Shown after a wrong PIN, with how many tries are left (1 is one try left).
# SAFETY: needs a human review before it ships
session-lock-wrong-pin =
    { $n ->
        [one] PIN si sahihi. Jaribio { $n } limebaki.
       *[other] PIN si sahihi. Majaribio { $n } yamebaki.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Inauliza simu yako...

# Shown when the phone did not unlock the screen and the PIN may be typed.
# SAFETY: needs a human review before it ships
session-lock-phone-no-pin = Simu yako haikufungua. Tumia PIN yako.

# Shown when the phone did not unlock the screen and the password must be typed.
# SAFETY: needs a human review before it ships
session-lock-phone-no-password = Simu yako haikufungua. Andika nenosiri lako.

# Shown when the password typed is not the person's password.
# SAFETY: needs a human review before it ships
session-lock-wrong-password = Nenosiri hilo si sahihi.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = Subiri kidogo, tafadhali

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = Picha hii ni kubwa kuliko skrini yako na inatumia kumbukumbu zaidi.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Mandhari zinazosogea hutumia kumbukumbu na betri zaidi.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = kubwa kuliko 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = kubwa kuliko skrini yako

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = zaidi ya MB 20 (MB 200 ikisogea)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = zaidi ya MB 20

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = zaidi ya MB 200

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } au { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count } zimeachwa: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = Hilo lilichukua muda mrefu sana. Jaribu tena.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = Kuanzisha upya eneo-kazi lote?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Kila dirisha lililo wazi litafungwa, na kilichokuwa ndani yake kisichohifadhiwa kitapotea.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = Ghairi

# Main button that restarts the whole desktop.
session-restart-go = Anzisha upya

