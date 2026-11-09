# machine translated (Russian), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Диски

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Об этой странице

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Все диски, установленные в этот компьютер или подключённые к нему, нарисованы полосой из разделов. Ширина каждого раздела соответствует его размеру, а заполнен он настолько, насколько занят, поэтому полный раздел сразу заметен.

# Second paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-2 = Под рисунком у каждого диска есть карточка. Откройте раздел, смонтируйте или размонтируйте его, безопасно извлеките диск или выключите его питание. Карандаш на разделе открывает его изменения: имя, собственная папка и готовность при запуске, размер, проверка и исправление, форматирование или удаление. Плюс в свободном месте создаёт новый раздел.

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = Диск, с которого работает система, показан, но для него никогда не предлагаются изменения, которые стирают или перемещают данные. Для любого изменения любого диска каждый раз запрашивается пароль администратора. Глаз скрывает диск или раздел от «Папок» и кольца: он остаётся здесь, затенённый, и не заблокирован.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Каждый раздел каждого диска, заполненный настолько, насколько занят.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Ваши диски

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Сейчас не удаётся прочитать диски.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Повторить

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Посмотреть снова

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Дисков не найдено. Подключите диск, и он появится здесь.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = Служба дисков

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = Системная шина

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what }: недоступно. Установите командой: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Смотрим на диски…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } выходит из строя. Скопируйте с него всё нужное прямо сейчас.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } требует внимания: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Диски и место на них.
        [one] Диски и место на них, а также подключённый диск.
        [few] Диски и место на них, а также подключённые диски: { $plugged }.
        [many] Диски и место на них, а также подключённые диски: { $plugged }.
       *[other] Диски и место на них, а также подключённые диски: { $plugged }.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Почти заполнен: на этом компьютере свободно всего { $free }. Скоро удалите или перенесите часть файлов.
        [one] Почти заполнен: на этом компьютере свободно всего { $free }, и подключён диск. Скоро удалите или перенесите часть файлов.
        [few] Почти заполнен: на этом компьютере свободно всего { $free }, подключённых дисков: { $plugged }. Скоро удалите или перенесите часть файлов.
        [many] Почти заполнен: на этом компьютере свободно всего { $free }, подключённых дисков: { $plugged }. Скоро удалите или перенесите часть файлов.
       *[other] Почти заполнен: на этом компьютере свободно всего { $free }, подключённых дисков: { $plugged }. Скоро удалите или перенесите часть файлов.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Заполняется: на этом компьютере свободно { $free }.
        [one] Заполняется: на этом компьютере свободно { $free }, и подключён диск.
        [few] Заполняется: на этом компьютере свободно { $free }, подключённых дисков: { $plugged }.
        [many] Заполняется: на этом компьютере свободно { $free }, подключённых дисков: { $plugged }.
       *[other] Заполняется: на этом компьютере свободно { $free }, подключённых дисков: { $plugged }.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Места достаточно: на этом компьютере свободно { $free }.
        [one] Места достаточно: на этом компьютере свободно { $free }, и подключён диск.
        [few] Места достаточно: на этом компьютере свободно { $free }, подключённых дисков: { $plugged }.
        [many] Места достаточно: на этом компьютере свободно { $free }, подключённых дисков: { $plugged }.
       *[other] Места достаточно: на этом компьютере свободно { $free }, подключённых дисков: { $plugged }.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Этот компьютер

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Ваши файлы

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] Ещё свободно { $free } там, где находится система.
       *[files] Ещё свободно { $free } для ваших файлов.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: заполнено { $full }, свободно { $free } из { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = из { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Показаний нет, диски не сообщают о своём состоянии

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Исправен, как сообщает сам диск

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [few] Исправны, как сообщают все { $n } диска
        [many] Исправны, как сообщают все { $n } дисков
        [one] Исправны, как сообщают все { $n } дисков
       *[other] Исправны, как сообщают все { $n } дисков
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } требует внимания, см. его карточку
        [few] { $n } требуют внимания, см. их карточки
        [many] { $n } требуют внимания, см. их карточки
       *[other] { $n } требуют внимания, см. их карточки
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] { $n } диск, всего { $total }. { $health }.
        [few] { $n } диска, всего { $total }. { $health }.
        [many] { $n } дисков, всего { $total }. { $health }.
       *[other] { $n } диска, всего { $total }. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = свободно { $size }

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Не смонтирован

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Свободное место

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = Нет диска или карты

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Пока пусто

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB или съёмный

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, внутри этого компьютера

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB или съёмный, скрыт

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, внутри этого компьютера, скрыт

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, заполнено { $full }, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Раздел

# Name given to a drive that gives no maker or model.
disks-drive-name = Диск

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Подкачка

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Зашифрован, разблокирован

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Зашифрован, заблокирован

# Kind of a part that holds other parts.
disks-kind-container = Содержит другие разделы

# Kind of a part that has no filesystem on it.
disks-kind-none = Нет файловой системы

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, раздел, с которого запускается компьютер
        [legacy] { $kind }, раздел, с которого запускается старый компьютер
       *[loader] { $kind }, собственный раздел загрузчика
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = любой компьютер

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = только Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = старые устройства

# Trouble a drive reports about its spare space.
disks-trouble-spare = заканчивается резервное место

# Trouble a drive reports about its heat.
disks-trouble-temperature = слишком горячий

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = изнашивается

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = перешёл в режим только для чтения

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = отказала резервная память

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = часть его перешла в режим только для чтения

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = изношен сверх ожидаемого производителем

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } повреждённое место
        [few] { $n } повреждённых места
        [many] { $n } повреждённых мест
       *[other] { $n } повреждённого места
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] в работе { $hours } час
        [few] в работе { $hours } часа
        [many] в работе { $hours } часов
       *[other] в работе { $hours } часа
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Выходит из строя: скопируйте с него всё нужное прямо сейчас

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Требует внимания: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Исправен

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Диск не отвечает

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = Корпус отвечает, а диск внутри него нет. Отключите его, подождите десять секунд и подключите в синий порт или порт USB-C его собственным кабелем; если ничего не изменится, диск внутри мог отсоединиться или выйти из строя.

# Tip of the round button on a drive's card that takes the drive out safely.
# SAFETY: needs a human review before it ships
disks-remove-safely = Безопасно извлечь

# Tip of the round button on a drive's card that powers the drive off.
# SAFETY: needs a human review before it ships
disks-power-off-tip = Выключить питание

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Стереть весь диск

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Показать этот диск
       *[part] Показать этот раздел
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Скрыть этот диск. { $lock }
       *[part] Скрыть этот раздел. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Скрыт от «Папок» и кольца. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = С него работает система: здесь ничего нельзя стереть.

# Note on the card of a drive that has no partition table.
disks-no-table = Нет таблицы разделов: весь диск свободен.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = не смонтирован

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = монтируется при запуске в { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = появляется в { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = в { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Открыть

# Tip of the round button that mounts a part.
disks-mount = Смонтировать

# Tip of the round button that unmounts a part.
disks-unmount = Размонтировать

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Изменить

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Имя

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Имя (необязательно)

# Tip of the button that saves a part's new name.
disks-rename-tip = Переименовать

# Note under the Name row of a part.
disks-name-note = Как он называется здесь и на других компьютерах.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Имя папки

# Tip of the button that saves a part's folder.
disks-use-folder = Использовать эту папку

# Label of the row where a part's folder is chosen.
disks-folder = Папка

# Note under the Folder row of a part.
disks-folder-note = Где появляются его файлы. Оставьте пустым, чтобы компьютер выбрал сам.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Монтировать при запуске

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Готов при каждом запуске компьютера. Если его нет, компьютер всё равно запустится.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Размер в ГБ

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = От { $least } до { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = От { $least } до { $most }. Смонтируйте его, чтобы увидеть, насколько его можно уменьшить.

# Tip of the button that resizes a part.
disks-resize-tip = Изменить размер

# Label of the row where a part is resized.
disks-size = Размер

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = Это сотрёт всё на { $words }. Отменить это нельзя.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Нажмите ещё раз: { $name } на { $drive } изменится с { $from } до { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Освободить место для нового раздела

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Место в ГБ

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Освободить место

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Нажмите ещё раз: { $name } на { $drive } уменьшится на { $size } после создания снимка

# Note under the row that makes room.
disks-room-note = Уменьшается во время работы, после создания снимка, с освобождением места.

# Tip of the button that checks a part for faults.
disks-check-tip = Проверить на ошибки

# Tip of the button that repairs a part.
disks-repair-tip = Исправить

# Label of the row where a part is checked and repaired.
disks-faults = Ошибки

# Note under the Faults row.
disks-faults-note = Ищет ошибки в том, как хранятся его файлы, и исправляет их. На время работы он размонтируется.

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = Нажмите ещё раз, чтобы исправить { $name } на { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Тип

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = Форматирование

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = Форматировать

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = Начинает заново как один раздел, занимающий весь диск.

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = Стирает всё только на этом разделе.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Нажмите ещё раз: всё на { $words } будет стёрто

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, на { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = Удалить { $name }, { $size }, на { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Удалить этот раздел

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = Его место становится свободным, готовым для нового раздела или для роста предыдущего.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Создать здесь новый раздел

# Label of the row where a new part is made.
disks-new-part = Новый раздел

# Note under the row where a new part is made.
disks-new-part-note = Создаётся в этом свободном месте и форматируется для использования.

# Tip of the button that makes a new part.
disks-make-tip = Создать

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = Нажмите ещё раз, чтобы создать раздел { $size } на { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = Стереть { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, на { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Новый раздел

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = свободно { $size }, на { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Закрыть

# Words on a row while a part is being mounted.
disks-mounting = Монтирование

# Words on a row when a part has been mounted.
disks-mounted = Смонтирован

# Words on a row while a part is being unmounted.
disks-unmounting = Размонтирование

# Words on a row when a part has been unmounted.
disks-unmounted = Размонтирован

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Закрываем

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Можно безопасно извлекать

# Words on a drive's row while it is being powered off.
disks-powering-off = Выключаем питание

# Words on a drive's row when it has been powered off.
disks-off-safe = Выключен: можно отключать

# Words on a row while a part is being renamed.
disks-renaming = Переименование

# Words on a row when a part has been renamed.
disks-renamed = Переименован

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = Компьютер снова сам выбирает папку

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Появляется в { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Монтируется при запуске в { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = При запуске не трогается

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Сохранение

# Words on a row while a part is being resized.
disks-resizing = Изменение размера

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Размер изменён до { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Не удалось найти файлы, поэтому освободить место нельзя.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Создаём снимок

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Потом не удалось найти снимок.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Размер не изменялся.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Пока создавался снимок, всё изменилось, поэтому размер не изменялся.

# Words on a row while room is being made.
disks-making-room = Освобождаем место

# Words on a row when room has been made. { $size } is how much.
disks-room-made = Освобождено места: { $size }

# Words on a row while a part is being repaired.
disks-repairing = Исправление

# Words on a row when a part has been repaired.
disks-repaired = Исправлен

# Red words when some faults could not be put right.
disks-repair-failed = Некоторые ошибки исправить не удалось. Скопируйте всё, что хотите сохранить.

# Words on a row while a part is being checked.
disks-checking = Проверка

# Words on a row when the check found no faults.
disks-no-faults = Ошибок не найдено

# Red words when the check found faults.
disks-faults-found = Найдены ошибки: нажмите гаечный ключ, чтобы исправить их.

# Words on a row while a part or drive is being formatted.
# SAFETY: needs a human review before it ships
disks-formatting = Форматирование

# Words on a row when a part has been formatted.
# SAFETY: needs a human review before it ships
disks-formatted = Отформатирован

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
# SAFETY: needs a human review before it ships
disks-erasing-whole = стирание всего диска

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = Не удалось найти новый раздел.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Не удалось связаться со службой дисков.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Не удалось прочитать идентификатор нового раздела.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = Удаление

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = Удалён

# Words on a row while a new part is being made.
disks-making-it = Создание

# Words on a row when a new part has been made.
disks-made = Создан

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = Не удалось сохранить: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Сначала укажите имя.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] Слишком длинное имя: не больше { $max } символа.
        [few] Слишком длинное имя: не больше { $max } символов.
        [many] Слишком длинное имя: не больше { $max } символов.
       *[other] Слишком длинное имя: не больше { $max } символов.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Используйте только буквы, цифры, - и _ (без пробелов).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Начинайте с буквы или цифры.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } уже используется.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] Слишком длинное имя для этого типа: не больше { $max } символа.
        [few] Слишком длинное имя для этого типа: не больше { $max } символов.
        [many] Слишком длинное имя для этого типа: не больше { $max } символов.
       *[other] Слишком длинное имя для этого типа: не больше { $max } символов.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Имена FAT не могут содержать * ? . , ; : / \ | + = < > [ ] или ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Сначала смонтируйте его: этот тип можно увеличить только в смонтированном виде.
       *[smaller] Сначала смонтируйте его: этот тип можно уменьшить только в смонтированном виде.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Этот тип нельзя увеличить.
       *[smaller] Этот тип нельзя уменьшить.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Пароль не введён, поэтому ничего не изменилось.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Пароль не введён, поэтому работа остановилась на середине.

# Red words when another change is pressed while one is still being made.
disks-change-going = Изменение уже выполняется.

disks-one-at-a-time = Другое изменение ещё выполняется. Повторите, когда оно закончится.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = Не удалось убедиться, что будет запрошен пароль, поэтому ничего не изменилось.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = Только что введённый пароль не удалось отозвать, поэтому в течение нескольких минут другое изменение диска может не запросить его. Попытка повторяется.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = Введённый пароль теперь отозван: следующее изменение запросит его.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Не удалось связаться со службой дисков, поэтому ничего не изменилось.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = «Диски» были закрыты, пока выполнялось это изменение.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = «Диски» остаются открытыми, пока это не закончится, чтобы сообщить, чем всё кончилось.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, поэтому ничего не изменилось.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, поэтому работа остановилась на середине.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Не удалось обратиться к службе дисков

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } Работа остановилась на середине.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Выполнено: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Не удалось: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Не выполнено: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Неизвестно: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } был размонтирован для этого и сейчас не смонтирован.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } был размонтирован для этого и по-прежнему не смонтирован: смонтируйте его в его строке.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } мог быть размонтирован: посмотрите его строку.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev } был размонтирован для этого, а для повторного монтирования нужен отдельный пароль: смонтируйте его в его строке.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } был размонтирован для этого и не смонтировался снова ({ $why }): смонтируйте его в его строке.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Отформатирован, но папка { $dir } ему снова не назначена. { $why } Выберите папку снова в его строке.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Что-то ещё использует его. Закройте окна и программы, показывающие его файлы, и повторите.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Для того, что на нём есть, не хватит места при таком размере.

# Step of a change. { $dev } is the part.
disks-step-unmount = размонтирование { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = повторное монтирование { $dev }

# Step of a change.
disks-step-eject = закрытие диска для извлечения

# Step of a change.
disks-step-power-off = выключение питания диска

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = форматирование { $dev }

# Step of a change.
disks-step-create = создание нового раздела

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = удаление { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = изменение размера раздела { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = изменение размера того, что на { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = исправление { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = проверка { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = переименование { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = сохранение выбора папки и запуска для { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = шаг { $method } для { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Этот раздел { $kind } можно уменьшить, только когда он не используется, поэтому освободить место здесь нельзя.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Этот раздел xfs нельзя уменьшить.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Этот раздел { $kind } нельзя уменьшить, пока он используется.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Снимок можно создать только для собственного диска системы, поэтому освободить место здесь нельзя.

# Red words under the Make room row.
disks-room-not-in-table = Это не раздел из таблицы диска, поэтому освободить место из него нельзя.

# Red words under the Make room row.
disks-room-no-online-shrink = Служба дисков не может уменьшить Btrfs, пока он здесь используется, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-holds-parts = Он содержит другие разделы, поэтому освободить место из него нельзя.

# Red words under the Make room row.
disks-room-not-gpt = У его диска нет таблицы GPT, поэтому освободить место из него нельзя.

# Red words under the Make room row.
disks-room-mount-first = Сначала смонтируйте его: Btrfs уменьшается в смонтированном виде.

# Red words under the Make room row.
disks-room-mounts-not-one = Он смонтирован не ровно в одном месте, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-read-only = Он смонтирован только для чтения, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-read-only-unknown = Не удалось определить, только ли для чтения он смонтирован, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-many-drives = Файлы распределены по нескольким дискам, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-drives-unknown = Не удалось определить, по скольким дискам распределены файлы, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-swap-file = На нём лежит файл подкачки, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-swap-unknown = Не удалось определить, лежит ли на нём файл подкачки, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-used-unknown = Не удалось прочитать, сколько из него используется, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-area-unknown = Служба дисков не сообщает размер области файлов, поэтому проверить не удалось и освободить место нельзя.

# Red words under the Make room row.
disks-room-open-missing = Не удалось найти его открытую копию, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-encryption-sizes = Размеры, указанные для шифрования, не совпадают, поэтому освободить место нельзя.

# Red words under the Make room row.
disks-room-sizes = Указанные для него размеры не совпадают, поэтому освободить место нельзя.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Он содержит { $used }, и нужно сохранить больше, поэтому освободить можно слишком мало.

# Red words under the Make room row.
disks-room-sizes-unread = Не удалось прочитать его размеры.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Можно освободить место от { $least } до { $most }.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, поэтому работа остановлена.

# What was found wrong while making room.
disks-room-check-unmounted = Файлы больше не смонтированы

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = Раздел имеет размер { $is }, а должен { $should }

# What was found wrong while making room.
disks-room-check-open = Открытая копия шифрования не того размера, какой должен быть

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = Область файлов имеет размер { $is }, а должна { $should }

# What was found wrong while making room.
disks-room-check-area-small = Область файлов меньше, чем используется

# What was found wrong while making room.
disks-room-check-fit = То, что содержит, больше не помещается

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] Служба дисков больше не показывает раздел, поэтому работа остановлена.
        [copy] Служба дисков больше не показывает открытую копию, поэтому работа остановлена.
       *[files] Служба дисков больше не показывает файловую систему, поэтому работа остановлена.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = уменьшение области файлов

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = уменьшение открытой копии шифрования

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = уменьшение раздела

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = проверка размеров

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Перед освобождением места под { $dev } ({ $at })

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Нажмите ещё раз: раздел { $name } на { $drive } уменьшится с { $from } до { $to }. Сначала сделайте резервную копию его файлов: при отключении питания во время уменьшения их можно потерять.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows на { $name } была переведена в режим гибернации или оставлена в режиме быстрого запуска, поэтому при уменьшении раздела можно потерять её файлы. Запустите Windows, выберите «Перезагрузка» и повторите попытку.

