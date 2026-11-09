# machine translated (Russian), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Запуск при входе

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay автозапуск запуск вход автоматически загрузка задержка

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Приложения, которые открываются сами, когда вы входите в систему. Задержка позволяет приложению сначала дождаться сети или другого приложения.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = При входе ничего не запускается само.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = При входе само запускается { $name }.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] При входе само запускается { $n } приложение.
        [few] При входе сами запускаются { $n } приложения.
        [many] При входе сами запускаются { $n } приложений.
       *[other] При входе сами запускаются { $n } приложения.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] Ещё одно хранится в списке выключенным.
        [few] Ещё { $n } хранятся в списке выключенными.
        [many] Ещё { $n } хранятся в списке выключенными.
       *[other] Ещё { $n } хранятся в списке выключенными.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = Сразу

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Через минуту

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = Через 2 минуты

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] Через { $n } секунду
        [few] Через { $n } секунды
        [many] Через { $n } секунд
       *[other] Через { $n } секунды
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Задержка для { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Запускать при входе

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = Больше не запускать при входе

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Не удалось убрать его, поэтому он по-прежнему запускается при входе.

# Red words under the add card when something could not be added.
settings-startup-not-added = Не удалось добавить.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = Найти приложение

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Запускать при входе

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = Не удалось добавить: приложение больше не установлено.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = например, syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Выбрать программу на диске

# Heading of the line for adding a command.
settings-startup-command = Команда

# Small words under the command line.
settings-startup-command-about = Как в Терминале, или программа на диске.

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Введите команду или выберите программу.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Выберите программу для запуска при входе

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Ничего не запускается само

# Small words under that line.
settings-startup-none-about = Добавьте приложение рядом.

# Title of the card listing what starts at login.
settings-startup-list-title = Запускается при входе

# Words behind the i of the card listing what starts at login.
settings-startup-about = Каждое приложение может немного подождать после входа, пока не будут готовы сеть или другое приложение. Его переключатель оставляет его здесь без запуска; корзина совсем убирает его из запуска при входе.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } выполняет: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Выбрать из установленных приложений

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Ввести команду или выбрать программу

# Heading of the line for adding an installed app.
settings-startup-app-heading = Установленное приложение

# Small words under that line.
settings-startup-app-about = Выберите одно из списка приложений.

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Команда или программа

# Small words under that line.
settings-startup-cmd-about = Введите или выберите на диске.

# Words behind the i of the Add an app card.
settings-startup-add-about = Выберите из установленных приложений или укажите команду, как вы ввели бы её в Терминале, либо программу, выбранную на диске. Она запустится при следующем входе, сразу; задержку можно выбрать, когда она появится в списке.

# Title of the card for adding an app.
settings-startup-add-title = Добавить приложение
