# machine translated (Russian), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = Нужен пароль администратора

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = Пароль пользователя { $user }

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = Разрешить

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = Отмена

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = Пароль неверный. Попробуйте ещё раз.

# Name a screen reader speaks for the password question window.
polkit-spoken = Ваш пароль

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Ещё { $n } запрос ждёт.
        [few] Ещё { $n } запроса ждут.
        [many] Ещё { $n } запросов ждут.
       *[other] Ещё { $n } запроса ждут.
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Диски нужен пароль администратора, чтобы сделать снимок перед освобождением места.

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Диски нужен пароль администратора, чтобы выключить файл подкачки, пока освобождается место.

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Диски нужен пароль администратора, чтобы снова включить файл подкачки.
