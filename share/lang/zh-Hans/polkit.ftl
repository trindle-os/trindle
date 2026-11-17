# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card asking for the administrator's password.
polkit-title = 需要管理员密码

# SAFETY: needs a human review before it ships
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = { $user } 的密码

# SAFETY: needs a human review before it ships
# Button that lets the step go ahead once the password is typed.
polkit-allow = 允许

# SAFETY: needs a human review before it ships
# Button that stops the step and closes the question.
polkit-cancel = 取消

# SAFETY: needs a human review before it ships
# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = 密码不对。请重试。

# Name a screen reader speaks for the password question window.
polkit-spoken = 你的密码

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
       *[other] 还有 { $n } 个请求在等待。
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = 驱动器需要管理员的密码，才能在腾出空间前创建快照。

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = 驱动器需要管理员的密码，才能在腾出空间时关闭交换文件。

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = 驱动器需要管理员的密码，才能重新开启交换文件。
