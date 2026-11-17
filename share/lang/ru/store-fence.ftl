# machine translated (Russian), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Недопустимое имя программы.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Не удалось сохранить.

# Red words when the folder chosen to share does not exist.
# SAFETY: needs a human review before it ships
store-fence-no-folder = Выберите папку, которая существует.

# Red words when the whole home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-whole-home = Домашняя папка целиком остаётся закрытой.

# Red words when a folder that holds the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-holds-home = В ней находится ваша домашняя папка, поэтому она остаётся закрытой.

# Red words when a folder outside the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-outside-home = Можно открыть общий доступ только к папкам внутри вашей домашней папки.

# Red words when a hidden folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-hidden = Скрытые папки всегда остаются закрытыми.

# Red words when a folder that keeps passwords or mail is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-passwords = В ней хранятся пароли или почта, поэтому она остаётся закрытой.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Для этой программы нет папки настроек.

# Red words under the box that names a site AI programs may reach, when the name is not one.
# SAFETY: needs a human review before it ships
store-fence-bad-site = Введите имя сайта, например api.example.com, или *.example.com для всех сайтов с таким именем.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
# SAFETY: needs a human review before it ships
store-fence-bad-computer = Введите имя компьютера, например nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Записывайте каждую в виде NAME=value.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Записывайте каждую в виде NAME=value, заглавными буквами.

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = Программа ИИ просит разрешения использовать эту папку. Сейчас она её не видит.

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = Программа ИИ просит запустить это от имени администратора. Оно запустится вне ограждения, и программа сможет видеть всё, что видите вы.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = Программа ИИ

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
# SAFETY: needs a human review before it ships
store-fence-read-only = Разрешить только чтение этой папки

# Note under the question about a folder.
store-fence-sees-next = Она увидит папку при следующем запуске программы.

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = Нет

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = Открыть доступ только на этот сеанс

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = Всегда открывать доступ

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = Не разрешать

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = Разрешить

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Эта команда слишком длинная, чтобы показать её целиком, поэтому она не запущена.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Другой запрос уже ждёт вашего ответа.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = Вы не разрешили это.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } недоступна внутри ограждения.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = Её не удалось запустить вне ограждения.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Укажите полный путь к папке.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = Вы не открыли к ней доступ.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Доступ открыт. Папка будет на месте при следующем запуске этой программы.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Укажите, к какому компьютеру подключиться.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = Программы ИИ в ограждении не могут подключиться к этому компьютеру по ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = Не удалось создать ключ для программ ИИ: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Укажите команду для запуска.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } здесь недоступна.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } недоступна внутри ограждения. Попросите запустить её снаружи.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: канала запросов ограждения нет.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: запрос не отправлен.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock отказал: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = Ограждение не запустилось: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = Ограждение не запустилось: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = Программы ИИ хранят здесь свои данные входа отдельно от ваших. Войдите внутри программы, когда она попросит.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Ваши папки закрыты от программ ИИ. Чтобы работать в папке, откройте к ней доступ в «Настройках», в разделе «Программы ИИ».

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = Ограждение не запустилось: не удалось создать фильтр системных вызовов.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = Ограждение не запустилось: не удалось настроить ограничение сети.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = Ограждение не запустилось: bubblewrap не запустился: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: не найдена вне ограждения
