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
polkit-not-accepted = Пароль не принят. Попробуйте ещё раз.

# Name a screen reader speaks for the password question window.
polkit-spoken = Проверка подлинности

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] { $n } запрос ждёт завершения этого.
        [few] { $n } запроса ждут завершения этого.
        [many] { $n } запросов ждут завершения этого.
       *[other] { $n } запроса ждут завершения этого.
    }
