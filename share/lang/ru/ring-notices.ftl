# machine translated (Russian), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Заряд батареи: { $pct }%

# Line under the low battery warning's title.
ring-battery-plug-in = Скоро подключите зарядное устройство.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = Зарядка

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Зарядное устройство отключено

# Line under the title when the charger comes out.
ring-battery-running = Работа от батареи, заряд { $pct }%

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 минут

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 час

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 часов

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = До утра

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = Отложить на 10 минут

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = Отложить на 1 час

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = Отложить на 6 часов

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Отложить до утра

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = Назад к сообщению

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Открепить

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Закрепить

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Отложить

# Tip of the button on the stack of notification circles that clears them all.
# SAFETY: needs a human review before it ships
ring-act-clear-all = Очистить все

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Скопировано

# Tip of the button on a notification circle that copies its message.
ring-act-copy = Копировать сообщение

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Нажмите, чтобы передать сведения вашему помощнику.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = Рабочий стол Trindle

# Name of the login screen in the crash notice.
ring-crash-greeter = Экран входа Trindle

# Name of the lock screen in the crash notice.
ring-crash-lock = Экран блокировки Trindle

# Name of Settings in the crash notice.
ring-crash-settings = Параметры Trindle

# Name of Apps in the crash notice.
ring-crash-apps = Приложения Trindle

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = Trindle: { $name }

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name }: неожиданная остановка

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = Этого диска больше нет

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = Не удалось открыть { $name }

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Спящий режим

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Гибернация

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Заблокировать экран

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Ничего

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Выключить

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = Закрытие крышки делает то, что указано в «Параметры», «Питание»

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Удерживайте в любом месте, чтобы увеличить

# Screen reader name of the magnifier's glass.
ring-magnify-name = Лупа

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Выключить лупу

# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = Выключить микрофон для всех программ

# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = Включить микрофон снова

# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = Включить микрофон снова можете только вы сами.

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = Не удалось переключить микрофон: { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = Программа не найдена

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Отправлено другой программой

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Не удалось запустить

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Скопировано для вашего помощника

# Screen reader name of a notification circle.
ring-notes-name = Уведомление

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Перемещение. Отпустите, чтобы оставить здесь, Escape, чтобы вернуть.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Перемещено

# Said when Escape put a held notification back.
ring-notes-put-back = Возвращено

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Закрепите его, чтобы переместить

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
        [one] { $n } секунда
        [few] { $n } секунды
        [many] { $n } секунд
       *[other] { $n } секунды
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Пока вы не закроете

# Time under a notification from yesterday.
ring-notify-yesterday = Вчера, { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = Снизу справа

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = Снизу слева

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = Сверху слева

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = Сверху справа

# Title of the notice when a plugin that asks for permissions has been given none.
# SAFETY: needs a human review before it ships
ring-plugin-needs = Для { $name } нужно ваше разрешение

# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
# SAFETY: needs a human review before it ships
ring-plugin-cannot-work = Пока не может работать: разрешено 0 из { $asks }.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Скопировано. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = Вышла в сеть

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = Открыты ваши файлы

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
        [one] { $names } и ещё { $more }
        [few] { $names } и ещё { $more }
        [many] { $names } и ещё { $more }
       *[other] { $names } и ещё { $more }
    }

