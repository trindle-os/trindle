# SAFETY: needs a human review before it ships
# machine translated (Hindi), not yet reviewed by a person.

# Heading of the card when the PIN is the way in.
session-lock-enter-pin = अपना PIN डालें

# Heading of the card when the password is the way in.
session-lock-locked = लॉक है

# SAFETY: needs a human review before it ships
# Under the heading of the password card: what to do.
session-lock-type-password = अपना पासवर्ड लिखें, फिर Enter दबाएँ

# SAFETY: needs a human review before it ships
# Button under the PIN card that changes to typing the password.
session-lock-use-password = पासवर्ड इस्तेमाल करें

# SAFETY: needs a human review before it ships
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = PIN इस्तेमाल करें

# SAFETY: needs a human review before it ships
# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = फ़ोन इस्तेमाल करें

# Shown while the typed password or PIN is being checked.
session-lock-checking = जाँच हो रही है...

# SAFETY: needs a human review before it ships
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = PIN का समय खत्म हो गया है; अपना पासवर्ड लिखें।

# SAFETY: needs a human review before it ships
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = गलत PIN बहुत हो गए; अपना पासवर्ड लिखें।

# SAFETY: needs a human review before it ships
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [one] गलत PIN। { $n} कोशिश बची है।
       *[other] गलत PIN। { $n } कोशिशें बची हैं।
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = आपके फ़ोन से पूछा जा रहा है...

# SAFETY: needs a human review before it ships
# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = आपके फ़ोन ने इसे अनलॉक नहीं किया। अपना PIN इस्तेमाल करें।

# SAFETY: needs a human review before it ships
# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = आपके फ़ोन ने इसे अनलॉक नहीं किया। अपना पासवर्ड लिखें।

# SAFETY: needs a human review before it ships
# Shown when the password typed is not the person's password.
session-lock-wrong-password = वह पासवर्ड सही नहीं है।

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = कृपया एक पल रुकें

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = यह तस्वीर आपकी स्क्रीन से बड़ी है और अतिरिक्त मेमोरी लेती है।

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = चलते वॉलपेपर ज़्यादा मेमोरी और बैटरी लेते हैं।

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = 4K से बड़ी

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = आपकी स्क्रीन से बड़ी

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = 20 MB से ऊपर (चलती हो तो 200 MB से ऊपर)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = 20 MB से ऊपर

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = 200 MB से ऊपर

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } या { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count } छोड़े गए: { $why }।

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = इसमें बहुत समय लगा। फिर से कोशिश करें।



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = पूरा डेस्कटॉप रीस्टार्ट करें?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = हर खुली विंडो बंद हो जाएगी, और उनमें जो कुछ सहेजा नहीं गया है वह खो जाएगा।

# Tip of the round button that leaves everything as it is.
session-restart-cancel = रद्द करें

# Main button that restarts the whole desktop.
session-restart-go = रीस्टार्ट करें

