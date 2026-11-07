# machine translated (Russian), not yet reviewed by a person.

# Heading of the card when the PIN is the way in.
session-lock-enter-pin = Введите PIN-код

# Heading of the card when the password is the way in.
session-lock-locked = Заблокировано

# Under the heading of the password card: what to do.
session-lock-type-password = Введите пароль и нажмите Enter

# Button under the PIN card that changes to typing the password.
session-lock-use-password = Ввести пароль

# Button under the password card that changes to typing a PIN.
session-lock-use-pin = Ввести PIN-код

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = Телефон

# Shown while the typed password or PIN is being checked.
session-lock-checking = Проверка...

# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = Время действия PIN-кода истекло, введите пароль.

# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = Слишком много неверных PIN-кодов, введите пароль.

# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [one] Неверный PIN-код. Осталась { $n } попытка.
        [few] Неверный PIN-код. Осталось { $n } попытки.
        [many] Неверный PIN-код. Осталось { $n } попыток.
       *[other] Неверный PIN-код. Осталось { $n } попытки.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Запрос к телефону...

# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Телефон не разблокировал экран. Введите PIN-код.

# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Телефон не разблокировал экран. Введите пароль.

# Shown when the password typed is not the person's password.
session-lock-wrong-password = Неверный пароль.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = Одну минуту, пожалуйста

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = Это изображение больше вашего экрана и занимает лишнюю память.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Движущиеся обои занимают больше памяти и расходуют больше заряда.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = больше 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = больше вашего экрана

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = больше 20 МБ (200 МБ для движущихся)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = больше 20 МБ

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = больше 200 МБ

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } или { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = Пропущено: { $count }. Причина: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = Это заняло слишком много времени. Повторите попытку.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = Перезапустить весь рабочий стол?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Все открытые окна закроются, а всё несохранённое в них будет потеряно.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = Отмена

# Main button that restarts the whole desktop.
session-restart-go = Перезапустить

