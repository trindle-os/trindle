# machine translated (Arabic), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card when the PIN is the way in.
session-lock-enter-pin = أدخل رمز PIN

# Heading of the card when the password is the way in.
session-lock-locked = مقفل

# SAFETY: needs a human review before it ships
# Under the heading of the password card: what to do.
session-lock-type-password = اكتب كلمة المرور ثم اضغط Enter

# SAFETY: needs a human review before it ships
# Button under the PIN card that changes to typing the password.
session-lock-use-password = استخدام كلمة المرور

# SAFETY: needs a human review before it ships
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = استخدام PIN

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = استخدام الهاتف

# Shown while the typed password or PIN is being checked.
session-lock-checking = جارٍ التحقق...

# SAFETY: needs a human review before it ships
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = انتهت مهلة رمز PIN؛ اكتب كلمة المرور.

# SAFETY: needs a human review before it ships
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = رموز PIN خاطئة كثيرة؛ اكتب كلمة المرور.

# SAFETY: needs a human review before it ships
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [zero] رمز PIN خاطئ. لا محاولات متبقية.
        [one] رمز PIN خاطئ. بقيت محاولة واحدة.
        [two] رمز PIN خاطئ. بقيت محاولتان.
        [few] رمز PIN خاطئ. بقيت { $n } محاولات.
        [many] رمز PIN خاطئ. بقيت { $n } محاولة.
       *[other] رمز PIN خاطئ. بقيت { $n } محاولة.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = جارٍ سؤال هاتفك...

# SAFETY: needs a human review before it ships
# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = لم يفتح هاتفك القفل. استخدم رمز PIN.

# SAFETY: needs a human review before it ships
# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = لم يفتح هاتفك القفل. اكتب كلمة المرور.

# SAFETY: needs a human review before it ships
# Shown when the password typed is not the person's password.
session-lock-wrong-password = كلمة المرور هذه غير صحيحة.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = لحظة من فضلك

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = هذه الصورة أكبر من شاشتك وتستهلك ذاكرة إضافية.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = الخلفيات المتحركة تستهلك ذاكرة وبطارية أكثر.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = أكبر من 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = أكبر من شاشتك

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = أكبر من 20 MB (200 MB إن كانت متحركة)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = أكبر من 20 MB

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = أكبر من 200 MB

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } أو { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = استُبعدت { $count }: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = استغرق ذلك وقتًا طويلًا. حاول مرة أخرى.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = إعادة تشغيل سطح المكتب كله؟

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = ستُغلق كل نافذة مفتوحة، وسيضيع كل ما لم يُحفظ فيها.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = إلغاء

# Main button that restarts the whole desktop.
session-restart-go = إعادة التشغيل

