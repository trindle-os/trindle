# machine translated (Bengali), not yet reviewed by a person.

# Heading of the card when the PIN is the way in.
# SAFETY: needs a human review before it ships
session-lock-enter-pin = আপনার পিন লিখুন

# Heading of the card when the password is the way in.
session-lock-locked = তালাবদ্ধ

# Under the heading of the password card: what to do.
# SAFETY: needs a human review before it ships
session-lock-type-password = আপনার পাসওয়ার্ড লিখে Enter চাপুন

# Button under the PIN card that changes to typing the password.
# SAFETY: needs a human review before it ships
session-lock-use-password = পাসওয়ার্ড ব্যবহার করুন

# Button under the password card that changes to typing a PIN.
# SAFETY: needs a human review before it ships
session-lock-use-pin = পিন ব্যবহার করুন

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = ফোন ব্যবহার করুন

# Shown while the typed password or PIN is being checked.
session-lock-checking = যাচাই হচ্ছে...

# Shown when the PIN has run out of time and the password is asked for instead.
# SAFETY: needs a human review before it ships
session-lock-pin-timed-out = পিনের সময় শেষ; আপনার পাসওয়ার্ড লিখুন।

# Shown after too many wrong PINs, and the password is asked for instead.
# SAFETY: needs a human review before it ships
session-lock-too-many-pins = ভুল পিন অনেকবার দেওয়া হয়েছে; আপনার পাসওয়ার্ড লিখুন।

# Shown after a wrong PIN, with how many tries are left (1 is one try left).
# SAFETY: needs a human review before it ships
session-lock-wrong-pin =
    { $n ->
        [one] পিন ভুল। আর { $n } বার চেষ্টা বাকি।
       *[other] পিন ভুল। আর { $n } বার চেষ্টা বাকি।
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = আপনার ফোনকে জিজ্ঞাসা করা হচ্ছে...

# Shown when the phone did not unlock the screen and the PIN may be typed.
# SAFETY: needs a human review before it ships
session-lock-phone-no-pin = আপনার ফোন এটি খুলতে পারেনি। আপনার পিন ব্যবহার করুন।

# Shown when the phone did not unlock the screen and the password must be typed.
# SAFETY: needs a human review before it ships
session-lock-phone-no-password = আপনার ফোন এটি খুলতে পারেনি। আপনার পাসওয়ার্ড লিখুন।

# Shown when the password typed is not the person's password.
# SAFETY: needs a human review before it ships
session-lock-wrong-password = পাসওয়ার্ডটি ঠিক নয়।

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = একটু অপেক্ষা করুন

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = এই ছবিটি আপনার পর্দার চেয়ে বড় এবং বাড়তি মেমরি নেয়।

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = চলমান ওয়ালপেপার বেশি মেমরি ও ব্যাটারি খরচ করে।

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = 4K-এর চেয়ে বড়

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = আপনার পর্দার চেয়ে বড়

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = ২০ MB-এর বেশি (চলমান হলে ২০০ MB)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = ২০ MB-এর বেশি

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = ২০০ MB-এর বেশি

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } বা { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count }টি বাদ গেছে: { $why }।

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = এতে অনেক সময় লেগেছে। আবার চেষ্টা করুন।



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = পুরো ডেস্কটপ আবার চালু করবেন?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = খোলা সব উইন্ডো বন্ধ হবে, আর তাতে যা সংরক্ষণ করা হয়নি তা হারিয়ে যাবে।

# Tip of the round button that leaves everything as it is.
session-restart-cancel = বাতিল করুন

# Main button that restarts the whole desktop.
session-restart-go = আবার চালু করুন

