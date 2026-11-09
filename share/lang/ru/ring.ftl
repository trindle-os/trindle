# machine translated (Russian), not yet reviewed by a person.

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = Закрыть

# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = Открыть

# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = Нечего закрывать

# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = Параметры

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = Использование:

# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [ПАРАМЕТР…]

# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = Параметры:

# Help for the ring's --say option, printed in a terminal.
ring-cli-say = Выполнить то, о чём просят эти слова, как если бы их сказали кольцу (для проверки голосовых команд без микрофона)

# Help for the ring's --background option, printed in a terminal.
ring-cli-background = Запустить без показа, чтобы первое открытие было мгновенным

# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = Убрать все окна, чтобы показать рабочий стол, или вернуть их (так работает Super и D)

# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = Открыть кольцо сразу в режиме прослушивания, как это делает сочетание клавиш

# Help for the ring's --all option, printed in a terminal.
ring-cli-all = Открыть кольцо со всеми приложениями вокруг

# Help for the ring's --power option, printed in a terminal.
ring-cli-power = Открыть кольцо на странице выхода, перезагрузки и выключения

# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = Открыть страницу избранного в параметрах

# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = Сразу применить новый вид (светлый или тёмный, или новый акцент) из параметров

# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = Снова создать сенсорные точки в верхних углах

# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = Перевести клавиатуру на круги уведомлений на экране (так работает Super и N)

# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = неизвестный параметр { $option }

# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = для --say не указан аргумент

# Line in the ring's results when what was typed matches no app or favourite.
ring-find-nothing = Нет приложения или избранного с таким названием

# Tip of the round button on an app's menu that removes the app.
# SAFETY: needs a human review before it ships
ring-uninstall = Удалить приложение

# Line that replaces the uninstall button while the app is being removed.
# SAFETY: needs a human review before it ships
ring-uninstalling = Удаление приложения

# Line that replaces the uninstall button once the app is removed.
# SAFETY: needs a human review before it ships
ring-uninstalled = Приложение удалено

# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = Убрать из кольца

# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = Убираем

# Line that replaces that button once the app is off the ring.
ring-taken-off = Убрано

# Name of the ring's round button that opens Apps.
ring-tool-apps = Приложения

# Name of the ring's round button that opens Activity.
ring-tool-activity = Активность

# Name of the ring's round button that opens Drives.
ring-tool-drives = Диски

# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = Снимок экрана

# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = Буфер обмена

# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = Уведомления

# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = Лупа

# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = Пипетка

# Name of the ring's round button that opens the report a problem window.
ring-tool-report = Сообщить о проблеме

# Line in the ring when the item that was pressed is not there any more.
ring-gone = Этого больше нет.

# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Ничто не смогло это открыть.

# Line in the ring while it listens to a spoken command.
ring-listening = Слушаю... говорите, остановлюсь, когда вы закончите

# Line in the ring while it works out what was said.
ring-working-out = Разбираю...

# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = Нечего закрывать

# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = Сейчас { $time }, { $date }

# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = Яркость этого экрана нельзя изменить отсюда

# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = Экранная клавиатура не установлена

# Screen reader name of the ring of open windows.
ring-open-windows = Открытые окна

# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = Восстановить

# Tip of the button on a window's menu that minimises it.
ring-window-minimise = Свернуть

# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = Вернуть размер

# Tip of the button on a window's menu that maximises it.
ring-window-maximise = Развернуть

# Tip of the button on a window's menu that stops a program that does not answer.
# SAFETY: needs a human review before it ships
ring-window-force-close = Принудительно закрыть

# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = Все приложения

# Name of the ring's round button that opens the power page.
ring-power = Питание

# Screen reader description of the ring's power button.
ring-power-spoken = Выйти, перезагрузить или выключить

# Screen reader name of the ring's results while something is typed.
ring-finding = Поиск: { $typed }

# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = Дата и время

# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = Новое окно

# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
        [one] Закрыть окно
        [few] Закрыть все { $n } окна
        [many] Закрыть все { $n } окон
       *[other] Закрыть все { $n } окна
    }

# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = Открепить от кольца

# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = Убрать из избранного

# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = Добавить в избранное

# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = Не выполнено

# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    В середине помещается: { $n }.
    Уберите одно, чтобы освободить место.

# Choice on the ring's power page: lock the screen.
ring-power-lock = Заблокировать

# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = Заставка

# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = Сон

# Choice on the ring's power page: hibernate.
ring-power-hibernate = Гибернация

# Choice on the ring's power page: log out.
# SAFETY: needs a human review before it ships
ring-power-log-out = Выйти

# Choice on the ring's power page: restart the computer.
# SAFETY: needs a human review before it ships
ring-power-restart = Перезагрузить

# Choice on the ring's power page: turn the computer off.
# SAFETY: needs a human review before it ships
ring-power-shut-down = Выключить

# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = Нажмите «Все приложения», затем щёлкните правой кнопкой по приложению, чтобы оставить его здесь

# Line in the ring when a spoken or typed power choice needs a click to confirm.
# SAFETY: needs a human review before it ships
ring-power-only-click = Подтвердить это можно только щелчком.

# Line in the ring when something other than the person tried to shut the computer down.
# SAFETY: needs a human review before it ships
ring-power-only-own = Выключить этот компьютер можете только вы.

# Line in the ring when the power page closed because a program pressed a key.
# SAFETY: needs a human review before it ships
ring-power-closed = Страница питания закрылась: клавишу нажала программа, а не вы.

ring-startup-add = Запускать при входе

ring-startup-remove = Не запускать при входе



# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = Показать список всех быстрых клавиш или убрать его (Super и K, если не изменено в «Настройках»)

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = Перезапущено

# Under the notice above.
ring-restarted-why = Кольцо и вспомогательные части запущены снова.

