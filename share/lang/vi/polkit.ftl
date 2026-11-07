
# machine translated (Vietnamese), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = Cần mật khẩu quản trị viên

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = Mật khẩu của { $user }

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = Cho phép

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = Huỷ

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = Mật khẩu không được chấp nhận. Hãy thử lại.

# Name a screen reader speaks for the password question window.
# SAFETY: needs a human review before it ships
polkit-spoken = Xác thực

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
       *[other] Còn { $n } yêu cầu khác đang chờ yêu cầu này.
    }
