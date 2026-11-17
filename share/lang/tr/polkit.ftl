
# machine translated (Turkish), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = Yönetici parolası gerekli

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = { $user } için parola

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = İzin ver

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = İptal

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = Parola doğru değil. Yeniden deneyin.

# Name a screen reader speaks for the password question window.
polkit-spoken = Parolanız

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Başka bir istek bekliyor.
       *[other] { $n } istek daha bekliyor.
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Sürücüler, yer açmadan önce anlık görüntü almak için bir yöneticinin parolasına ihtiyaç duyar.

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Sürücüler, yer açarken takas dosyasını kapatmak için bir yöneticinin parolasına ihtiyaç duyar.

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Sürücüler, takas dosyasını yeniden açmak için bir yöneticinin parolasına ihtiyaç duyar.
