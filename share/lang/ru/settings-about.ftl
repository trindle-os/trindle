# machine translated (Russian), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Об этом компьютере

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = имя узла имя компьютера имя в сети о системе версия debian trindle память оперативная озу процессор cpu диск графика видеокарта экран модель ядро kernel активность запущенные программы занято диспетчер задач системный монитор диски накопители usb место

# Text behind the info button of the About page.
settings-about-help = На этой странице показано имя этого компьютера в сети, из чего он состоит и сколько времени занимает запуск. «Активность» показывает, что компьютер делает прямо сейчас и какие программы используют память и процессор. «Диски» показывает ваши диски и место на них. Каждый из этих разделов открывается в отдельном окне. Откройте их стрелками в разделе «Подробнее».

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Неизвестно

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } поток
        [few] { $model }, { $cores } потока
        [many] { $model }, { $cores } потоков
       *[other] { $model }, { $cores } потока
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Графика Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = Графика AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = Графика NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Графика виртуальной машины

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Графика

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }, { $hz } Гц

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = Занято: { $amount }

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = Свободно: { $amount }

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = из { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, включён уже { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, включён уже { $on }. Диск почти заполнен.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } с

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } мин

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } минуту
        [few] { $n } минуты
        [many] { $n } минут
       *[other] { $n } минуты
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } час
        [few] { $n } часа
        [many] { $n } часов
       *[other] { $n } часа
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } день
        [few] { $n } дня
        [many] { $n } дней
       *[other] { $n } дня
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Используйте только буквы, цифры и дефисы. Имя не должно начинаться или заканчиваться дефисом.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Ожидание вашего пароля…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Не удалось переименовать компьютер.

# Words under the computer name box after renaming worked.
settings-about-name-done = Уже открытые программы будут использовать новое имя после следующего входа.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Имя компьютера

# Tip of the save button beside the computer name box.
settings-about-rename = Переименовать этот компьютер

# Bold line of the computer name setting.
settings-about-name = Имя компьютера

# Heading of the card with the computer's name.
settings-about-name-group = Имя

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Это имя компьютера в сети и в приглашении Терминала. Используйте буквы, цифры и дефисы, но имя не должно начинаться или заканчиваться дефисом. Чтобы изменить имя, нужно ввести пароль.

# Heading of the card about the computer's hardware.
settings-about-hardware = Компьютер

# Text behind the info button of the hardware card.
settings-about-hardware-about = Здесь показаны производитель и модель компьютера, его процессор и графика. Также показано, сколько потоков процессор может выполнять одновременно, и каждый используемый экран с его размером и частотой обновления.

# Heading of the card about the software.
settings-about-software = Программы

# Text behind the info button of the software card.
settings-about-software-about = Trindle - это рабочий стол, который работает поверх Debian. Debian управляет программами под ним. Ядро - часть, которая работает с оборудованием. Собственный композитор Trindle рисует рабочий стол с помощью Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Собственный композитор Trindle

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } приложение
        [few] { $n } приложения
        [many] { $n } приложений
       *[other] { $n } приложения
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Подробнее

# Text behind the info button of the More detail card.
settings-about-more-about = Что компьютер делает прямо сейчас и какие программы используют память и процессор, показывает «Активность». Все диски, их заполненность и состояние, а также USB-накопители показывают «Диски».

# Bold line of the setting that opens Activity.
settings-about-activity = Что сейчас работает

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Активность

# Bold line of the setting that opens Drives.
settings-about-drives = Диски и место

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Диски

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Не удалось сохранить время запуска в Загрузки.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Дата и время,Секунды

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Время запуска Trindle.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = В Загрузках, файл { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Сохранить

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Сохранить время каждого запуска в Загрузки

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Сохранять время каждого запуска

# Quiet line under that setting.
settings-about-times-keep-sub = Сохраняется в Загрузки как таблица.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Каждый столбец - один запуск, самый новый справа.

# Heading of the card with the last ten start-up times.
settings-about-times = Последние десять запусков

# Text behind the info button of the start-up times card.
settings-about-times-about = Каждый запуск измеряется от нажатия кнопки питания до экрана входа, без времени, потраченного на ввод пароля диска. Самый новый вверху.

# Heading of the memory tile.
settings-about-memory = Память

# Heading of the disk tile.
settings-about-disk = Диск

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Отчёты о проблемах

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Отчёты уходят в Buildmorphic, сообщество, которое помогает создавать Trindle. Присоединиться можно с одним лишь адресом электронной почты. За проблемы, о которых вы сообщили первым, вы получаете баллы и уровни, даже если не можете их исправить. В отчёте только заранее заданный набор сведений, и без нажатия «Отправить» ничего не уходит. Имя пользователя или имя на GitHub указывать необязательно, а именем пользователя никогда не бывает ваша почта.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Предлагать отправить отчёт, когда что-то пошло не так

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Имя пользователя в Buildmorphic

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = Имя на GitHub

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = О сообществе

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] Выключено. Удалён { $n } ожидавший отчёт.
        [few] Выключено. Удалены { $n } ожидавших отчёта.
        [many] Выключено. Удалено { $n } ожидавших отчётов.
       *[other] Выключено. Удалено { $n } ожидавших отчёта.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] Выключено. Не удалось удалить { $n } ожидающий отчёт.
        [few] Выключено. Не удалось удалить { $n } ожидающих отчёта.
        [many] Выключено. Не удалось удалить { $n } ожидающих отчётов.
       *[other] Выключено. Не удалось удалить { $n } ожидающих отчёта.
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = Обновления

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle проверяет новые версии приложений, плагинов, прошивок и самой себя вскоре после входа, а затем каждые несколько часов. Если это выключить, Trindle проверяет только когда вы нажимаете «Проверить обновления» в «Приложениях» или кнопку обновлений на кольце. Исправления безопасности Debian ставятся каждый день, и у них свой переключатель. Если выключить его, они больше не ставятся сами, и известные уязвимости остаются, пока вы не обновитесь. Для изменения любого из переключателей нужен пароль администратора, чтобы выбор действовал на весь компьютер.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Проверять обновления

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Ожидание вашего пароля…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Обновления безопасности

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Исправления безопасности больше не ставятся сами.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Выключено, но ожидающие отчёты дополнения Reporting не были удалены.
