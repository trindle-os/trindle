# machine translated (Russian), not yet reviewed by a person.

# The page shown before a Flatpak program is installed, with what it will be able to reach.
# Headline over the page; the source of the program is given.
store-fp-headline-from = Установить { $name } из { $host }?

# Headline over the page for a program that has been downloaded.
store-fp-headline = Установить { $name }?

# Name of the card that lists what the program will be able to reach.
store-fp-card = К чему у неё будет доступ

# Tooltip of the round button that leaves without installing.
store-fp-cancel = Не устанавливать

# The main button.
store-fp-install = Установить

# Said while Flatpak is being asked what the program wants.
store-fp-reading = Читаем, что ей нужно…

# Red words when what the program asks for could not be read.
store-fp-unread = Trindle не смог прочитать, что ей нужно.

# Red words after Install was pressed once for a program whose list could not be read.
store-fp-unread-again = Нажмите «Установить» ещё раз, чтобы продолжить.

# Said when the program is already on this computer; the name is given.
store-fp-have = { $name } уже установлена.

# Said when the program asks to reach nothing outside its own box.
store-fp-none = Ничего за пределами её собственной песочницы.

# Each line below is one thing the program will be able to reach.
store-fp-network = Интернет и ваша локальная сеть

store-fp-x11 = Окна других приложений и то, что вы в них печатаете

store-fp-sound = Звук и ваш микрофон

store-fp-devices = Ваша камера и все другие подключённые устройства

store-fp-input = Игровые контроллеры и другие устройства ввода

store-fp-kvm = Быстрые виртуальные машины

store-fp-shm = Память, общая с другими приложениями

store-fp-usb = Устройства USB

store-fp-canbus = Устройства с шиной CAN

store-fp-cups = Ваши принтеры

store-fp-multiarch = Запуск 32-битных программ

store-fp-escape = Запуск любой программы на этом компьютере

# A reference that also adds a second place to get apps from, named by its address.
store-fp-adds-source = Добавить { $host } в источники приложений

# Last line, when the program asks for more than the lines above say.
store-fp-other = И другие настройки

store-fp-bluetooth = Устройства Bluetooth

store-fp-smartcard = Смарт-карты

store-fp-ssh = Ваши ключи SSH

store-fp-gpg = Ваши ключи GPG

store-fp-devel = Средства отладки, с которыми она может следить за запущенными ею программами и управлять ими

store-fp-session-bus = Все программы, работающие в вашем сеансе

store-fp-system-bus = Все службы на этом компьютере

# A place the program can open and change; the place is given, as below.
store-fp-files = Открывать и изменять: { $what }

# The same for a place it can only look at.
store-fp-files-read = Просматривать: { $what }

store-fp-where-host = все файлы на этом компьютере

store-fp-where-system = системные файлы этого компьютера

store-fp-where-home = вся ваша домашняя папка, включая скрытые файлы

store-fp-where-path = папка { $path }

# A program on your session that it can talk to; the name is given.
store-fp-talk = Обращаться к { $name }

# A service the program runs on your session.
store-fp-own = Запускать службу { $name }

store-fp-talk-system = Обращаться к системной службе { $name }

store-fp-own-system = Запускать системную службу { $name }

# Said in red when Install failed and taking the program out again failed too.
store-fp-undo-failed = Возможно, она установлена не полностью.

# Said in red when a bundle could not be copied for a safe read.
store-fp-copy-failed = Не удалось скопировать файл, чтобы безопасно его прочитать.
