# machine translated (Russian), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Установка пакета

# Words at the top of the window while the package is read.
store-deb-reading = Чтение пакета…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = Этот файл не является пакетом Debian, который можно установить. Возможно, он не догрузился.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } готов к установке в отдельную папку.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Этот пакет предназначен для компьютеров { $arch }, поэтому на этот его установить нельзя.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Это сам Trindle. Обновляйте его на странице «Обновления» в разделе «Приложения», а не из файла.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } является драйвером или фоновой службой, которые не работают из отдельной папки, поэтому установить его здесь нельзя.

# SAFETY: needs a human review before it ships
# Words on the caution card above every package.
store-deb-caution = Пакет не из Debian и не из Trindle устанавливается в отдельную папку, где он ограничен, но не изолирован полностью. Устанавливайте только пакеты от разработчиков, которым доверяете.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } установлен в отдельную папку.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Проверка источника…

# Heading in the card that says where the file came from.
store-deb-where = Откуда он взялся

# SAFETY: needs a human review before it ships
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = Удалить его и его данные

# Word beside the ring while the package goes in.
store-deb-installing = Установка

# Main button that installs the package.
store-deb-install = Установить

# Main button that installs the same version again.
store-deb-reinstall = Переустановить

# Main button that brings the installed package up to this version.
store-deb-update = Обновить

# SAFETY: needs a human review before it ships
# Heading of the card for a package not installed yet.
store-deb-goes-in = Устанавливается в собственную папку с ограничениями

# SAFETY: needs a human review before it ships
# Under that heading.
store-deb-goes-in-note = Пароль не нужен. Пока вы не разрешите, у него нет доступа к интернету, звуку и вашим файлам.

# Heading of the card for a package installed at this version.
store-deb-same-version = Версия { $version } установлена в отдельную папку

# Heading of the card for a package installed at another version.
store-deb-other-version = Версия { $version } установлена в отдельную папку

# Under that heading: the version this brings.
store-deb-other-version-note = Это обновит её до версии { $version }, а сохранённые данные останутся.

# Facts line: how big the package is once installed.
store-deb-fact-size = Размер после установки

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Предназначен для

# Text of the About card when the package has no description.
store-deb-no-description = У этого пакета нет описания.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Установка в отдельную папку. Загрузка нужных компонентов может занять несколько минут.

# Words at the top of the window once the package is installed.
store-deb-done = Установлено в отдельную папку. Вы найдёте его в кольце.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Установлено в отдельную папку, но этих компонентов не хватает, поэтому программа может не запуститься: { $parts }.

# SAFETY: needs a human review before it ships
# Words at the top of the window once the package is removed.
store-deb-removed = Удалено из своей папки вместе со всем, что оно сохранило.

# Red words when a file is not a Debian package.
store-boxed-not-deb = Этот файл не является пакетом Debian, который можно установить.

# Step shown while a package is unpacked.
store-boxed-unpacking = Распаковка { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = Добавление { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } не является пакетом Debian, который можно установить.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = Не удалось распаковать { $path }.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Эта программа не работает в отдельной папке.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Терминал

# Title of the notice when older programs are off.
store-boxed-x-off = Старые программы выключены

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Настройки, Разрешения

# Title of the notice when too many older programs are open.
store-boxed-x-many = Слишком много старых программ

# Body of that notice.
store-boxed-x-many-close = Закройте одну и повторите попытку

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = Не удалось открыть { $app }

# SAFETY: needs a human review before it ships
# Body of the notice the first time a terminal opens outside its box.
store-boxed-terminal-told = Терминал имеет доступ к большей части вашего компьютера, чем другие приложения. Вводите только понятные вам команды и только от людей, которым доверяете.

# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = Нет локальной сети

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = У { $app } нет сети: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = отсутствует программа Trindle

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare не установлена

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta не установлена

# The same: the network room was not made.
store-boxed-home-why-not-made = её сеть не была настроена

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = её сеть уже существовала

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle не смог настроить защитный фильтр

# The same: the room's rules would not load.
store-boxed-home-why-rules = правила её сети не загрузились

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } не запустилась: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta сообщила: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta не смогла её подключить

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = Верхняя папка { $name } это ссылка, поэтому пакет использовать нельзя.

# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } это драйвер или фоновая служба. Такие программы не работают из собственной папки, поэтому он не установлен.


# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Открыто без интернета

# Body of that notice: why, and what the program was given.
store-downloaded-why = Поскольку файл был скачан, программа может только читать этот один файл.

# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = Старые программы не могут открыться

# Body of that notice: what to do.
store-boxed-x-unasked-why = Выйдите из системы и войдите снова, затем повторите попытку
