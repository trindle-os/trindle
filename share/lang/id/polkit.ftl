# machine translated (Indonesian), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = Kata sandi administrator diperlukan

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = Kata sandi untuk { $user }

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = Izinkan

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = Batal

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = Kata sandi itu salah. Coba lagi.

# Name a screen reader speaks for the password question window.
# SAFETY: needs a human review before it ships
polkit-spoken = Kata sandi Anda

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
       *[other] { $n } permintaan lain sedang menunggu.
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Drive memerlukan kata sandi administrator untuk membuat snapshot sebelum memberi ruang.

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Drive memerlukan kata sandi administrator untuk mematikan file swap saat memberi ruang.

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Drive memerlukan kata sandi administrator untuk menyalakan kembali file swap.
