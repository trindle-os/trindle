
# machine translated (Bengali), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = প্রশাসকের পাসওয়ার্ড দরকার

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = { $user }-এর পাসওয়ার্ড

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = অনুমতি দিন

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = বাতিল করুন

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = পাসওয়ার্ডটি ঠিক ছিল না। আবার চেষ্টা করুন।

# Name a screen reader speaks for the password question window.
polkit-spoken = আপনার পাসওয়ার্ড

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] আরেকটি অনুরোধ অপেক্ষায় আছে।
       *[other] আরও { $n }টি অনুরোধ অপেক্ষায় আছে।
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = জায়গা করার আগে স্ন্যাপশট নিতে ড্রাইভের জন্য অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড লাগে।

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = জায়গা করার সময় সোয়াপ ফাইল বন্ধ করতে ড্রাইভের জন্য অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড লাগে।

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = সোয়াপ ফাইল আবার চালু করতে ড্রাইভের জন্য অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড লাগে।
