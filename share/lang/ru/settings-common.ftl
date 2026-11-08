# machine translated (Russian), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Сохранено

# Said beside a button that worked, when nothing more is known.
settings-common-done = Готово

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Не получилось

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Об этой странице

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = О разделе «{ $name }»

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = «{ $name }» уже существует

# Under the question when more of the same kind wait: how many more.
# SAFETY: needs a human review before it ships
settings-common-same-for-rest-more =
    { $n ->
        [one] Сделать так же для остальных (ещё { $n })
        [few] Сделать так же для остальных (ещё { $n })
        [many] Сделать так же для остальных (ещё { $n })
       *[other] Сделать так же для остальных (ещё { $n })
    }

# Label of the switch that answers the same way for all that wait.
# SAFETY: needs a human review before it ships
settings-common-same-for-rest = Сделать так же для остальных

# Tip of the button that keeps the one already there.
settings-common-skip = Пропустить: оставить то, что уже есть

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = Заменить

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = Заменить то, что уже есть, на то, что переносится

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = Тот, который переносится

# Heading of the line that shows the one already there.
settings-common-already-line = Тот, который уже есть

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = Заменить то, что уже есть?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] Прежнее состояние вернётся через { $left } секунду.
        [few] Прежнее состояние вернётся через { $left } секунды.
        [many] Прежнее состояние вернётся через { $left } секунд.
       *[other] Прежнее состояние вернётся через { $left } секунды.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] Прежнее состояние вернётся через { $left } секунду. { $more }
        [few] Прежнее состояние вернётся через { $left } секунды. { $more }
        [many] Прежнее состояние вернётся через { $left } секунд. { $more }
       *[other] Прежнее состояние вернётся через { $left } секунды. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Вернуть сейчас

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Оставить

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = «{ $what }» недоступно

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Установите командой: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = Не удалось открыть веб-страницу: не получилось запустить браузер.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = Не удалось открыть «{ $name }».

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Не удалось сохранить.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Не получилось: нет доступа к системной шине сообщений. Перезагрузите компьютер и попробуйте снова.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Не получилось: нет доступа к шине сообщений этого сеанса. Выйдите и войдите снова, затем повторите.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Не удалось открыть окно выбора. Выйдите и войдите снова, затем повторите.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = Диск заполнен.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Сейчас на диск нельзя записывать.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = Часть системы, которая это делает, не запущена.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = Ответ занял слишком много времени. Попробуйте ещё раз.

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = Пароль не введён.

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = У этой учётной записи нет на это прав.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = На этом компьютере нет чего-то необходимого.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } и { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = Кольцо открывается щелчком у края экрана.

# Word drawn in the middle of that picture.
settings-common-ring-name = Кольцо

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Не удалось связаться с прошивкой ноутбука: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = Прошивка ноутбука не приняла ограничение зарядки.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Этот компьютер не позволяет прекращать зарядку раньше времени.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Не удалось записать { $path }: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Trindle в виртуальной машине

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle работает в { $machine }. Там она работает хорошо, но виртуальная машина никогда не бывает такой быстрой и плавной, как настоящий компьютер, поэтому оценивайте скорость на настоящем оборудовании.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle работает в { $machine } без 3D-графики, поэтому процессору приходится рисовать всё то, что обычно рисует графический чип. Окна, видео и веб-страницы будут работать медленнее, чем на настоящем компьютере, а Trindle отключает анимацию, чтобы помочь. Если компьютер, на котором она запущена, это позволяет, включите 3D-ускорение в настройках экрана виртуальной машины: это сильно поможет. На настоящем компьютере всё это неактуально.
