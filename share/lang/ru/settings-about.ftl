# machine translated (Russian), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Об этом компьютере

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = имя узла имя компьютера имя в сети о системе версия debian trindle память оперативная озу процессор cpu диск графика видеокарта экран модель ядро kernel активность запущенные программы занято диспетчер задач системный монитор диски накопители usb место

# Text behind the info button of the About page.
settings-about-help = Имя этого компьютера в сети, из чего он состоит и сколько времени занимает запуск. Что он делает прямо сейчас и какие программы используют память и процессор, показывает «Активность», а диски и место на них показывают «Диски». Это отдельные окна: они открываются стрелками в разделе «Подробнее».

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
settings-about-name-bad = Только буквы, цифры и дефисы; имя не должно начинаться или заканчиваться дефисом.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Ожидание пароля...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Не удалось переименовать компьютер.

# Words under the computer name box after renaming worked.
settings-about-name-done = Уже открытые программы увидят новое имя после следующего входа.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Имя компьютера

# Tip of the save button beside the computer name box.
settings-about-rename = Переименовать этот компьютер

# Bold line of the computer name setting.
settings-about-name = Имя компьютера

# Quiet line under the computer name setting.
settings-about-name-sub = Буквы, цифры и дефисы.

# Heading of the card with the computer's name.
settings-about-name-group = Имя

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Как этот компьютер отображается в сети и в приглашении Терминала. Буквы, цифры и дефисы; имя не должно начинаться или заканчиваться дефисом. Для смены имени потребуется ваш пароль.

# Heading of the card about the computer's hardware.
settings-about-hardware = Компьютер

# Text behind the info button of the hardware card.
settings-about-hardware-about = Производитель и модель, процессор и число потоков, которые он выполняет одновременно, графика и каждый используемый экран с его размером и частотой обновления.

# Quiet line under the maker and model.
settings-about-hw-computer = Компьютер

# Quiet line under the processor.
settings-about-hw-processor = Процессор

# Quiet line under the graphics.
settings-about-hw-graphics = Рисует изображение

# Quiet line under the screens.
settings-about-hw-screen = Экран

# Heading of the card about the software.
settings-about-software = Программы

# Text behind the info button of the software card.
settings-about-software-about = Trindle - это рабочий стол поверх Debian, который управляет программами под ним. Ядро отвечает за связь с оборудованием, а собственный композитор Trindle рисует рабочий стол в Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Работает в основе

# Quiet line under the kernel version.
settings-about-sw-kernel = Ядро

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Собственный композитор Trindle

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Рисует рабочий стол в Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } приложение
        [few] { $n } приложения
        [many] { $n } приложений
       *[other] { $n } приложения
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Установлены, готовы в кольце

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Подробнее

# Text behind the info button of the More detail card.
settings-about-more-about = Что компьютер делает прямо сейчас и какие программы используют память и процессор, показывает «Активность». Все диски, их заполненность и состояние, а также USB-накопители показывают «Диски».

# Bold line of the setting that opens Activity.
settings-about-activity = Что сейчас работает

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Программы, использующие память и процессор.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Активность

# Bold line of the setting that opens Drives.
settings-about-drives = Диски и место

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Все диски, их заполненность и состояние.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Диски

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Не удалось сохранить время запуска в Загрузки.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = В Загрузках, файл { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Сохранить

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Сохранить время каждого запуска в Загрузки

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Каждый запуск, на память

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


# Heading of the card with the switch for looking for updates.
settings-about-updates = Обновления

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle ищет новые версии приложений, плагинов, прошивок и самой себя вскоре после входа и каждые несколько часов, а исправления безопасности Debian ставятся каждый день. Когда это выключено, Trindle ищет только когда вы нажимаете «Проверить обновления» в «Приложениях» или кнопку обновлений на кольце. У исправлений безопасности Debian свой переключатель: когда он выключен, они больше не ставятся сами, и известные ошибки остаются, пока вы не обновитесь. Изменение любого переключателя требует пароля администратора, чтобы службы самого компьютера тоже узнали о выборе.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Проверять обновления

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Ожидание пароля...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Выключено: ничего не ищется, пока вы не попросите.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Обновления безопасности

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Исправления безопасности ставятся сами каждый день.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Исправления безопасности больше не ставятся сами.

