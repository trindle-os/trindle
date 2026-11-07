# machine translated (Russian), not yet reviewed by a person.

# Name of the login screen's window (what a screen reader and the window list call it).
# SAFETY: needs a human review before it ships
greeter-window-title = Вход

# Sentence at the top of the panel, for someone this computer knows.
greeter-welcome-back = С возвращением.

# Sentence at the top of the panel, when the name has not been typed yet.
greeter-welcome = Добро пожаловать.

# Under the boxes, while there is nothing else to say.
# SAFETY: needs a human review before it ships
greeter-ready = Введите пароль и нажмите Enter.

# Under the boxes, when a paired phone can stand in for the password.
# SAFETY: needs a human review before it ships
greeter-ready-phone = Введите пароль или нажмите Enter, чтобы войти с помощью телефона.

# Under the boxes, before a name is in.
# SAFETY: needs a human review before it ships
greeter-ready-name = Введите имя и пароль и нажмите Enter.

# Red line under the boxes after the phone did not log the person in.
# SAFETY: needs a human review before it ships
greeter-phone-failed = Телефон не выполнил вход. Введите пароль.

# Under the password box for a few seconds, where a phone is paired.
greeter-phone-hint = Кнопка телефона выполняет вход с помощью телефона.

# At the foot of the screen, for when nothing else works.
greeter-trouble = Не получается войти? Нажмите Ctrl+Alt+F2 для текстового входа.

# Beside the light drawing switch, on a computer with no graphics chip.
greeter-no-chip = Графический чип не найден, поэтому облегчённая отрисовка ускоряет работу.

# The main button's name (a round button with an arrow) that logs in.
greeter-sign-in = Войти

# Red line under the boxes when the name and password are wrong, or the account is unknown.
# SAFETY: needs a human review before it ships
greeter-no-match = Имя и пароль не совпадают.

# Red line under the boxes after too many wrong passwords.
# SAFETY: needs a human review before it ships
greeter-pam-max-tries = Слишком много попыток. Подождите минуту и попробуйте снова.

# Red line under the boxes when the account has run out.
# SAFETY: needs a human review before it ships
greeter-pam-account-expired = Срок действия этой учётной записи истёк.

# Red line under the boxes when the password must be changed before logging in.
# SAFETY: needs a human review before it ships
greeter-pam-password-expired = Срок действия этого пароля истёк. Войдите в текстовой консоли (Ctrl+Alt+F2), чтобы сменить его.

# Red line under the boxes, or the sentence at the top, when the login failed and the system gave no reason.
greeter-login-failed = Войти не удалось.

# Red line under the name box when Enter is pressed with no name.
greeter-name-first = Сначала введите имя.

# Sentence at the top while waiting for the phone to answer.
greeter-asking-phone = Запрос к телефону…

# Sentence at the top while the system checks the password.
# SAFETY: needs a human review before it ships
greeter-logging-in = Выполняется вход…

# Typing box for the person's name, shown quietly while empty.
greeter-name-box = Имя

# Typing box for the password, shown quietly while empty.
# SAFETY: needs a human review before it ships
greeter-password-box = Пароль

# Typing box for an extra question the login asked (a code, for one), when the question itself is empty.
greeter-answer-box = Ответ

# Name and tip of the round button that logs in with a paired phone.
greeter-use-phone = Войти с телефоном

# Name of the switch for light drawing on a computer with no graphics chip, and the word under it.
greeter-light-drawing = Облегчённая отрисовка

# Tip of the round button that shows or hides the on-screen keyboard.
greeter-keyboard-tip = Экранная клавиатура

# Word under the on-screen keyboard button.
greeter-keyboard = Клавиатура

# Tip and word under the round button that turns the screen reader on or off.
greeter-screen-reader = Экранный диктор

# Word under a power button after the first press, and its tip.
greeter-press-again = Нажмите ещё раз

# Tip and word under the round button that restarts the computer.
greeter-restart = Перезагрузить

# Tip of the restart button after the first press.
greeter-restart-sure = Нажмите ещё раз, чтобы перезагрузить

# Tip and word under the round button that turns the computer off.
greeter-shut-down = Выключить

# Tip of the shut down button after the first press.
greeter-shut-down-sure = Нажмите ещё раз, чтобы выключить

# Red line under the boxes when a power button is pressed and the system has no bus to ask.
greeter-no-bus = Системная шина недоступна.

# Red line under the boxes when the system refused to restart; the reason follows, as the system worded it.
greeter-restart-failed = Не удалось перезагрузить: { $why }

# Red line under the boxes when the system refused to turn off; the reason follows, as the system worded it.
greeter-shut-down-failed = Не удалось выключить: { $why }

# Red line under the boxes when the login service could not be spoken to.
greeter-service-no-talk = Не удалось связаться со службой входа.

# Red line under the boxes when the login service was too slow.
greeter-service-timeout = Служба входа не ответила вовремя.

# Red line under the boxes when the login service hung up.
greeter-service-closed = greetd закрыл соединение.

# Red line under the boxes when the login service's answer was far too large.
greeter-service-too-big = Служба входа прислала слишком большой ответ, его не удалось прочитать.

# Red line under the boxes when the login screen was started by hand and not by the login service.
greeter-service-not-started = GREETD_SOCK не задан: экран входа запущен не из greetd.

# Red line under the boxes when there is no connection to the login service.
greeter-service-no-connection = нет соединения
