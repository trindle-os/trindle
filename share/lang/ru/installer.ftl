# machine translated (Russian), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Установка { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Добро пожаловать

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Диск

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Конфиденциальность

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Пароль

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Вы

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Готово

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Установка

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Завершено

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Подготовка

# Red line at the foot when the installer's helper could not be started.
installer-no-start = Не удалось запустить установщик: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Не удалось прочитать план.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Не удалось прочитать план: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = Установка остановлена. Больше ничего не записано.

# Name of the round button at the foot that goes back a page.
installer-back = Назад

# Name of the main button at the foot that goes to the next page.
installer-next = Далее

# Name of the main button at the foot of the Ready page that starts the install.
# SAFETY: needs a human review before it ships
installer-install = Установить

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Перезагрузить сейчас

# Beside the keyboard test box on the first page.
installer-try-keyboard = Проверьте клавиатуру

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Печатайте здесь, чтобы проверить клавиатуру

# Note beside Install after one press, when the second press writes beside another system.
# SAFETY: needs a human review before it ships
installer-arm = Нажмите ещё раз, чтобы установить

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = Нажмите ещё раз, чтобы стереть диск и установить

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Выберите язык

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Выберите клавиатуру

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Выберите время

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Выберите, куда установить Trindle

# Note beside Next on the disk password page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-disk-password = Введите пароль диска дважды, одинаково оба раза

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Нужно имя для входа

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Дайте компьютеру имя

# Note beside Next on the account page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-password = Введите пароль дважды, одинаково оба раза

# Title of the first page.
installer-welcome-title = Добро пожаловать в { $name }

# Sentence under the title of the first page.
installer-welcome-help = Выберите язык; клавиатура и время подстроятся под него, и вы можете изменить и то, и другое.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Язык

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Клавиатура

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Время

# Under a card's head while nothing has been chosen in its list.
installer-select = Выбрать

# Title of the disk page.
installer-disks-title = Куда устанавливается { $name }

# Sentence under the title of the disk page.
# SAFETY: needs a human review before it ships
installer-disks-help = Всё на выбранном диске будет заменено, если только вы не освободите место рядом с уже имеющимся.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Не найдено достаточно большого диска. Trindle нужно 20 ГБ.

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = Использовать весь диск, заменив всё, что на нём есть

# Name and words of the choice to keep a partition and share the disk; it says which one.
# SAFETY: needs a human review before it ships
installer-disks-share = Сохранить { $what } и разделить диск

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } остаётся для того, что там сейчас, { $mine } для Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Свободное место

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } и { $last }

# Under a disk's name: what is on it.
installer-disk-has = На нём: { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = На нём: { $list }, зашифровано

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = На нём зашифрованная система

# Under a disk's name: there is nothing on it.
installer-disk-empty = Пусто

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = На нём есть файлы, но нет системы, которую знает Trindle

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = Файл подкачки { $gb } ГБ

# Name of the switch that makes a swap file.
installer-swap-switch = Создать файл подкачки

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Включена безопасная загрузка (Secure Boot), поэтому этот компьютер не может переходить в гибернацию, но файл подкачки всё равно помогает, когда не хватает памяти. Он создаётся, только если на диске достаточно свободного места.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = Место для гибернации, при которой всё открытое сохраняется при выключенном питании, и запас на случай нехватки памяти. Без него компьютер не сможет перейти в гибернацию. Он создаётся, только если на диске достаточно свободного места.

# Title of the page about encrypting the disk.
installer-privacy-title = Шифрование диска

# Sentence under the title of that page.
installer-privacy-help = Зашифрованный диск никто не сможет прочитать, вынув его из этого компьютера.

# Head of the card with the three choices.
installer-privacy-locked = Как заблокирован диск

# Name and words of the choice not to encrypt the disk.
# SAFETY: needs a human review before it ships
installer-privacy-none = Не шифровать диск

# Name and words of the choice to encrypt the disk with a password typed at start-up.
# SAFETY: needs a human review before it ships
installer-privacy-password = Зашифровать и запрашивать пароль при запуске компьютера

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
# SAFETY: needs a human review before it ships
installer-privacy-chip-choice = Зашифровать и разрешить чипу TPM этого компьютера разблокировать диск

# Head of the card about the TPM chip.
installer-privacy-about = О чипе

# In the card about the chip, on a computer that has one.
# SAFETY: needs a human review before it ships
installer-privacy-chip = В этом компьютере есть чип TPM. При первом запуске после установки пароль будет запрошен один раз, пока чип запоминает этот компьютер; после этого чип сам разблокирует диск, и вводить ничего не нужно. Сохраните пароль: это единственный способ войти, если чип заменят или диск перенесут в другой компьютер.

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = В этом компьютере нет чипа TPM, который мог бы использовать Trindle, поэтому пароль будет запрашиваться при каждом запуске. На компьютере с таким чипом он может сам разблокировать диск.

# Title of the disk password page.
# SAFETY: needs a human review before it ships
installer-disk-title = Пароль диска

# Sentence under the title of the disk password page.
installer-disk-help = Его вводят первым делом при запуске компьютера.

# Head of the card with the disk password boxes, and the name of the first box.
# SAFETY: needs a human review before it ships
installer-disk-password = Пароль диска

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Предложить парольную фразу

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Повторите то же самое

# Red line under the password boxes when the two do not match.
# SAFETY: needs a human review before it ships
installer-mismatch = Пароли не совпадают.

# Head of the card that tells the person to write the password down.
# SAFETY: needs a human review before it ships
installer-disk-write-down = Запишите его

# Words in that card.
# SAFETY: needs a human review before it ships
installer-disk-write-down-text = Запишите этот пароль на бумаге и храните в надёжном месте, вдали от компьютера. Без него диск невозможно открыть ни вам, ни кому-либо другому, никогда. Пути назад нет. Четыре обычных слова образуют парольную фразу, которую легко набрать и трудно угадать.

# Title of the account page.
installer-you-title = О вас

# Sentence under the title of the account page.
installer-you-help = Это учётная запись, с которой вы входите.

# Head of the card with the name boxes.
installer-account = Ваша учётная запись

# Label and name of the box for the person's name.
installer-your-name = Ваше имя

# Label and name of the box for the sign-in name.
installer-user-name = Имя для входа

# Label and name of the box for the computer's name.
installer-host-name = Имя этого компьютера

# Head of the card with the password boxes.
# SAFETY: needs a human review before it ships
installer-your-password = Ваш пароль

# Label and name of the first password box.
# SAFETY: needs a human review before it ships
installer-password = Пароль

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Насколько трудно угадать

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Нужно имя для входа.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Это имя слишком длинное.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Начните со строчной буквы.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Только строчные буквы, цифры, - и _.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Это имя уже занято системой.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Дайте компьютеру имя.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Только буквы, цифры и -.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Компьютер угадает это сразу же.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] Компьютер угадает это примерно за { $n } минуту.
        [few] Компьютер угадает это примерно за { $n } минуты.
        [many] Компьютер угадает это примерно за { $n } минут.
       *[other] Компьютер угадает это примерно за { $n } минуты.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] Компьютер угадает это примерно за { $n } час.
        [few] Компьютер угадает это примерно за { $n } часа.
        [many] Компьютер угадает это примерно за { $n } часов.
       *[other] Компьютер угадает это примерно за { $n } часа.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] Компьютер угадает это примерно за { $n } день.
        [few] Компьютер угадает это примерно за { $n } дня.
        [many] Компьютер угадает это примерно за { $n } дней.
       *[other] Компьютер угадает это примерно за { $n } дня.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] Компьютер угадает это примерно за { $n } год.
        [few] Компьютер угадает это примерно за { $n } года.
        [many] Компьютер угадает это примерно за { $n } лет.
       *[other] Компьютер угадает это примерно за { $n } года.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Компьютеру понадобится больше времени, чем длится вся записанная история, чтобы угадать это.

# Title of the Ready page.
installer-ready-title = Готово к установке

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Пока ничего не изменено

# Head of the card about security updates.
installer-ready-security = Обновления безопасности

# In the card about security updates.
installer-ready-security-text = { $name } сам устанавливает исправления безопасности Debian каждый день. Это нельзя отключить.

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = Всё на { $disk } будет заменено.

# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Всё на { $part } сохранится, но сначала сделайте резервную копию: при уменьшении раздела файлы можно потерять, если отключится питание.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Диск ещё не выбран.

# Name of a row in the list: the whole disk will be used.
# SAFETY: needs a human review before it ships
installer-ready-whole = Весь { $disk }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = Всё на нём будет заменено на { $name }.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Рядом с { $part }

# Words of that row.
installer-ready-beside-text = { $part } будет уменьшен до { $size }, а { $name } займёт освободившееся место.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Нет диска

# Words of that row.
installer-ready-no-disk-text = Диск не выбран.

# Name of a row in the list: the disk will not be encrypted.
# SAFETY: needs a human review before it ships
installer-ready-plain = Не зашифровано

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-plain-text = Любой, кто вынет диск, сможет его прочитать.

# Name of a row in the list: the disk will be encrypted.
# SAFETY: needs a human review before it ships
installer-ready-locked = Зашифровано

# Words of that row when a password is typed at each start.
# SAFETY: needs a human review before it ships
installer-ready-locked-password = Вы вводите пароль диска при каждом запуске компьютера.

# Words of that row when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-ready-locked-chip = Пароль нужен один раз, затем диск разблокирует чип TPM этого компьютера.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Клавиатура: { $keyboard }, время: { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = Установка { $name }

# Head of the card that holds the short tour.
installer-tour = Краткий обзор

# Title of the last page.
installer-done-title = { $name } установлен

# Sentence under the title of the last page.
installer-done-help = Извлеките USB-накопитель и перезагрузите компьютер.

# Head of the card that repeats the warning about the disk password.
# SAFETY: needs a human review before it ships
installer-done-keep = Храните пароль диска в надёжном месте

# In that card when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-keep-chip = Диск зашифрован. При первом запуске пароль диска будет запрошен один раз, пока чип запоминает этот компьютер; после этого диск открывается сам. Запишите этот пароль на бумаге и храните в надёжном месте вдали от компьютера. Без него диск невозможно открыть ни вам, ни кому-либо другому, никогда.

# In that card when the password is typed at each start.
# SAFETY: needs a human review before it ships
installer-keep-password = Диск зашифрован. Если вы ещё этого не сделали, запишите пароль диска на бумаге сейчас и храните в надёжном месте вдали от компьютера. Без него диск невозможно открыть ни вам, ни кому-либо другому, никогда.

# Title of the first slide.
installer-slide-ring = Всё в одном круге

# Words of the first slide.
installer-slide-ring-text = Переместите указатель к краю экрана и дважды щёлкните, и откроется кольцо: ваши программы по внешнему краю, время и переключатели посередине.

# Title of the second slide.
installer-slide-snapshots = Верните то, что сломало обновление

# Words of the second slide.
installer-slide-snapshots-text = Снимок системы делается до и после каждого изменения программ. В «Настройках» можно вернуть всю систему к любому из них, а меню загрузки сделает это, если компьютер не запускается.

# Title of the third slide.
installer-slide-apps = Приложения, которые не мешают

# Words of the third slide.
installer-slide-apps-text = Приложения Flathub, приложения самого Debian, AppImage и источники программ в одном месте, с резервными копиями, которые можно перенести на другой компьютер.

# Title of the fourth slide.
installer-slide-yours = Ваш, и быстро

# Words of the fourth slide.
installer-slide-yours-text = Светлая или тёмная тема, собственные звуки и буфер обмена, который помнит скопированное.

# Title of the fifth slide.
installer-slide-small = Небольшой и тихий

# Words of the fifth slide.
installer-slide-small-text = Рабочий стол почти не нагружает память и запускается за несколько секунд, так что старый ноутбук снова ощущается как новый

# Title of the tour slide about the ring.
installer-tour-ring = Кольцо - это весь рабочий стол

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Подведите указатель в этот угол, и кольцо откроется

# Note on the ring picture, at the time.
installer-tour-ring-time = Время и дата посередине

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Ваши программы расположены по внешнему краю

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Звук, экран, все ваши приложения, режим «не засыпать» и выключение

# Title of the tour slide about Settings.
installer-tour-settings = Настройки простыми словами

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Введите здесь, чтобы найти настройку, не разыскивая её

# Note on the Settings picture, at Network.
installer-tour-settings-network = В разделе «Сеть» вы подключаетесь к сети Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Светлая или тёмная тема, когда захотите

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Двадцать цветов, или введите свой

# Title of the tour slide about Apps.
installer-tour-apps = Приложения для всего остального

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Ищите программу по названию или по тому, что она делает

# Note on the Apps picture, at the list.
installer-tour-apps-many = Тысячи бесплатных программ из Flathub и Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Обновления для всего, включая сам Trindle

# Title of the tour slide about voice control.
installer-tour-voice = Можно говорить с ним вместо этого

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = Голосовое управление находится здесь: «Настройки», затем «Кольцо»

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Один раз нажмите «Добавить голос». Это около 150 МБ и займёт несколько минут

# Note on the voice picture, about what to say.
installer-tour-voice-say = Затем коснитесь середины кольца и назовите приложение, скажите «громче» или «настройки Wi-Fi»

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Сказанное распознаётся на этом компьютере и никуда не уходит

# Title of the tour slide about snapshots.
installer-tour-snapshots = Путь назад, прежде чем что-то изменится

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Компьютер делает снимок перед каждым обновлением

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Если что-то сломается, вернитесь к прежнему состоянию через меню загрузки

# Name of the picture of the ring, and the head of its card.
installer-live-caption = Кольцо, где находится всё

# Sentence under the title of the welcome from a stick.
installer-live-help = Вы работаете с USB-накопителя, поэтому на этом компьютере ничего не изменено.

# Head of the card with the three first things to do.
installer-live-begin = Для начала

# Name of the first thing to do.
installer-live-open = Откройте кольцо

# Line under it.
installer-live-open-line = Дважды щёлкните у края экрана или нажмите Super.

# Name of the second thing to do.
installer-live-speak = Говорите с ним

# Line under it when voice control is there.
installer-live-speak-line = Коснитесь середины кольца и скажите, что вам нужно.

# Line under it when voice control has to be added.
installer-live-speak-add = Добавьте голос в «Настройках», «Кольцо», затем коснитесь середины и говорите.

# Name of the third thing to do.
installer-live-tab = Всё в кольце

# Line under it.
installer-live-tab-line = Открытое, ваши уведомления и буфер обмена: всё здесь.

# The choice of the dark look.
installer-live-dark = Тёмная

# The choice of the light look.
installer-live-light = Светлая

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Светлая или тёмная тема для всего рабочего стола

# Name of the round button that closes this window and opens the ring.
installer-live-around = Сначала осмотреться

# Name of the main button that opens the installer.
installer-live-install = Установить { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = Освобождение места

# Foot of the install page while the disk is prepared.
installer-step-prepare = Подготовка диска

# Foot of the install page while a place is made beside another system.
installer-step-place = Создание места для Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = Проверка того, что уже есть на диске

# Foot of the install page while another system is made smaller.
installer-step-smaller = Уменьшение другой системы

# Foot of the install page while the disk is encrypted.
installer-step-lock = Шифрование диска

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Настройка чипа компьютера

# Foot of the install page while the system is copied.
installer-step-copy = Копирование Trindle на диск

# Foot of the install page while settings are written.
installer-step-settings = Запись настроек

# Foot of the install page while the person's account is made.
installer-step-account = Создание вашей учётной записи

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Удаление частей, нужных только для запуска с накопителя

# Foot of the install page while the start-up files are made.
installer-step-startup = Создание файлов запуска

# Foot of the install page while the boot menu is set up.
installer-step-boot = Настройка меню загрузки

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Настройка снимков

# Foot of the install page at the very end.
installer-step-finishing = Завершение

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = Не удалось запустить { $program }: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } не завершилась. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = Не удалось записать { $path }: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } - это не диск.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } - это не диск этого компьютера.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Диск не выбран.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Trindle не может использовать это имя для входа.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Trindle не может использовать это имя компьютера.

# Red line at the foot: the language cannot be used.
installer-bad-language = Trindle не может использовать этот язык.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Trindle не может использовать эту клавиатуру.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Trindle не может использовать этот часовой пояс.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = В этом имени есть то, что Trindle не может записать.

# Red line at the foot: a password has a line break in it.
# SAFETY: needs a human review before it ships
installer-bad-password = Пароль не может содержать перевод строки.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = Вторая половина установщика должна работать от имени root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = На этом диске нет загрузочного раздела для совместного использования, поэтому Trindle нельзя установить рядом с тем, что уже есть.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Не удалось найти новую часть диска.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = Не удалось прочитать размер { $part }.

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle не может уменьшить раздел { $fs }.

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle не может уменьшить раздел неизвестного типа.

# Red line at the foot: encrypting needs a password and there is none.
# SAFETY: needs a human review before it ships
installer-disk-password-needed = Для шифрования диска нужен пароль.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Не удалось скопировать систему: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Не удалось скопировать систему. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Нет места для файла подкачки.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Не удалось прочитать расположение файла подкачки на диске.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Не удалось опознать новые разделы.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Не удалось задать часовой пояс: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Не удалось установить меню загрузки на этот компьютер.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Уверены?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Что открыто, буфер обмена и кнопка, которая очищает экран

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } это флешка, с которой запущена Trindle.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } используется. Закройте то, что его использует, и попробуйте снова.

# Said beside how far the install has got, once it has started.
installer-started = Установка началась

# The button that ends an install under way.
installer-halt = Остановить

# Beside Stop after its first press.
installer-halt-sure = Остановить установку? Прежнее содержимое диска уже стёрто.

# While an install is being stopped.
installer-halting = Остановка

# When Stop could not let go of the disk.
installer-halt-stuck = Не удалось освободить диск. Перезагрузите компьютер и попробуйте снова.

# The copy is done and the person is still answering.
installer-step-waiting = Скопировано. Ждём ваших ответов.



installer-already-running = На этом компьютере уже запущена другая программа установки.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Для Trindle нужно не менее 1 GB памяти, а на этом компьютере её меньше.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Диск можно зашифровать только на компьютере, который запускается через UEFI, а этот так не запускается.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows на { $part } была переведена в режим гибернации или оставлена в режиме быстрого запуска, поэтому при уменьшении раздела можно потерять её файлы. Запустите Windows, выберите «Перезагрузка» и повторите попытку.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle установлен, но снимки настроить не удалось: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle установлен, но меню загрузки не удалось настроить так, чтобы в нём были снимки: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Настроено не всё

