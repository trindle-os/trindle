
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the login screen's window (what a screen reader and the window list call it).
greeter-window-title = 登录

# Sentence at the top of the panel, for someone this computer knows.
greeter-welcome-back = 欢迎回来。

# Sentence at the top of the panel, when the name has not been typed yet.
greeter-welcome = 欢迎。

# SAFETY: needs a human review before it ships
# Under the boxes, while there is nothing else to say.
greeter-ready = 输入你的密码，然后按 Enter。

# SAFETY: needs a human review before it ships
# Under the boxes, when a paired phone can stand in for the password.
greeter-ready-phone = 输入你的密码，或按 Enter 使用你的手机。

# SAFETY: needs a human review before it ships
# Under the boxes, before a name is in.
greeter-ready-name = 输入你的名字和密码，然后按 Enter。

# SAFETY: needs a human review before it ships
# Red line under the boxes after the phone did not log the person in.
greeter-phone-failed = 你的手机没有让你登录。请输入你的密码。

# Under the password box for a few seconds, where a phone is paired.
greeter-phone-hint = 手机按钮可让你用手机登录。

# At the foot of the screen, for when nothing else works.
greeter-trouble = 登录有困难？按 Ctrl+Alt+F2 进入文字登录。

# Beside the light drawing switch, on a computer with no graphics chip.
greeter-no-chip = 没有找到图形芯片，所以使用轻量绘制以保持流畅。

# SAFETY: needs a human review before it ships
# The main button's name (a round button with an arrow) that logs in.
greeter-sign-in = 登录

# SAFETY: needs a human review before it ships
# Red line under the boxes when the name and password are wrong, or the account is unknown.
greeter-no-match = 该名字和密码不匹配。

# SAFETY: needs a human review before it ships
# Red line under the boxes after too many wrong passwords.
greeter-pam-max-tries = 尝试次数过多。请等待一分钟，然后再试。

# SAFETY: needs a human review before it ships
# Red line under the boxes when the account has run out.
greeter-pam-account-expired = 此账户已过期。

# SAFETY: needs a human review before it ships
# Red line under the boxes when the password must be changed before logging in.
greeter-pam-password-expired = 此密码已过期。请在文字控制台（Ctrl+Alt+F2）登录以更改密码。

# Red line under the boxes, or the sentence at the top, when the login failed and the system gave no reason.
greeter-login-failed = 登录没有成功。

# Red line under the name box when Enter is pressed with no name.
greeter-name-first = 请先输入你的名字。

# Sentence at the top while waiting for the phone to answer.
greeter-asking-phone = 正在询问你的手机…

# Sentence at the top while the system checks the password.
greeter-logging-in = 正在登录…

# Typing box for the person's name, shown quietly while empty.
greeter-name-box = 名字

# SAFETY: needs a human review before it ships
# Typing box for the password, shown quietly while empty.
greeter-password-box = 密码

# Typing box for an extra question the login asked (a code, for one), when the question itself is empty.
greeter-answer-box = 回答

# Name and tip of the round button that logs in with a paired phone.
greeter-use-phone = 使用手机

# Name of the switch for light drawing on a computer with no graphics chip, and the word under it.
greeter-light-drawing = 轻量绘制

# Tip of the round button that shows or hides the on-screen keyboard.
greeter-keyboard-tip = 屏幕键盘

# Word under the on-screen keyboard button.
greeter-keyboard = 键盘

# Tip and word under the round button that turns the screen reader on or off.
greeter-screen-reader = 屏幕阅读器

# SAFETY: needs a human review before it ships
# Word under a power button after the first press, and its tip.
greeter-press-again = 再按一次

# SAFETY: needs a human review before it ships
# Tip and word under the round button that restarts the computer.
greeter-restart = 重启

# SAFETY: needs a human review before it ships
# Tip of the restart button after the first press.
greeter-restart-sure = 再按一次以重启

# SAFETY: needs a human review before it ships
# Tip and word under the round button that turns the computer off.
greeter-shut-down = 关机

# SAFETY: needs a human review before it ships
# Tip of the shut down button after the first press.
greeter-shut-down-sure = 再按一次以关机

# Red line under the boxes when a power button is pressed and the system has no bus to ask.
greeter-no-bus = 系统总线不可用。

# Red line under the boxes when the system refused to restart; the reason follows, as the system worded it.
greeter-restart-failed = 无法重启：{ $why }

# Red line under the boxes when the system refused to turn off; the reason follows, as the system worded it.
greeter-shut-down-failed = 无法关机：{ $why }

# Red line under the boxes when the login service could not be spoken to.
greeter-service-no-talk = 无法与登录服务通信。

# Red line under the boxes when the login service was too slow.
greeter-service-timeout = 登录服务没有及时回应。

# Red line under the boxes when the login service hung up.
greeter-service-closed = greetd 关闭了连接。

# Red line under the boxes when the login service's answer was far too large.
greeter-service-too-big = 登录服务返回的内容太大，无法读取。

# Red line under the boxes when the login screen was started by hand and not by the login service.
greeter-service-not-started = 未设置 GREETD_SOCK：登录界面不是由 greetd 启动的。

# Red line under the boxes when there is no connection to the login service.
greeter-service-no-connection = 没有连接

