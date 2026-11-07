
# machine translated (Urdu), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card asking for the administrator's password.
polkit-title = منتظم کا پاس ورڈ درکار ہے

# SAFETY: needs a human review before it ships
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = { $user } کا پاس ورڈ

# SAFETY: needs a human review before it ships
# Button that lets the step go ahead once the password is typed.
polkit-allow = اجازت دیں

# SAFETY: needs a human review before it ships
# Button that stops the step and closes the question.
polkit-cancel = منسوخ کریں

# SAFETY: needs a human review before it ships
# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = یہ پاس ورڈ قبول نہیں ہوا۔ دوبارہ کوشش کریں۔

# Name a screen reader speaks for the password question window.
polkit-spoken = تصدیق

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] ایک اور درخواست اس کے ختم ہونے کی منتظر ہے۔
       *[other] { $n } اور درخواستیں اس کے ختم ہونے کی منتظر ہیں۔
    }
