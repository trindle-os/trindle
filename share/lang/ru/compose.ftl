# machine translated (Russian), not yet reviewed by a person.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = Звук выключен

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = Микрофон выключен

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = Программа

# Notice shown when the ring stopped working.
compose-stopped-ring = Кольцо остановилось. Нажмите Super, чтобы запустить его снова.

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = Снимок экрана остановился и был закрыт.

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = Повторить

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = Экран не заблокирован.

# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = PIN-код не задан.

# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = Время для PIN-кода вышло.

# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = Слишком много попыток.

# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = PIN-код состоит из 4 цифр.

# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
        [one] Неверный PIN-код. Осталась { $left } попытка.
        [few] Неверный PIN-код. Осталось { $left } попытки.
        [many] Неверный PIN-код. Осталось { $left } попыток.
       *[other] Неверный PIN-код. Осталось { $left } попытки.
    }
