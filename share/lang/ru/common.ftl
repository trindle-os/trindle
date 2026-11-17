# machine translated (Russian), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Said instead of an Uninstall button on an app that is part of Trindle itself.
common-removal-trindle = Это часть Trindle, поэтому она остаётся.

# SAFETY: needs a human review before it ships
# Said instead of an Uninstall button on an app that the computer cannot work without.
common-removal-needed = Он нужен компьютеру, поэтому остаётся.

# SAFETY: needs a human review before it ships
# Said instead of an Uninstall button on an app whose removal would take a part of Trindle with it.
common-removal-trindle-needs = Он нужен части Trindle, поэтому остаётся.

# SAFETY: needs a human review before it ships
# Red words when an app's saved data was not deleted because its id is not a plain name.
common-removal-data-name = Данные не удалены, потому что в имени приложения есть необычные символы.

# SAFETY: needs a human review before it ships
# Red words when an app's saved data folder is a link or not a folder, so nothing was deleted.
common-removal-data-link = Данные не удалены, потому что вместо его папки там ссылка на другое место.

# SAFETY: needs a human review before it ships
# Red words when an app's saved data could not be deleted.
common-removal-data-failed = Не удалось удалить его данные.

# Name of the updates button, and its tip, while the ring's own update button is putting updates in.
common-updating-short-ring = Обновление

# Name of an update button while Apps is putting updates in.
common-updating-short-apps = Обновление в «Приложениях»

# Name of an update button while the daily security fixes for the apps' boxes are going in.
common-updating-short-boxes = Устанавливаются исправления безопасности

# Name of an update button while Debian's own installer is at work.
common-updating-short-system = Debian обновляется

# Name of an update button while something not named is installing updates.
common-updating-short-unknown = Обновление идёт в другом месте

# Headline of an updates page while the ring is installing updates.
common-updating-sentence-ring = Обновления устанавливаются из кольца.

# Headline of an updates page while Apps is installing updates.
common-updating-sentence-apps = Обновления устанавливаются из «Приложений».

# Headline of an updates page while the daily security fixes are going into the apps' boxes.
common-updating-sentence-boxes = Для ваших приложений устанавливаются исправления безопасности.

# Headline of an updates page while Debian's installer is at work (the daily security updates, apt in a terminal, or another person).
common-updating-sentence-system = Работает установщик Debian, как каждый день при обновлениях безопасности.

# Headline of an updates page while something not named is installing updates.
common-updating-sentence-unknown = Обновления устанавливаются в другом месте.

# Beside an app in a list of updates, when what waits is a newer build of the version already installed. { $version } is the version.
common-updating-newer-build-of = более новая сборка версии { $version }

# Beside an app in a list of updates: the installed version, then the version that waits.
common-updating-version-to = версия { $from } до { $to }

# Beside an app in a list of updates, when only the installed version is known. { $version } is that version.
common-updating-now-version = сейчас версия { $version }

# Beside an app in a list of updates, when only the new version is known. { $version } is that version.
common-updating-to-version = до версии { $version }

# Beside a thing whose own version is shown, when what waits is a newer build of the same version.
common-updating-newer-waiting = ожидает более новая сборка

# Beside a thing whose own version is shown: the version that waits.
common-updating-version-waiting = ожидает версия { $version }

# Beside a thing whose own version is shown, when the new version is not known.
common-updating-update-waiting = ожидает обновление

# SAFETY: needs a human review before it ships
# Said in the password box when Debian's and Trindle's updates are put in together.
common-updating-reason-all = Обновления Debian и Trindle меняют весь компьютер для всех, кто им пользуется, поэтому нужен пароль администратора. Приложения, которые вы установили сами, обновляются без него.

# SAFETY: needs a human review before it ships
# Said in the password box when only Debian's updates are put in.
common-updating-reason-debian = Обновления Debian меняют весь компьютер для всех, кто им пользуется, поэтому нужен пароль администратора.

# SAFETY: needs a human review before it ships
# Said in the password box when only Trindle's own update is put in.
common-updating-reason-trindle = Обновление Trindle меняет весь компьютер для всех, кто им пользуется, поэтому нужен пароль администратора.

# SAFETY: needs a human review before it ships
# Said in the password box when new firmware is put in.
common-updating-reason-firmware = Новая прошивка меняет сам компьютер, поэтому нужен пароль администратора.

# SAFETY: needs a human review before it ships
# Said in the password box when the lists of the whole computer's updates are fetched away from the computer's own screen.
common-updating-reason-refresh = Для поиска обновлений всего компьютера нужен пароль администратора, если запрос идёт не с собственного экрана компьютера.

# The end of a sentence saying why nothing was installed. { $why } is the reason, such as "no administrator's password was given".
common-updating-because = потому что { $why }.

# Red words beside an update that did not finish and gave no reason.
common-updating-no-reason = Не завершилось, причина не указана.

# Red words beside an update that did not finish. { $why } is the reason the system gave.
common-updating-failed = Не завершилось. { $why }

# Red words beside an update that was never started. { $why } is the end of a sentence such as "because no administrator's password was given."
common-updating-not-installed = Не установлено, { $why }

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = пароль администратора не был введён

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = пароль администратора не был принят

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = вашей учётной записи это запрещено, даже с правильным паролем

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = окно ввода пароля так и не появилось

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = пароль администратора не был принят или вашей учётной записи это запрещено

# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = Пароль администратора не был введён

# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = Пароль администратора не был принят

# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = Вашей учётной записи это запрещено, даже с правильным паролем

# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = Окно ввода пароля так и не появилось

# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = Пароль администратора не был принят или вашей учётной записи это запрещено

# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = Не выполнено, потому что пароль администратора не был введён.

# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = Не выполнено, потому что пароль администратора не был принят.

# SAFETY: needs a human review before it ships
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = Не выполнено. Вашей учётной записи это запрещено, даже с правильным паролем. Выйдите из системы и войдите снова, затем повторите попытку.

# SAFETY: needs a human review before it ships
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = Не выполнено, потому что окно ввода пароля так и не появилось. Выйдите из системы и войдите снова, затем повторите попытку.

# SAFETY: needs a human review before it ships
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = Не выполнено. Пароль администратора не был принят, или вашей учётной записи это запрещено. Если окно ввода пароля не появилось, выйдите из системы и войдите снова, затем повторите попытку.

# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Не выполнено. Что-то пошло не так, причина не указана.

# SAFETY: needs a human review before it ships
# Red words when the keyring will not open without the person's password.
common-keyring-locked = Связка ключей заблокирована. Выйдите из системы и войдите снова или разблокируйте её, затем повторите попытку.

# SAFETY: needs a human review before it ships
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = На этом компьютере нет связки ключей, куда это можно сохранить.

# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Связка ключей не ответила вовремя.

# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Связка ключей так и не ответила.

# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Trindle не удалось получить доступ к связке ключей ({ $why }), поэтому она не использовалась.

# SAFETY: needs a human review before it ships
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Связка ключей пока не ответила, поэтому { $what } пока не удалено. Если связка всё же удалит это, здесь будет сказано.
       *[other] Связка ключей пока не ответила, поэтому { $what } пока не сохранено. Если связка всё же сохранит это, здесь будет сказано.
    }

# SAFETY: needs a human review before it ships
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Связка ключей ответила с опозданием: { $what } удалено.
       *[other] Связка ключей ответила с опозданием: { $what } сохранено.
    }

# SAFETY: needs a human review before it ships
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Связка ключей так и не ответила, поэтому { $what } не удалено.
       *[other] Связка ключей так и не ответила, поэтому { $what } не сохранено.
    }

# SAFETY: needs a human review before it ships
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Связка ключей ответила с опозданием и не удалила { $what }: { $why }
       *[other] Связка ключей ответила с опозданием и не сохранила { $what }: { $why }
    }

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the ring.
common-renewed-ring = кольцо

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the window that opens and saves files for other programs.
common-renewed-portal = окно открытия и сохранения

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the wallpaper.
common-renewed-wallpaper = обои

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the clipboard.
common-renewed-clipboard = буфер обмена

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the keyring.
common-renewed-keyring = связка ключей

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that dims the screen and locks it.
common-renewed-idle = затемнение и блокировка

# Name of a part of Trindle, said in the sentence about what is not yet the new version: night light.
common-renewed-nightlight = ночной свет

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that lets other computers find this one.
common-renewed-wsdd = обнаружение в сети

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the plugins that run in the background.
common-renewed-plugin = плагины, работающие в фоне

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the desktop itself.
common-renewed-compose = сам рабочий стол

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the link an AI assistant is using.
common-renewed-mcp = связь, которую использует ИИ-помощник

# Joins two or more names of parts in a list, all but the last: "the ring, the keyring". { $first } is the list so far; { $next } is the next name.
common-renewed-join = { $first }, { $next }

# Joins the last name of a list of parts to the rest: "the ring, the keyring and night light". { $most } is the list so far.
common-renewed-join-last = { $most } и { $last }

# Sentence on Settings and Apps after an update, when some parts stay the old program until the next log out. { $list } is the names of the parts.
common-renewed-sentence = Это обновление уже используется, кроме следующего: { $list }. Новая версия начнёт работать после следующего выхода из системы.

# Sentence on Settings and Apps after an update, when some parts may still be the old program and will change at the next safe moment or the next log out. { $list } is the names of the parts.
common-renewed-sentence-latest = Это обновление уже используется, кроме следующего: { $list }. Новая версия начнёт работать не позже следующего выхода из системы.

# Title of the notice, and red words in Apps, when the older-programs display for a box would not start.
common-x11-would-not-start = Xwayland не запустился

# Second line of that notice, when the desktop will try the older-programs display again soon.
common-x11-try-again = Повторите через минуту

# Head of the card in Settings, Permissions, and of the page in Welcome, that shows each dot with what it means.
common-in-use-key = Что означают точки

# Under the dots in that key: where a dot shows up.
common-in-use-where = Точки показываются вверху по центру экрана и на вкладках окон.

# Screen reader name of the microphone dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-microphone = Микрофон используется

# Line on pointing at the dots, and its screen reader name, when programs use the microphone. { $who } is the programs' names.
common-in-use-microphone-by = Микрофон используется: { $who }

# Screen reader name of the camera dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-camera = Камера используется

# Line on pointing at the dots, and its screen reader name, when programs use the camera. { $who } is the programs' names.
common-in-use-camera-by = Камера используется: { $who }

# Screen reader name of the screen dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-screen = Идёт запись экрана

# Line on pointing at the dots, and its screen reader name, when programs record the screen. { $who } is the programs' names.
common-in-use-screen-by = Запись экрана ведёт: { $who }

# Screen reader name of the input dot and its row, and the line shown on pointing at the dots, when no program is named: the mouse and keyboard are in another program's hands.
common-in-use-input = Мышь и клавиатура используются

# Line on pointing at the dots, and its screen reader name, when programs have taken the mouse and keyboard. { $who } is the programs' names.
common-in-use-input-by = { $who } управляет вашей клавиатурой и мышью

# Screen reader name of the copied files dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-files = Скопированные вами файлы читаются

# Line on pointing at the dots, and its screen reader name, when programs read the files the person just copied. { $who } is the programs' names.
common-in-use-files-by = { $who } читает скопированные вами файлы

# The ring's who-is-using-what row while a plugin reaches devices on the local network.
common-in-use-home-network = Локальная сеть используется

# The same with the plugin's name.
common-in-use-home-network-by = Локальную сеть использует: { $who }

# The ring's who-is-using-what row while a plugin uses scanners and USB devices.
common-in-use-usb = Сканеры и USB-устройства используются

# The same with the plugin's name.
common-in-use-usb-by = Сканеры и USB-устройства использует: { $who }

# Line on pointing at the dots when the microphone is switched off and the program listening hears nothing.
common-in-use-microphone-off = Микрофон выключен

# Line on pointing at the dots when the microphone is switched off but programs want it. { $who } is the programs' names.
common-in-use-microphone-off-by = Микрофон выключен. Попытка использовать его: { $who }.

# Line under a microphone's mute switch in Settings and the ring's sound page while the microphone is off for every program.
common-in-use-microphone-off-note = Вы выключили его для всех программ кнопкой микрофона на кольце.

# Said where the person is working when a file of their own was damaged, was put aside beside itself and a new one begun. { $what } names what the file keeps, such as "Launch counts".
common-damaged-reset = { $what } сброшено, потому что его файл был повреждён. Повреждённый файл оставлен рядом.

# Said where the person is working when a file of their own is there but cannot be read, so nothing is saved into it and it is left as it is. { $what } names what the file keeps, such as "Launch counts".
common-unreadable-kept = { $what } не сохранено, потому что файл не удалось прочитать. Ничего не изменено.

# What the file kept by the list of programs a box may use is called, in the words above.
common-unreadable-name-box-permissions = Разрешения приложений

# What the file that notes which questions an app has already put to the person is called, in the words above.
common-unreadable-name-box-asked = Уже заданные вопросы

# What the file that notes which apps came with the image is called, in the words above.
common-unreadable-name-box-offered = Приложения, предложенные при первом входе

# What the lists of a fenced program (its folders, hosts, settings and programs) are called, in the words above.
common-unreadable-name-fence = Списки ограждения

# What the list of icons the desktop found missing is called, in the words above.
common-unreadable-name-missing-icons = Список недостающих значков

# What the screen arrangement an older version kept in display.sh is called, in the words above.
common-unreadable-name-old-display = Расположение экранов

# Said where the person works when a known chip has no driver yet. The kernel is a version number.
common-no-driver-known-driver = Для { $chip } здесь пока нет драйвера. Впервые он входит в Linux { $kernel }.

# Said when a known chip is missing a firmware file. The package and version name where it comes from.
common-no-driver-known-firmware = Для { $chip } не хватает прошивки. Она есть в { $package } { $version }.

# Said of a chip nothing is known about that has no driver. The class is Wi-Fi, Bluetooth, sound or graphics; the id is two hex numbers.
common-no-driver-generic = Для этого чипа { $class } ({ $id }) нет драйвера.

# Said of a chip nothing is known about whose firmware file could not be loaded.
common-no-driver-generic-firmware = Не хватает прошивки ({ $class }): { $file }.

# Added after a Wi-Fi line.
common-no-driver-stick = Пока можно пользоваться USB-адаптером Wi-Fi.

# Tip of the small round i button that opens Help.
common-no-driver-more = Подробнее об этом

common-no-driver-class-wifi = Wi-Fi

common-no-driver-class-bluetooth = Bluetooth

common-no-driver-class-sound = звук

common-no-driver-class-graphics = графика

# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Ещё

common-more-spoken = Подробнее об этом, открывает справку

# First choice of "Ask for the password again after": the phone stays good for as long as it likes.
common-pocketkey-never = Никогда

# A choice of "Ask for the password again after", and the time in the screen's label: { $n } hours.
common-pocketkey-hours =
    { $n ->
        [one] { $n } час
        [few] { $n } часа
        [many] { $n } часов
       *[other] { $n } часов
    }

# A choice of "Ask for the password again after", and the time in the screen's label: { $n } days.
common-pocketkey-days =
    { $n ->
        [one] { $n } день
        [few] { $n } дня
        [many] { $n } дней
       *[other] { $n } дней
    }

# A choice of "Ask for the password again after", and the time in the screen's label: { $n } weeks.
common-pocketkey-weeks =
    { $n ->
        [one] { $n } неделя
        [few] { $n } недели
        [many] { $n } недель
       *[other] { $n } недель
    }

# On the lock and login screens, when the time chosen has passed and the phone is not accepted: { $time } is such as "1 hour".
# SAFETY: needs a human review before it ships
common-pocketkey-due = Прошло больше { $time }, поэтому снова нужно ввести пароль.

# Said under the greyed Earlier versions card.
common-snap-no-snapper = Для прежних версий нужен snapper, а на этом компьютере он не установлен.

# Said under the greyed Earlier versions card.
common-snap-from-snapshot = Недоступно, пока компьютер запущен из снимка.

# Said under the greyed Earlier versions card.
common-snap-part-of-system = На этом компьютере домашняя папка входит в состав системы, а не лежит на отдельной части диска, поэтому прежние версии нельзя хранить отдельно от системы.

# Said under the greyed Earlier versions card.
common-snap-network = Домашняя папка хранится на другом компьютере, поэтому прежние версии нельзя хранить здесь.

# Said under the greyed Earlier versions card: { $fs } is the kind of file system, such as ext4.
common-snap-not-btrfs = Для прежних версий домашняя папка должна быть на Btrfs, а здесь она на { $fs }.

# Said under the greyed Earlier versions card.
common-snap-not-whole = Ваша домашняя папка находится внутри другой части диска, а не на отдельной, поэтому Trindle не может хранить её прежние версии.

# Said under the greyed Earlier versions card.
common-snap-read-only = Сейчас в домашнюю папку нельзя записывать, поэтому ничего сохранить нельзя.

# Said under the greyed Earlier versions card.
common-snap-locked-files = Здесь домашняя папка хранится как один заблокированный файл, а ежедневная копия заполнила бы диск.

# Said under the greyed Earlier versions card.
common-snap-elsewhere = Ваша домашняя папка не находится в /home, где делаются копии.

# Said under the greyed Earlier versions card.
common-snap-apart = Ваша домашняя папка заблокирована или хранится отдельно от остальных, поэтому копии не могут её включить.

# Said under the greyed Earlier versions card.
common-snap-own-part = Ваша домашняя папка лежит на отдельной части диска, а копии /home её не включают.

# Why a plugin's port line in plugin.ini is refused. { $text } is what was written.
common-firewall-not-protocol-port = «{ $text }» написано не в виде протокол:порт, например tcp:24810.

# Why a plugin's port line in plugin.ini is refused. { $text } is what was written before the colon.
common-firewall-bad-protocol = «{ $text }» не tcp и не udp. Пишите строчными буквами.

# Why a plugin's port line in plugin.ini is refused. { $text } is what was written, { $lowest } the lowest port allowed.
common-firewall-bad-port = «{ $text }» не порт от { $lowest } до 65535.

# Said when a port you open by hand isn't written as protocol:port.
common-firewall-own-form = Напишите в виде протокол:порт, например tcp:37666.

# Said when a port you open by hand starts with something other than tcp or udp.
common-firewall-own-protocol = Укажите tcp или udp.

# Said when a port you open by hand isn't a number from 1 to 65535.
common-firewall-own-port = Это не порт от 1 до 65535.

# Joins the last port to the others in a list of ports. { $first } is the others, comma separated.
common-firewall-and = { $first } и { $last }

# Heads a list of ports. { $count } is how many, { $list } is the ports, such as TCP 24810 and UDP 24811.
common-firewall-ports =
    { $count ->
        [one] Порт { $list }
        [few] Порты { $list }
        [many] Порты { $list }
       *[other] Порты { $list }
    }

# Why a plugin's ports are refused. { $max } is the most it may name.
common-firewall-too-many = Плагин может указать не больше { $max } портов.

# Why a plugin's ports are refused. { $port } is a port such as TCP 24810.
common-firewall-twice = { $port } указан дважды.

# Why a plugin's ports are refused. { $port } is a port such as TCP 80, { $lowest } the lowest allowed.
common-firewall-too-low = { $port } ниже { $lowest }. Эти порты занимает система.

# Why a plugin's port is refused. { $port } is the number.
common-firewall-reserved = Порт { $port } занят Trindle или обычной службой, поэтому плагин не может его запросить.

# Why a plugin's port can't be opened. { $port } is such as TCP 24810, { $plugin } is the other plugin's name.
common-firewall-taken = { $port } уже занят плагином { $plugin }, поэтому открыть его ещё и для этого плагина нельзя.

# Why a plugin cannot open another port: { $max } is how many may be open.
common-firewall-full = Для плагинов уже открыто наибольшее число портов: { $max }.

# Told by a notice when the ring has been started again.
common-ring-held-back = Кольцо несколько раз подряд останавливалось, и Trindle на время его приостановил. Сейчас оно снова работает.

# Told by a notice when the ring has been started again.
common-ring-not-started = Кольцо какое-то время не удавалось запустить. Сейчас оно снова работает.

# Told by a notice when the ring has been started again.
common-ring-not-ended = Кольцо перестало отвечать, и его какое-то время не удавалось завершить. Сейчас оно снова работает.

# Told by a notice.
common-ring-not-done = Компьютеру велели перейти в ожидание, перезагрузиться или выключиться, но он этого не сделал.

# Told by a notice; the ring is what the round menu is called.
common-ring-by-name = Рабочий стол не смог сам запустить это кольцо, поэтому оно не открывается поверх полноэкранного окна, а его уведомления не остаются поверх остальных. Возможно, поможет выйти из системы и войти снова.

# A group of apps in the ring.
common-category-internet = Интернет

# A group of apps in the ring (as above).
common-category-office = Офис

# A group of apps in the ring (as above).
common-category-graphics = Графика

# A group of apps in the ring (as above).
common-category-media = Звук и видео

# A group of apps in the ring (as above).
common-category-games = Игры

# A group of apps in the ring (as above).
common-category-development = Разработка

# A group of apps in the ring (as above).
common-category-learning = Обучение

# A group of apps in the ring (as above).
common-category-utilities = Утилиты

# A group of apps in the ring (as above).
common-category-system = Система

# Why an app cannot be started: its launcher names nothing to run.
common-app-no-command = У него нет команды для запуска.

# Why a launcher was left alone: { $why } is the reason.
common-launcher-unchanged = Ничего не изменено, потому что { $why }.

# The ring's notice when something is plugged in.
common-drive-arrived-usb = USB-накопитель подключён

# The ring's notice when something is plugged in (as above).
common-drive-arrived-card = Карта памяти подключена

# The ring's notice when something is plugged in (as above).
common-drive-arrived-disc = Диск вставлен

# The ring's notice when something is plugged in (as above).
common-drive-arrived-other = Накопитель подключён

# Said in the password box after what the password is for.
common-drive-every-time = Пароль будут спрашивать при каждом изменении накопителя. Если закрыть это окно, ничего не изменится.

# Said in the password box: { $rest } is what is being done, such as "format this drive".
common-drive-password-needed-to = Нужен пароль администратора, чтобы { $rest }

# Said in the password box when the action is not named.
common-drive-password-needed = Нужен пароль администратора, чтобы изменить этот накопитель

# Why a folder cannot be shared.
common-share-not-yours = Открыть общий доступ можно только к своим папкам.

# Why a share cannot have the name asked for.
common-share-name-is-login = Это имя чьего-то входа на этом компьютере, поэтому общая папка не может так называться. Выберите другое.

# Why a share cannot have the name asked for.
common-share-bad-name = В имени общей папки не может быть символов % < > * ? | / \ + = ; : " и запятой.

# Why a folder cannot be shared.
common-share-no-guests = Настройки Samba на этом компьютере не разрешают общий доступ без пароля.

# Why a folder cannot be shared.
common-share-most = На этом компьютере уже открыто наибольшее число общих папок.

# Why a folder cannot be shared.
common-share-gone = Этой папки больше нет.

# Why a folder cannot be shared, when the program said nothing.
common-share-no-reason = Samba не назвала причину.

# Why a folder cannot be shared: { $said } is what the sharing program said.
common-share-said = Samba сообщила: { $said }

# Why a screenshot could not be taken.
common-shot-no-desktop = Нет рабочего стола, который можно снять.

# Why a screenshot could not be taken.
common-shot-no-display = Нет экрана, который можно снять.

# Why a screenshot could not be taken.
common-shot-no-display-there = В этой части рабочего стола нет экрана.

# Why a screenshot could not be taken.
common-shot-cannot = Этот рабочий стол нельзя снять.

# Why a screenshot could not be taken.
common-shot-too-big = Снимок экрана слишком большой.

# Why a screenshot could not be taken.
common-shot-no-size = Экран передал пустой снимок, поэтому Trindle не смог сделать снимок экрана.

# Why a screenshot could not be taken.
common-shot-unreadable = Рабочий стол передал снимок, который здесь нельзя прочитать.

# Why a screenshot could not be taken.
common-shot-no-memory = Не хватило памяти для снимка.

# Why a screenshot could not be taken.
common-shot-stopped = Рабочий стол перестал снимать экран.

# Why a screenshot could not be taken.
common-shot-failed = Рабочему столу не удалось снять экран.

# Said where the person is working when no screen reader is installed.
common-reader-none-yet = Программа чтения с экрана не установлена.

# Said wherever hiding a drive is offered or shown.
common-drive-not-a-lock = Скрытие не блокирует и не защищает накопитель. Вместе с ним скрывается и его двойник, который сообщает о себе то же самое.

# Said when typed words are not a change to a key.
common-remap-not-a-key = Такой клавиши Trindle не знает. Попробуйте Escape, Ctrl+C или Left Click.

# Said when typed words are not a change to a key.
common-remap-too-long = Здесь больше клавиш, чем помещается в одно изменение.

# Said when typed words are not a change to a key or a button.
common-no-equals = В изменении нужно указать, что нажимается, знак равенства и во что это превращается.

# Said when typed words are not a change to a key.
common-remap-tapped-held = Скажите, что нажимается коротко, а что удерживается, например: Escape коротко, Ctrl удерживается.

# Said when typed words are not a change to a controller button.
common-pads-not-a-button = Такой кнопки Trindle не знает. Попробуйте South, Left Trigger или Escape.

# Said when typed words are not a change to a controller.
common-pads-not-a-zone = Мёртвая зона бывает от 0% до 50%.

# Said of a line in the controllers file that is not understood.
common-pads-not-a-heading = Trindle не узнаёт этот заголовок контроллера.

# Said of a line in the controllers file that is not understood.
common-pads-no-heading = Она стоит не под заголовком контроллера.

# What a theme's tile says when its file holds more than a theme may.
common-theme-too-big = Этот файл слишком велик для темы, поэтому он не используется. Тема занимает не больше 64 КБ.

# What a theme's tile says when it is not an ordinary file.
common-theme-not-a-sheet = Это не обычный текстовый файл, поэтому он не может быть темой и не используется.

# What is said of a theme that was chosen and whose file has since gone.
common-theme-gone = Её файла больше нет.

# What is said of a theme on a disk that does not answer.
common-theme-too-slow = Её файл читался слишком долго, поэтому Trindle её не использует.

# What is said when the folders of themes are on a disk that does not answer.
common-theme-list-slow = Папки с темами читались слишком долго, поэтому показан только собственный вид Trindle.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-big = Ваша таблица стилей style.css больше 256 КБ, поэтому она не используется.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-not-a-sheet = Ваша таблица стилей style.css не обычный текстовый файл, поэтому она не используется.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-slow = Ваша таблица стилей style.css читалась слишком долго, поэтому она не используется.

# What is said when colours in the person's own style sheet fall short of what can be read.
common-theme-own-hard-to-read = Некоторые цвета трудно читать.

# What is said under the tiles when a folder holds more themes than are shown.
common-theme-not-all = В папке с темами может быть больше тем, чем показано: показываются только первые сорок тем в папке, и просматриваются только её первые 400 файлов.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-no-target = Ваша таблица стилей style.css это ссылка на файл, которого нет, поэтому она не используется.

# The name of a file set aside when damaged, said in a sentence: Clipboard history.
common-damaged-clipboard = История буфера обмена

# The name of a file set aside when damaged, said in a sentence: Last-used apps.
common-damaged-last-used = Недавно использованные приложения

# What kind of virtual machine this is, said in a sentence.
common-vm-qemu = виртуальная машина QEMU (GNOME Boxes, virt-manager и подобные)

# What kind of virtual machine this is, said in a sentence.
common-vm-apple = виртуальная машина Mac

# What kind of virtual machine this is, said in a sentence: { $name } is the name systemd gives it.
common-vm-other = виртуальная машина ({ $name })

# What a program offered in Software sources is for, in a line.
common-source-claude = Приложение Claude от Anthropic: чат, Cowork и Claude Code

# What a program offered in Software sources is for, in a line.
common-source-tailscale = Частная сеть между вашими устройствами

# What a program offered in Software sources is for, in a line.
common-source-syncthing = Держит папки на ваших компьютерах одинаковыми, без облака. Версия в Debian старее.

# What a program offered in Software sources is for, in a line.
common-source-jellyfin = Ваш собственный сервер фильмов, музыки и фото для любого экрана в доме

# What a program offered in Software sources is for, in a line.
common-source-nodejs = Текущая версия Node.js с долгой поддержкой, для разработчиков. В Debian версия 20.

# What a program offered in Software sources is for, in a line.
common-source-github-cli = GitHub из терминала, для разработчиков. Версия в Debian старее.

# What a program offered in Software sources is for, in a line.
common-source-docker = Запускает программы в контейнерах; для разработчиков

# Said in the box that asks for a drive's passphrase: { $name } is the drive.
common-volumes-passphrase-for = Парольная фраза для { $name }

# The title of the box that asks for a drive's passphrase.
common-volumes-unlock-title = Разблокировать накопитель

# What a snapshot is, in the Earlier versions list: made just before software was changed.
common-snap-before-change = Перед изменением программ

# What a snapshot is, in the Earlier versions list: made just after software was changed.
common-snap-after-change = После изменения программ

# What a snapshot is, in the Earlier versions list: made when the computer started.
common-snap-boot = При запуске компьютера

# What a snapshot is, in the Earlier versions list: one taken on a schedule.
common-snap-regular = Регулярная копия

# What a snapshot is, in the Earlier versions list, when it has no note.
common-snap-plain = Снимок

# Said when a font file can't be added. { $name } is the file's name, { $error } the system's reason.
common-pack-not-copied = Trindle не смог скопировать { $name }: { $error }

# Said when a file given as a font isn't one. { $name } is the file's name.
common-pack-not-fonts = { $name } не шрифт и не архив со шрифтами.

# Said when a drive can't be hidden because it has no name of its own to remember it by.
common-hidden-no-name = Trindle не может запомнить этот накопитель, потому что у него нет собственного имени.

# Said when the list of hidden drives can't be read. { $error } is the system's reason.
common-hidden-left = Список скрытых накопителей оставлен как был: { $error }

# Said when a drive can't be released because your login session can't be found.
common-leave-no-session = Trindle не нашёл ваш сеанс входа, поэтому ничего не изменено.

# Said under the sound choices when PipeWire gives no answer and says nothing about why.
common-audio-no-answer = PipeWire не ответил.

# Said when the screen reader can't be turned on or off. { $error } is the first problem, { $undo } the second.
common-screen-reader-undo-failed = { $error } Trindle также не смог отменить изменение: { $undo }

# How long copies are kept, in a sentence such as "kept for a week".
common-keep-week = неделю

# How long copies are kept, in a sentence such as "kept for two weeks".
common-keep-two-weeks = две недели

# How long copies are kept, in a sentence such as "kept for a month".
common-keep-month = месяц

# How long copies are kept, in a sentence such as "kept for 3 days". { $n } is the number of days.
common-keep-days =
    { $n ->
        [one] { $n } день
        [few] { $n } дня
        [many] { $n } дней
       *[other] { $n } дня
    }

# Said when an app can't be started. { $program } is the command, { $error } the system's reason.
common-app-not-started = Trindle не смог запустить { $program }: { $error }

# Said when a settings file is too big to read.
common-keyfile-too-big = Этот файл слишком велик, чтобы его прочитать.

# Said when a file Trindle was about to write at login isn't yours.
common-login-file-not-yours = Файл там не ваш.


# Said when only an app's saved data is to go and the app is running.
# SAFETY: needs a human review before it ships
common-removal-data-running = Данные приложения не удалены, потому что оно запущено. Сначала закройте его.

# Said when the app's data path is a file, not a folder.
# SAFETY: needs a human review before it ships
common-removal-data-not-folder = Данные приложения не удалены, потому что Trindle нашёл файл там, где должна быть его папка.


# A choice of "Ask for the password again after", and the time in the screen's label: { $n } minutes.
common-pocketkey-minutes =
    { $n ->
        [one] { $n } минута
        [few] { $n } минуты
        [many] { $n } минут
       *[other] { $n } минут
    }

# Said where the person is working when one of Steam's own settings files is there but cannot be read, so it is left as it is.
common-steam-unreadable = Не удалось прочитать настройки Steam, поэтому ничего не изменено.

# Said when a permission is asked to be changed for something that is not an app's name.
common-flatpak-not-an-app = Это не название приложения.

# Said when a permission is asked for that Trindle doesn't know.
common-flatpak-not-a-permission = Trindle не может изменить такое разрешение.

# Said where fonts are being added when the file chosen holds far more than a set of fonts is, so none of it was unpacked.
common-fonts-too-big = Этот файл слишком велик для набора шрифтов, поэтому ничего не распаковано.

# Said where fonts are being added when the file chosen could not be unpacked.
common-fonts-unpack-failed = Не удалось распаковать этот файл. Возможно, он повреждён или не того типа, за который себя выдаёт.

# Said where fonts are being added when the program that unpacks files could not run. { $error } is the system's reason.
common-fonts-unpack-error = Не удалось распаковать этот файл: { $error }

# Said where fonts are being added when the file chosen holds no fonts. Help has a section on adding fonts.
common-fonts-none = В этом файле нет шрифтов. См. «Справка», раздел о добавлении шрифтов.

# Said where displays are set when the desktop refused the change and gave no reason.
common-displays-refused = Рабочий стол не смог применить такую настройку экранов.

# Said where displays are set when the desktop refused the change. { $why } is the desktop's own reason.
common-displays-refused-why = Рабочий стол не смог применить такую настройку экранов: { $why }.

# Said where displays are set when the displays changed again and again while the change was being made.
common-displays-kept-changing = Экраны всё время менялись, поэтому ничего не изменено.

# Said where displays are set when Settings is not running in a desktop.
common-displays-no-desktop = Trindle не нашёл рабочий стол.

# Said where displays are set when the desktop has no way to list its displays.
common-displays-cannot-say = Этот рабочий стол не может сообщить, какие есть экраны.

# Said where displays are set when the desktop gave no list of its displays.
common-displays-did-not-say = Рабочий стол не сообщил, какие есть экраны.

# Said where displays are set when the change would switch every display off.
common-displays-none-on = Тогда не останется ни одного включённого экрана.

# Said where displays are set when the desktop gave no answer within a few seconds, or went away.
common-displays-stopped-answering = Рабочий стол перестал отвечать.

# Said where the lock screen PIN is set when what was typed is not the right length. { $n } is how many digits a PIN has.
common-pin-digits = Цифр в PIN-коде: { $n }.

# Said where the lock screen PIN is set when it could not be turned into the secret form it is kept in.
common-pin-not-hashed = Не удалось сохранить PIN-код.

# Said where a theme is chosen when no theme has the name asked for.
common-themes-no-such = Темы с таким названием нет.

# Said where a theme is saved when it could not be kept. { $error } is the system's reason.
common-themes-not-kept = Не удалось сохранить тему: { $error }.

# Said where a theme file is added, when its name doesn't end in .css. { $name } is the file's name.
common-theme-add-not-css = { $name } не тема. Тема это один файл, имя которого заканчивается на .css.

# Said where a theme file is added, when it can't be read. { $name } is the file's name.
common-theme-add-unreadable = Trindle не смог прочитать { $name }.

# Said where a theme file is added, when it is over 64 KB. { $name } is the file's name.
common-theme-add-too-big = { $name } слишком велик для темы. Тема занимает не больше 64 КБ.

# Said where a theme file is added, when it holds more than plain text. { $name } is the file's name.
common-theme-add-not-a-sheet = { $name } не таблица стилей. В нём не только обычный текст.

# Said where a theme file is added, when reading it took too long. { $name } is the file's name.
common-theme-add-too-slow = { $name } читался слишком долго.

# Said where a drive is unlocked when the passphrase did not open it.
common-volumes-wrong-passphrase = Эта парольная фраза не подошла.

# Said where a drive is opened when it is open already.
common-volumes-already-open = Уже открыто.

# Said where a drive is opened when it is locked and no passphrase has been given.
common-volumes-needs-passphrase = Чтобы открыть, нужна парольная фраза.

# Said where a drive is opened when the small window that asks for its passphrase could not be shown.
common-volumes-window-failed = Окно для ввода парольной фразы не открылось.

# Under Trindle's own look on Settings, Appearance: a few words about it.
common-theme-trindle-about = Собственный вид Trindle: глубокая синяя ночь, мягкие края.

# Under the Bold theme on Settings, Appearance.
common-theme-bold-about = Чёрное и белое с чёткими краями, крупные слова и крупные кнопки. Для слабого зрения.

# Under the Midnight theme on Settings, Appearance.
common-theme-midnight-about = Тёмный и плоский, в оттенках серого: чёрный и серый, без рамок, маленькие углы.

# Under the Paper theme on Settings, Appearance.
common-theme-paper-about = Тёплая белая бумага на кремовом столе, для светлых комнат. Светлый вид.

# Under the Snug theme on Settings, Appearance.
common-theme-snug-about = Мельче и теснее, квадратный и холодно-серый. На маленьком экране помещается больше.

# What the security updates switch says on a hover while it is on.
common-auto-updates-on = Автообновления включены

# What the security updates switch says on a hover while it is off.
common-auto-updates-off = Автообновления выключены

# Said when polkit's list of the passwords it is keeping for this login couldn't be read, so Trindle can't make sure a password given for a drive is forgotten.
common-leave-list-unread = Не удалось прочитать, какие пароли ещё хранятся.
