# machine translated (Russian), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Пакеты Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Отложено на версии { $have }; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Отложено; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Отложено на версии { $have }

# On a thing held back with no update waiting.
store-upd-held = Отложено

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Отложено на версии { $have }; более новых пока нет

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Отложено; более новых пока нет

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] { $n } обновление
        [few] { $n } обновления
        [many] { $n } обновлений
       *[other] { $n } обновления
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] Готово { $n } обновление
        [few] Готово { $n } обновления
        [many] Готово { $n } обновлений
       *[other] Готово { $n } обновления
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, всего { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] { $n } отложено
        [few] { $n } отложено
        [many] { $n } отложено
       *[other] { $n } отложено
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = Чтобы завершить обновление микропрограммы, нужна перезагрузка.

# Headline while the check runs.
store-upd-checking = Проверка обновлений.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Обновляется { $name }. Пока идёт установка, можно продолжать работу.

# Headline while all the updates go in.
store-upd-installing-all = Устанавливаются обновления. Пока идёт установка, можно продолжать работу.

# Headline when the updates are in and the computer must start again.
# SAFETY: needs a human review before it ships
store-upd-restart = Перезагрузите компьютер, чтобы завершить: обновления установлены и вступят в силу при следующем запуске.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Чтобы завершить установленное, нужна перезагрузка.

# Added to the headline with no internet.
store-upd-no-more = Этот компьютер не подключён к интернету, поэтому искать новые обновления нельзя.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Готовых обновлений нет

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Известных этому компьютеру обновлений нет. Пока он не подключён к интернету, искать новые нельзя.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = приложений

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle и Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = микропрограммы

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Не удалось проверить обновления для: { $names }.

# Headline when everything is up to date.
store-upd-current = Всё обновлено.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Последняя проверка: { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Без интернета проверить нельзя

# Red words under a card when a check took too long.
store-upd-too-long = Ответ занял слишком много времени.

# Tip of the round button that checks again.
store-upd-check-again = Проверить снова

# Main button while the check runs.
store-upd-checking-main = Проверка

# Main button while the updates go in.
store-upd-installing-main = Установка

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] Установить { $n } обновление
        [few] Установить { $n } обновления
        [many] Установить { $n } обновлений
       *[other] Установить { $n } обновления
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = Перезагрузить сейчас

# Main button that checks for updates.
store-upd-check-for = Проверить обновления

# Tip of the round button that releases a thing held back.
store-upd-release = Снять отсрочку: { $name }

# Tip of the round button that holds a thing's updates back.
store-upd-hold = Отложить обновления: { $name }

# A card's state while it is being checked.
store-upd-status-checking = Проверка

# A card's state while its updates go in.
store-upd-status-installing = Установка

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Не удалось проверить. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; отложено: { $held }

# A card's state when only held back updates are left.
store-upd-held-back-n = отложено: { $n }

# A card's state when nothing waits.
store-upd-up-to-date = Обновлено

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = Источник: { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = Источник: { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Общая часть, источник: { $from }

# Note under the Apps card.
store-upd-yours-alone = Только ваши, поэтому обновляются без пароля. Приложение приносит с собой общие части, которые нужны его новой версии.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Отложенное обновление остаётся на текущей версии и не входит в «Установить все» здесь и в кольце, пока вы не снимете отсрочку.

# Heading of the Apps card.
store-upd-card-apps = Приложения

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Все плагины обновлены.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] Обновление { $n } плагина отложено.
        [few] Обновления { $n } плагинов отложены.
        [many] Обновления { $n } плагинов отложены.
       *[other] Обновления { $n } плагина отложены.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] У { $n } плагина есть обновление
        [few] У { $n } плагинов есть обновление
        [many] У { $n } плагинов есть обновление
       *[other] У { $n } плагина есть обновление
    }

# Heading of the Plugins card.
store-upd-card-plugins = Плагины

# Held back card: nothing is held back.
store-upd-nothing-held = Ничего не отложено.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Отсрочка снята

# Note under the Held back card: what releasing does.
store-upd-release-why = Освобождённое снова устанавливается вместе с остальным через «Установить все» здесь и в кольце.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = Микропрограмма устройства

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = микропрограмма устройства { $id }

# Heading of the Held back card.
store-upd-card-held = Отложено

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } до { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] { $n } обновление, которое устанавливается само по себе
        [few] { $n } обновления, которые устанавливаются вместе
        [many] { $n } обновлений, которые устанавливаются вместе
       *[other] { $n } обновления, которые устанавливаются вместе
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Источник: Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Показано обновлений Debian: { $shown } из { $total }.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = На этом компьютере установлен Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle и Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Пакеты Debian подогнаны друг к другу, и один пакет, установленный отдельно, может потянуть за собой другие или нарушить согласованность. Поэтому они устанавливаются вместе, одной кнопкой.

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Обновления Trindle и Debian содержат исправления безопасности для всего компьютера, поэтому их нельзя откладывать.

# Heading of the card about the daily security updates.
store-upd-card-security = Обновления безопасности

# On the security card: its state.
store-upd-security-on = Включены, каждый день

# On the security card.
store-upd-security-note = Исправления Debian устанавливаются каждый день без запроса.

# On the security card.
store-upd-security-part = Часть Trindle; отключить нельзя.

# On the security card: that holding back never stops these.
store-upd-never-held = Отсрочка обновления их никогда не останавливает.

# Firmware card: the computer has no updater.
store-upd-fw-none = На этом компьютере нет средства обновления микропрограмм.

# Firmware card: restart to finish.
store-upd-fw-restart = Перезагрузите, чтобы завершить

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Версия { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Новая микропрограмма устанавливается при перезагрузке компьютера.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Установите fwupd, чтобы обновлять микропрограммы.

# Heading of the Firmware card.
store-upd-card-firmware = Микропрограммы

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd не удалось получить свежий список.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Не удалось подключиться к Flathub.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Не удалось получить списки Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Не удалось получить списки Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Не отложено. Не удалось сохранить список отложенного: { $why }.

# The same, for a release.
store-upd-not-released-list = Отсрочка не снята. Не удалось сохранить список отложенного: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Отложено в «Приложениях», но самому Flatpak сообщить не удалось, поэтому другое средство обновления приложений всё же может установить его.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Отсрочка снята в «Приложениях», но самому Flatpak сообщить не удалось, и он может по-прежнему пропускать это. Отложите и снимите отсрочку ещё раз.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak сообщил: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Автоматическая проверка обновлений выключена. Нажмите «Проверить обновления», чтобы посмотреть.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Не проверено

# Tip of the round button that ends an update that is running.
store-upd-stop = Остановить

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Остановка

# Under a row whose update was stopped.
store-upd-stopped = Остановлено. Нажмите «Обновить», чтобы продолжить.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Остановлено. Нажмите «Установить», чтобы продолжить.

# Note under the Apps card.
store-upd-steam-restarts = Steam перезапустится, когда не будет запущена ни одна игра.

