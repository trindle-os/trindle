# machine translated (Russian), not yet reviewed by a person.

# Name of the Permissions page, in the page list and as its heading.
# SAFETY: needs a human review before it ships
settings-permissions-title = Разрешения

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = разрешения разрешить конфиденциальность flatpak flathub песочница интернет сеть файлы звук микрофон камера bluetooth уведомления плагины deb x11 xwayland старые программы steam wine java экран терминал компьютер thunderbolt usb4 док устройства

# Help line at the foot of the Permissions page.
# SAFETY: needs a human review before it ships
settings-permissions-help = На этой странице показано, что может использовать каждое приложение. Приложения из Flathub, программы, установленные в собственную папку, и плагины работают каждое в своей изолированной среде с ограничениями по умолчанию. Изменение для приложения из Flathub вступает в силу при следующем запуске. Разрешённые устройства Thunderbolt и USB4 всегда подключаются без вопросов. Забудьте устройство, если хотите, чтобы вас спросили снова.

# Red words when a permission could not be changed and nothing more is known.
# SAFETY: needs a human review before it ships
settings-permissions-not-changed = Не удалось изменить.

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Сохранено, но Trindle не смогла перезапустить плагин. Изменение вступит в силу при следующем запуске.

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Сохранено

# Help text of the AppImages card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-images = Каждый AppImage запускается в своей изолированной среде с личной домашней папкой. Изменение вступает в силу при следующем запуске AppImage.

# Help text of the Started from Folders card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-programs = Каждая программа, которую вы запускаете из Folders, работает в своей изолированной среде с личной домашней папкой. Изменение вступает в силу при следующем запуске программы.

# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }: { $words }

# Under the Allow older programs switch.
# SAFETY: needs a human review before it ships
settings-permissions-older-why = Каждая изолирована от остальных. Если выключено, они не открываются.

# Heading of the card with the switch for older programs.
settings-permissions-older = Старые программы

# Label and tip of the switch that lets older programs open.
# SAFETY: needs a human review before it ships
settings-permissions-allow-older = Разрешить старые программы

# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = Эти порты предназначены только для экранов, поэтому ни одно устройство не может получить доступ к памяти этого компьютера. Спрашивать не о чем.

# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = Эти порты предназначены только для USB и экранов, поэтому ни одно устройство не может получить доступ к памяти этого компьютера. Спрашивать не о чем.

# Help text on the Thunderbolt and USB4 card when the ports let every device in.
# SAFETY: needs a human review before it ships
settings-permissions-lets-in-all = Эти порты подключают любое устройство, поэтому спрашивать не о чем. Это задаётся в прошивке компьютера.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Эти порты используются только для экранов.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Эти порты используются только для USB и экранов.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
# SAFETY: needs a human review before it ships
settings-permissions-short-firmware = Прошивка подключает любое устройство.

# Under a package's name: its version number is { $v }.
settings-permissions-version = Версия { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Версия { $v }. Ничего не запрашивает.

# Under a plugin's name: its version number is { $v }, and { $notes } lists what it may do that matters most, such as "it can act as you and it can type into your windows".
settings-permissions-version-notes = Версия { $v }: { $notes }.

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }

# Red words when a Thunderbolt or USB4 device could not be forgotten.
# SAFETY: needs a human review before it ships
settings-permissions-not-forgotten = Не удалось забыть устройство.

# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt и USB4

# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
# SAFETY: needs a human review before it ships
settings-permissions-thunder-about = Разрешённые вами устройства Thunderbolt и USB4 всегда подключаются без вопросов. Забудьте устройство, если хотите, чтобы вас спросили снова. { $about }

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = Порты

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Проверяем порты…

# Heading of a line on the Thunderbolt and USB4 card.
# SAFETY: needs a human review before it ships
settings-permissions-devices-allowed = Разрешённые устройства

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Загрузка…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Защита памяти

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = Нет портов Thunderbolt и USB4

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = Поэтому спрашивать не о чем.

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = Устройства так не подключаются.

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Спрашивать не о чем

# Under Devices allowed when the ports let every device in themselves.
# SAFETY: needs a human review before it ships
settings-permissions-every-one = Все. Решают сами порты.

# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-iommu = Этот компьютер также не пускает устройства к памяти, которая им не принадлежит, поэтому риск меньше. Если забыть устройство, оно останется подключённым, пока вы его не отключите.

# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-no-iommu = Если забыть устройство, оно останется подключённым, пока вы его не отключите. При следующем подключении вас спросят снова.

# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
# SAFETY: needs a human review before it ships
settings-permissions-asked-before = Вас спрашивают о новых устройствах

# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Не удалось прочитать список разрешённых устройств: { $why }

# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = Пока ни одному устройству не разрешено подключаться без вопросов.

# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
# SAFETY: needs a human review before it ships
settings-permissions-forget-device = Забыть это устройство

# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = Устройство Thunderbolt

# Under an allowed device's name when its maker is not known.
# SAFETY: needs a human review before it ships
settings-permissions-may-always = Всегда может подключаться

# Under an allowed device's name: the maker is { $vendor }.
# SAFETY: needs a human review before it ships
settings-permissions-made-by = Производитель: { $vendor }. Всегда может подключаться.

# Headline while the list of apps is being read.
settings-permissions-reading-apps = Проверяем, что могут использовать приложения…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Проверяем, что могут использовать приложения (всего { $n })…
        [few] Проверяем, что могут использовать приложения (всего { $n })…
        [many] Проверяем, что могут использовать приложения (всего { $n })…
       *[other] Проверяем, что могут использовать приложения (всего { $n })…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = Вы не установили приложений или плагинов, которые работают в собственной изолированной среде.

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = Одно приложение работает в собственной изолированной среде и может использовать только то, что вы разрешите здесь.

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } приложение или плагин работает в собственной изолированной среде и может использовать только то, что вы разрешите здесь.
        [few] Приложений и плагинов: { $n }. Каждый работает в своей изолированной среде и может использовать только то, что вы разрешите здесь.
        [many] Приложений и плагинов: { $n }. Каждый работает в своей изолированной среде и может использовать только то, что вы разрешите здесь.
       *[other] Приложений и плагинов: { $n }. Каждый работает в своей изолированной среде и может использовать только то, что вы разрешите здесь.
    }

# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Приложения из Flathub

# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = В собственной папке

# Heading of the card for plugins.
settings-permissions-plugins = Плагины

# Heading of the card for AppImages.
settings-permissions-appimages = AppImage

# Heading of the card for programs started from Folders.
settings-permissions-from-folders = Запущены из Folders

# Tab name on Settings, Permissions, beside Started from Folders.
settings-permissions-other = Другие настройки

# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = Ничего не установлено.

# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = Пока нет.

# Help text of the Flathub apps card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-flathub-about = Каждое приложение из Flathub работает в собственной изолированной среде и может использовать только то, что разрешено здесь. Изменение вступает в силу при следующем запуске приложения.

# Help text of the In their own folder card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-debs-about = Программы, которые поставляются с Trindle, например Терминал, и программы, которые вы установили из файлов .deb. Каждая работает в собственной папке с ограничениями по умолчанию.

# Help text of the Plugins card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-plugins-about = Плагины сначала ограничены, и вы можете разрешить только то, что плагин запрашивает. Плагин, работающий в фоне, сразу перезапускается, когда вы что-то меняете.

# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Коды для набора

# SAFETY: needs a human review before it ships
settings-permissions-codes-about = Короткие коды, которые Trindle заменяет словами при наборе. Для каждого указан плагин, которому он принадлежит.

settings-permissions-codes-none = Активных кодов нет

# Button on a plugin's row: stops that plugin's codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-off = Отключить коды плагина { $name }

settings-permissions-codes-count =
    { $n ->
        [one] { $n } код, { $id }
        [few] { $n } кода, { $id }
        [many] { $n } кодов, { $id }
       *[other] { $n } кода, { $id }
    }

# A code's second line: what it types and who owns it.
settings-permissions-code-types = Вводит «{ $words }» для { $name }

settings-permissions-codes-more =
    { $n ->
        [one] Активен ещё { $n } код
        [few] Активны ещё { $n } кода
        [many] Активны ещё { $n } кодов
       *[other] Активны ещё { $n } кода
    }

# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread-line = Не удалось прочитать папку плагинов

# Red words under that line.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread = Не удалось прочитать папку плагинов, поэтому в списке могут отсутствовать некоторые коды.

settings-permissions-not-added = Открыто из Folders, нет в «Приложениях»

settings-permissions-kind-tip = Тип приложения



# Heading of the card for programs that are not in a box.
settings-outside-heading = Программы вне изолированной среды

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle проверяет каждые несколько секунд и сообщает вам, что нашла. Она не может остановить эти программы и может пропустить ту, что работает лишь мгновение. Программы в изолированной среде могут использовать только то, что она разрешает.

# Label of the switch for the notice about the network.
settings-outside-network = Сообщать мне, когда программы используют сеть

# Label of the switch for the notice about the person's files.
settings-outside-files = Сообщать мне, когда программы открывают мои файлы

# Under those two switches.
settings-outside-once = Один раз для каждой программы.

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Больше не игнорировать «{ $name }»

# Under a program marked fine for the network.
settings-outside-fine-network = Использование сети допустимо.

# Under a program marked fine for the person's files.
settings-outside-fine-files = Открытые файлы допустимы.

# The line in the card when no program is marked fine.
settings-outside-no-fine = Ни одна программа не отмечена как допустимая.

# Heading of the card for downloaded files.
settings-downloads-heading = Скачанные файлы

# Help text of that card (the information button).
settings-downloads-about = Файлы в «Загрузках» и файлы, которые браузер пометил как скачанные, открываются в изолированной среде. Программа, которая открывает такой файл, получает только его. Она может только читать его и не может пользоваться интернетом и вашими папками. Это не относится к собственной кнопке «Открыть» в браузере, которая запускает программу сама.

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Открывать скачанные файлы в изолированной среде

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Открывать файлы { $kind } как обычно

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = Файлы { $kind }

# Under a file type's switch.
settings-downloads-normal-sub = Открываются как обычно, с интернетом.

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = Типов файлов пока нет. Тип появится здесь после того, как скачанный файл откроется в изолированной среде.
