# machine translated (Russian), not yet reviewed by a person.

# Name of the Permissions page, in the page list and as its heading.
# SAFETY: needs a human review before it ships
settings-permissions-title = Разрешения

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = разрешения разрешить конфиденциальность flatpak flathub песочница интернет сеть файлы звук микрофон камера bluetooth уведомления плагины deb x11 xwayland старые программы steam wine java экран терминал компьютер thunderbolt usb4 док устройства

# Help line at the foot of the Permissions page.
# SAFETY: needs a human review before it ships
settings-permissions-help = Что может использовать каждое приложение. Приложения из Flathub, программы, установленные в собственную папку, и плагины устанавливаются каждое в свою папку и по умолчанию ограничены. Закрашенный значок означает «разрешено»; щёлкните по нему, чтобы изменить. Изменение для приложения из Flathub вступает в силу при следующем запуске. Разрешённые устройства Thunderbolt и USB4 всегда подключаются без вопросов; забудьте устройство, чтобы вас спросили снова.

# Red words when a permission could not be changed and nothing more is known.
# SAFETY: needs a human review before it ships
settings-permissions-not-changed = Не удалось изменить.

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Сохранено, но плагин не удалось запустить снова. Изменение вступит в силу при следующем запуске.

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Сохранено

# Help text of the AppImages card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-images = Каждый AppImage запускается в отдельной изолированной среде с личной домашней папкой. Закрашенный значок означает «разрешено»; щёлкните по нему, чтобы изменить. Изменение вступает в силу при следующем запуске AppImage.

# Help text of the Started from Folders card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-programs = Программы, запущенные из «Папок», работают каждая в отдельной изолированной среде с личной домашней папкой. Закрашенный значок означает «разрешено»; щёлкните по нему, чтобы изменить. Изменение вступает в силу при следующем запуске программы.

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
settings-permissions-lets-in-screens = Эти порты передают только изображение на экран, поэтому ни одно устройство не может получить доступ к памяти этого компьютера, и спрашивать не о чем.

# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = Эти порты передают только USB и изображение на экран, поэтому ни одно устройство не может получить доступ к памяти этого компьютера, и спрашивать не о чем.

# Help text on the Thunderbolt and USB4 card when the ports let every device in.
# SAFETY: needs a human review before it ships
settings-permissions-lets-in-all = Эти порты сами подключают любое устройство, поэтому спрашивать не о чем. Это задаётся в прошивке компьютера.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Эти порты передают только изображение на экран.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Эти порты передают только USB и изображение на экран.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
# SAFETY: needs a human review before it ships
settings-permissions-short-firmware = Прошивка подключает любое устройство.

# Under a package's name: its version number is { $v }.
settings-permissions-version = Версия { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Версия { $v }. Ничего не запрашивает.

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }

# Red words when a Thunderbolt or USB4 device could not be forgotten.
# SAFETY: needs a human review before it ships
settings-permissions-not-forgotten = Не удалось забыть устройство.

# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt и USB4

# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
# SAFETY: needs a human review before it ships
settings-permissions-thunder-about = Разрешённые устройства Thunderbolt и USB4 всегда подключаются без вопросов; забудьте устройство, чтобы вас спросили снова. { $about }

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = Порты

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Узнаём, что они пропускают…

# Heading of a line on the Thunderbolt and USB4 card.
# SAFETY: needs a human review before it ships
settings-permissions-devices-allowed = Разрешённые устройства

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Чтение…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Память отделена

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = Нет портов Thunderbolt и USB4

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = Поэтому спрашивать не о чем.

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = Нет: так ничего не подключается.

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Спрашивать не о чем

# Under Devices allowed when the ports let every device in themselves.
# SAFETY: needs a human review before it ships
settings-permissions-every-one = Все, сами порты.

# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-iommu = Этот компьютер также не пускает устройства к памяти, которую им не выдали, поэтому риск меньше. Если забыть устройство, оно останется подключённым, пока его не отключат.

# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-no-iommu = Если забыть устройство, оно останется подключённым, пока его не отключат; в следующий раз о нём спросят снова.

# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
# SAFETY: needs a human review before it ships
settings-permissions-asked-before = Вопрос перед подключением нового устройства

# Under the heading above.
# SAFETY: needs a human review before it ships
settings-permissions-forget-one = Забудьте устройство, чтобы вас спросили снова.

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
settings-permissions-reading-apps = Узнаём, что может использовать каждое приложение…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Узнаём, что может использовать каждое из приложений: { $n }…
        [few] Узнаём, что может использовать каждое из приложений: { $n }…
        [many] Узнаём, что может использовать каждое из приложений: { $n }…
       *[other] Узнаём, что может использовать каждое из приложений: { $n }…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = Нет установленных приложений или плагинов в собственной изолированной среде.

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = Одно приложение работает в собственной изолированной среде и использует только то, что разрешено здесь.

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } приложение или плагин работает в собственной изолированной среде и использует только то, что разрешено здесь.
        [few] Приложения и плагины в собственных изолированных средах: { $n }. Они используют только то, что разрешено здесь.
        [many] Приложения и плагины в собственных изолированных средах: { $n }. Они используют только то, что разрешено здесь.
       *[other] Приложения и плагины в собственных изолированных средах: { $n }. Они используют только то, что разрешено здесь.
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
settings-permissions-from-folders = Запущены из «Папок»

# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = Ничего не установлено.

# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = Пока ничего не запускалось.

# Help text of the Flathub apps card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-flathub-about = Каждое приложение из Flathub работает в собственной изолированной среде и может использовать только то, что разрешено здесь. Закрашенный значок означает «разрешено»; щёлкните по нему, чтобы изменить. Изменение вступает в силу при следующем запуске приложения.

# Help text of the In their own folder card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-debs-about = Программы, установленные в собственную папку, по умолчанию ограничены. Закрашенный значок означает «разрешено»; щёлкните по нему, чтобы изменить.

# Help text of the Plugins card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-plugins-about = Плагины по умолчанию ограничены, и им можно разрешить только то, что они запрашивают. Плагин, работающий в фоне, сразу запускается снова с изменением.

# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Коды для набора

# SAFETY: needs a human review before it ships
settings-permissions-codes-about = Короткие коды, которые Trindle заменяет словами при наборе, и модуль, которому принадлежит каждый код. Отключите коды модуля круглой кнопкой.

settings-permissions-codes-none = Активных кодов нет

# SAFETY: needs a human review before it ships
settings-permissions-codes-none-sub = Здесь перечислены коды модулей, которым разрешено их раскрывать.

# Button on a plugin's row: stops that plugin's codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-off = Отключить коды модуля { $name }

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

settings-permissions-not-added = Запущено из Folders, не добавлено в Apps

settings-permissions-kind-tip = Тип приложения



# Heading of the card for programs that are not in a box.
settings-outside-heading = Программы вне изолированной среды

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle проверяет каждые несколько секунд и сообщает вам. Она не может остановить эти программы и может пропустить быструю. Программа в изолированной среде ограничена ею.

# Label of the switch for the notice about the network.
settings-outside-network = Сообщать, когда программа выходит в сеть

# Label of the switch for the notice about the person's files.
settings-outside-files = Сообщать, когда у программы открыты ваши файлы

# Under those two switches.
settings-outside-once = Один раз для каждой программы.

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Сообщить снова про «{ $name }»

# Under a program marked fine for the network.
settings-outside-fine-network = Выход в сеть допустим.

# Under a program marked fine for the person's files.
settings-outside-fine-files = Открытые файлы допустимы.

# The line in the card when no program is marked fine.
settings-outside-no-fine = Ни одна программа не отмечена как допустимая.

# Heading of the card for downloaded files.
settings-downloads-heading = Скачанные файлы

# Help text of that card (the information button).
settings-downloads-about = Файл из интернета это файл в «Загрузках» или отмеченный браузером. Программа, которая его открывает, получает только этот файл, только для чтения, без интернета и без ваших папок. Это не относится к собственной кнопке «Открыть» в браузере, которая запускает программу сама.

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Открывать скачанные файлы в изолированной среде

# Under that switch.
settings-downloads-box-sub = Без интернета и только этот файл, только для чтения.

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Открывать файлы { $kind } как обычно

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = Файлы { $kind }

# Under a file type's switch.
settings-downloads-normal-sub = Включено: открываются как обычно, с интернетом.

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = Типов пока нет. Тип появится в списке, когда скачанный файл откроется в изолированной среде.

