
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
polkit-not-accepted = Mật khẩu chưa đúng. Hãy thử lại.

# Name a screen reader speaks for the password question window.
# SAFETY: needs a human review before it ships
polkit-spoken = Mật khẩu của bạn

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
       *[other] Còn { $n } yêu cầu khác đang chờ.
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Ổ đĩa cần mật khẩu của quản trị viên để chụp ảnh nhanh trước khi tạo chỗ trống.

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Ổ đĩa cần mật khẩu của quản trị viên để tắt tệp hoán đổi trong lúc tạo chỗ trống.

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Ổ đĩa cần mật khẩu của quản trị viên để bật lại tệp hoán đổi.
