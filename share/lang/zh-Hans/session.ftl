# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card when the PIN is the way in.
session-lock-enter-pin = 请输入您的 PIN 码

# Heading of the card when the password is the way in.
session-lock-locked = 已锁定

# SAFETY: needs a human review before it ships
# Under the heading of the password card: what to do.
session-lock-type-password = 请输入您的密码，然后按 Enter

# SAFETY: needs a human review before it ships
# Button under the PIN card that changes to typing the password.
session-lock-use-password = 使用密码

# SAFETY: needs a human review before it ships
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = 使用 PIN 码

# SAFETY: needs a human review before it ships
# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = 使用手机

# Shown while the typed password or PIN is being checked.
session-lock-checking = 正在检查……

# SAFETY: needs a human review before it ships
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = PIN 码已超时；请输入您的密码。

# SAFETY: needs a human review before it ships
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = PIN 码错误次数过多；请输入您的密码。

# SAFETY: needs a human review before it ships
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
       *[other] PIN 码不对。还剩 { $n } 次机会。
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = 正在询问您的手机……

# SAFETY: needs a human review before it ships
# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = 您的手机没有解锁。请使用您的 PIN 码。

# SAFETY: needs a human review before it ships
# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = 您的手机没有解锁。请输入您的密码。

# SAFETY: needs a human review before it ships
# Shown when the password typed is not the person's password.
session-lock-wrong-password = 密码不对。

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = 请稍候

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = 这张图片比您的屏幕大，会多占用内存。

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = 动态壁纸会占用更多内存和电量。

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = 大于 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = 大于您的屏幕

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = 超过 20 MB（动态的超过 200 MB）

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = 超过 20 MB

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = 超过 200 MB

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first }或{ $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = 有 { $count } 张被略过：{ $why }。

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = 耗时过长。请重试。



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = 重启整个桌面吗？

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = 所有打开的窗口都会关闭，其中未保存的内容将丢失。

# Tip of the round button that leaves everything as it is.
session-restart-cancel = 取消

# Main button that restarts the whole desktop.
session-restart-go = 重启

