
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
polkit-not-accepted = পাসওয়ার্ডটি গ্রহণ করা হয়নি। আবার চেষ্টা করুন।

# Name a screen reader speaks for the password question window.
polkit-spoken = পরিচয় যাচাই

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] আরেকটি অনুরোধ এটির শেষ হওয়ার অপেক্ষায় আছে।
       *[other] আরও { $n }টি অনুরোধ এটির শেষ হওয়ার অপেক্ষায় আছে।
    }

