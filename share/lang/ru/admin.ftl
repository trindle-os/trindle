# machine translated (Russian), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Недопустимое имя для входа.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Слишком длинное имя для входа.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = Пользователя «{ $name }» нет.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = «{ $name }» - системная учётная запись, её не трогаем.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = Пароль не передан.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] Пароль должен содержать не менее { $n } символа.
        [few] Пароль должен содержать не менее { $n } символов.
        [many] Пароль должен содержать не менее { $n } символов.
       *[other] Пароль должен содержать не менее { $n } символа.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = Пароль не может содержать перевод строки.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = «{ $name }» уже существует.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = «{ $name }» - единственный администратор, поэтому остаётся им.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Нельзя удалить свою учётную запись.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = «{ $name }» удалён, но его картинка для экрана входа осталась: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Слишком большая картинка.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Эта картинка создана не Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Не удалось открыть папку с картинками для экрана входа: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Это не название языка.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Не удалось прочитать список языков Debian: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Язык «{ $code }» неизвестен Debian.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Имя компьютера состоит из букв, цифр и дефисов, не длиннее 63 знаков, и не начинается и не заканчивается дефисом.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = Ограничение пока включено, но пропадёт после выключения компьютера: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Зарядка пока полная, но ограничение вернётся при следующем запуске: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Другие компьютеры всё ещё могут подключаться, так как переключатель не удалось выключить: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Это не идентификатор плагина.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Этот пользователь неизвестен.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Папка этого пользователя неизвестна.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Не удалось сохранить список разрешённых плагинов: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Плагин не найден.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Ничего не обновлено: не удалось узнать, какая версия Trindle стоит на этом компьютере, поэтому обновление самой Trindle нельзя было исключить.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Установка обновлений

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Наведение порядка

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Установка Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Это не имя пакета.

# Red words when the package to remove is not installed.
admin-package-not-installed = Не сделано: он не установлен.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Это устройство неизвестно fwupd.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Не удалось узнать, кто запросил Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Блокировка не запустилась вовремя.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Блокировка выключена не полностью: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Не удалось вернуть на место { $path }: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Это не название сайта.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Такого сервера нет.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } остановился, не получив всех данных, поэтому это нельзя считать выполненным: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } завершился с ошибкой.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-file-not-removed = Не удалось удалить { $path }: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Ничего не изменено, потому что { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
# SAFETY: needs a human review before it ships
admin-undone-failed = { $why } Вернуть всё, как было, тоже не удалось: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Снимки нельзя хранить так долго или так мало.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Папки со снимками нет.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = На месте папки со снимками лежит не папка.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Не удалось узнать, кто запросил действие.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = Это запускается только через pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Это не имя компьютера.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Это не имя общей папки.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Это не адрес компьютера.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Это не путь к общей папке.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = Здесь можно закрыть только свои общие папки.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = Там не открыта ни одна общая папка.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } не в том виде, в каком её оставила Trindle, поэтому там ничего не подключено.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } не в том виде, в каком её оставила Trindle.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Открыто слишком много общих папок.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } не установлен. Он входит в пакет Debian «{ $package }».

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Не удалось прочитать данные для входа.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Не удалось прочитать, что подключено к этому компьютеру: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Какая-то программа ещё использует эту папку. Сначала закройте её.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Этот компьютер не отвечает. Папка закроется, когда вы выйдете из системы.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Это не номер пользователя.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = Не удалось создать группу sambashare.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Загрузка Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Настройка общего доступа

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = Общий доступ ещё не настроен. Сначала откройте доступ к папке.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = Этот компьютер всё ещё виден другим в сети: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Это не номер снимка.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Создан в настройках

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Не удалось определить, на каком диске стоит система.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = Система находится не в отдельном подтоме Btrfs, поэтому её нельзя заменить снимком.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Не удалось получить ключ подписи с { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = По этому адресу получен не ключ подписи.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Ничего не изменено, потому что файл { $path } не читается: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Получение ключа подписи

# Second line of the progress while a source is added.
admin-step-adding-source = Добавление источника и чтение его списка

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Источник нельзя использовать. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Источник нельзя использовать, поэтому он снова удалён. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = «{ $user }» пока не может включать и выключать его: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = не удалось запустить для «{ $user }»: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = «{ $user }» не удалось разрешить использовать виртуальные машины, которые нужны для Cowork: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = «{ $user }» не удалось разрешить использовать его без пароля: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = «{ $name }» установлен, но его настройка не завершена: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Установка

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Это не один из готовых источников.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = «{ $name }» не предназначен для этого типа компьютера ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Настройка

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Адрес должен начинаться с https:// и не содержать ничего лишнего.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = Набор (suite) и компоненты должны быть простыми словами.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Это не имя пакета.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Удаление

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Повторное чтение списков

# Red words in Apps when the source to remove was not added by Trindle.
# SAFETY: needs a human review before it ships
admin-source-not-ours = Здесь удаляются только источники, добавленные Trindle.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = Источник всё ещё на месте: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = Снимка { $n } нет на диске.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-not-ready = Ничего не изменено: снимок не удалось подготовить. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-aside = Не удалось отложить в сторону текущую систему: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = текущая система теперь называется { $aside }, а не { $sub }, и компьютер не запустится, пока ей снова не вернут имя { $sub }: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-snapshots-across = Не удалось перенести снимки: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-in-place = Не удалось установить снимок на место: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Не удалось обновить список отложенных систем: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = Не удалось удалить: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Эти программы больше никому не нужны, поэтому они удалены: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Установлены не все обновления. Отложены: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = Отключено: { $feature }. Его драйвер ({ $driver }, DKMS) пока не собирается для нового ядра.

admin-driver-off = Драйвер ({ $driver }, DKMS) пока не собирается для нового ядра, поэтому то, что он даёт, отключено.

# Said once after the drivers that are off.
admin-driver-off-rest = Ядро и остальные обновления установлены.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = виртуальная камера

admin-language-in-use = { $code } это язык, на котором запускается этот компьютер, поэтому он остаётся.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = На этом компьютере нет замедления из-за split-lock, которое можно было бы отключить.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Этот накопитель не найден или на нём находится система.

# The drive could not be closed where it was open.
admin-games-drive-busy = Накопитель используется. Закройте всё, что на нём открыто, и попробуйте снова: { $why }

# Another software change has the read-only system open.
admin-system-busy = Сейчас выполняется другое изменение программ. Попробуйте снова, когда оно закончится.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Ничего не изменено: систему не удалось разблокировать: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Систему не удалось заблокировать снова. Она будет заблокирована при следующем запуске: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Не удалось оставить /usr/local доступным для записи: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Это может запустить только root, и не через pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Нужна Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = Снимок установлен на место, и компьютер запустится с него. Убрать старую систему не удалось, это будет повторено при следующем запуске: { $why }

