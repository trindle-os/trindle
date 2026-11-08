# machine translated (Russian), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } приложение
        [few] { $count } приложения
        [many] { $count } приложений
       *[other] { $count } приложения
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] ждёт { $count } обновление
        [few] ждут { $count } обновления
        [many] ждут { $count } обновлений
       *[other] ждут { $count } обновления
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } на выбор из Flathub; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = всё ваше обновлено

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Нет приложений, подходящих под «{ $text }».

# Line under the title of an empty page of apps.
store-grid-none-here = Здесь пока нет приложений.

# Advice in the empty page of a search.
store-grid-try-shorter = Попробуйте слово покороче или просмотрите категорию слева.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Ищите по названию выше или просмотрите другую категорию слева.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = Показано { $shown } из { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = Показать ещё { $next }

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Загружается список приложений из Flathub. В первый раз это занимает около минуты.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Приложениям не удалось подключиться к Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Этот компьютер не подключён к интернету. «Приложения» повторят попытку сами, как только подключение появится.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Приложения на выбор берутся из Flathub через интернет.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } и настройки приложений слева работают без него.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } и настройки приложений слева работают без Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } и настройки приложений слева работают без интернета.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Повторить

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Результаты

# Red words under the search box when no web browser opens a link.
store-no-browser = Нет браузера, чтобы открыть { $url }. Сначала установите его в «Приложениях».

# Said in the card or under the title while the lists are being read.
store-looking = Поиск…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Теперь оно в разделе «Установленные» и в кольце

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Удалено

# The word on an app's dial when installing or removing did not work.
store-done-not = Не выполнено

# The word on an app's dial when it was installed.
store-done-installed = Установлено

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak отказал.

# Tab of the Installed page that shows everything.
store-tab-all = Все

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Файлы Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Файлы deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = Плагины

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Пока ничего не установлено. В разделе «Обзор» много интересного.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] Установлен { $n } плагин
        [few] Установлено { $n } плагина
        [many] Установлено { $n } плагинов
       *[other] Установлено { $n } плагина
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] Установлена { $n } программа
        [few] Установлено { $n } программы
        [many] Установлено { $n } программ
       *[other] Установлено { $n } программы
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = Установлено объектов: { $n }

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, из Flathub
        [few] { $start }, все из Flathub
        [many] { $start }, все из Flathub
       *[other] { $start }, все из Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, из файла Flatpak
        [few] { $start }, все из файлов Flatpak
        [many] { $start }, все из файлов Flatpak
       *[other] { $start }, все из файлов Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, это AppImage
        [few] { $start }, все это AppImage
        [many] { $start }, все это AppImage
       *[other] { $start }, все это AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, из файла .deb
        [few] { $start }, все из файлов .deb
        [many] { $start }, все из файлов .deb
       *[other] { $start }, все из файлов .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = из Flathub: { $n }

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] из файлов Flatpak: { $n }
        [few] из файлов Flatpak: { $n }
        [many] из файлов Flatpak: { $n }
       *[other] из файлов Flatpak: { $n }
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] AppImage: { $n }
        [few] AppImage: { $n }
        [many] AppImage: { $n }
       *[other] AppImage: { $n }
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] { $n } приложение
        [few] { $n } приложения
        [many] { $n } приложений
       *[other] { $n } приложения
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] из файлов .deb: { $n }
        [few] из файлов .deb: { $n }
        [many] из файлов .deb: { $n }
       *[other] из файлов .deb: { $n }
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] { $n } плагин
        [few] { $n } плагина
        [many] { $n } плагинов
       *[other] { $n } плагина
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Обновлено.

# End of the Installed line when everything is up to date.
store-installed-state-all = Всё обновлено.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] { $n } ожидает обновления.
        [few] { $n } ожидают обновления.
        [many] { $n } ожидают обновления.
       *[other] { $n } ожидает обновления.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = Программ, подходящих под «{ $find }»: { $shown } из { $total }.

# The same when plugins are among the things.
store-installed-match-things = Объектов, подходящих под «{ $find }»: { $shown } из { $total }.

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Добавить AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = Поиск среди установленного

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = Не удалось добавить AppImage.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Здесь появятся приложения, которые вы установите из раздела «Обзор», с GitHub или как AppImage, программы из файлов .deb и плагины.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Среди установленного нет ничего, подходящего под «{ $find }».

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = Удаление спрашивает дважды: нажмите кнопку ещё раз в течение трёх секунд.

# On an installed thing's line: its version.
store-installed-version = версия { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = отложено

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = отложено; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = Обновляется { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Обновить { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = О приложении

# Under an installed thing's row while it is being taken off.
store-installed-removing = Удаление...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Нет шины сеанса, чтобы спросить, какой файл.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Выберите AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Не удалось обратиться к окну выбора файла.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Добавление...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } уже есть: см. «Установленные».

# Words once an AppImage is in the ring.
store-add-in-ring = Добавлено. Теперь это в вашем списке.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Это не ссылка Flatpak, которую может использовать этот компьютер.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = Устанавливается { $name } из { $host }...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } установлено.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak не стал это устанавливать.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = Не удалось установить { $name }. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Замена...

# Heading of the question when an AppImage has the name of one already there.
# SAFETY: needs a human review before it ships
store-replace-head = { $name } уже есть

# Small heading over the AppImage being brought.
store-replace-new = Добавляемый

# Small heading over the AppImage already there.
store-replace-old = Уже имеющийся

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = Пропустить: оставить уже имеющийся

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = Заменить

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = Удалить { $name }?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Его сохранённые данные занимают { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = Сохранённых данных нет.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Подсчёт сохранённых данных...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Удалённые данные восстановить нельзя.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Удалить, сохранив данные

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Удалить вместе с данными

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Удалить только данные

# Why Delete its data only is greyed.
store-flatpak-data-only-none = Нет сохранённых данных для удаления.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Отмена: оставить и то и другое

# Said under the card or the row once only the data has been deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-gone = Его сохранённые данные удалены.

# Words while only the data is being deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-going = Удаление сохранённых данных...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Работа не завершилась, причина не указана.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Показать, в чём проблема

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Поиск на GitHub: введите и нажмите Enter

# Heading of the card of matches from AppImageHub.
store-gh-hub = На AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Нажмите Enter, чтобы искать и по всему GitHub.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub не опрашивался: нет интернета.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Запрос к GitHub...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = Больше на GitHub ничего нет.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] Поиск файлов AppImage и Flatpak в { $n } проекте...
        [few] Поиск файлов AppImage и Flatpak в { $n } проектах...
        [many] Поиск файлов AppImage и Flatpak в { $n } проектах...
       *[other] Поиск файлов AppImage и Flatpak в { $n } проектах...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Ни у одного из найденных на GitHub проектов нет файла AppImage или Flatpak для этого компьютера.

# Heading of the card of matches from GitHub.
store-gh-on = На GitHub

# Heading of the card that holds the search box.
store-gh-search-card = Поиск на GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = Приложения на GitHub в виде файлов AppImage или Flatpak. Их никто не проверяет: устанавливайте только от тех, кому доверяете.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = Искать на GitHub нельзя, пока этот компьютер не подключён к интернету.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak и AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Показать файлы

# Button that installs a GitHub project or file.
# SAFETY: needs a human review before it ships
store-gh-install = Установить

# Link to a project's page on GitHub.
store-gh-open = Открыть на GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, выпуск { $tag }

# Name of the GitHub key's button and of its box.
# SAFETY: needs a human review before it ships
store-gh-key = Ключ GitHub

# Name of the GitHub key's button when a key is stored.
# SAFETY: needs a human review before it ships
store-gh-key-stored = Ключ GitHub: сохранён

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = Скорее всего, он вам не нужен. Без ключа GitHub отвечает этому компьютеру на 60 запросов в час, с ключом на 5000. Ключ только повышает этот предел. Он нужен лишь для просмотра общедоступных файлов и ни для чего больше: он не даёт Trindle доступа к вашей учётной записи, и при его создании не нужно отмечать никаких разрешений. Он хранится на этом компьютере, в вашей связке ключей, и отправляется только в GitHub.

# In the GitHub key's box: how to make a key.
# SAFETY: needs a human review before it ships
store-gh-key-how = Создайте ключ на GitHub, дайте ему любое имя, не меняйте ни одного разрешения, нажмите Generate token, а затем вставьте сюда показанное значение.

# Red words under the GitHub key's box when the key could not be removed.
# SAFETY: needs a human review before it ships
store-gh-key-not-removed = Не удалось удалить: { $why }

# Red words under the GitHub key's box when the key could not be kept.
# SAFETY: needs a human review before it ships
store-gh-key-not-kept = Не удалось сохранить: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
# SAFETY: needs a human review before it ships
store-gh-key-keep-empty = Оставьте пустым, чтобы сохранить текущий

# Placeholder of the GitHub key's box.
# SAFETY: needs a human review before it ships
store-gh-key-paste = Вставьте ключ

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = Удалить ключ

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = Создать ключ на GitHub

# Tip of the round button that keeps the GitHub key typed.
# SAFETY: needs a human review before it ships
store-gh-key-keep = Сохранить ключ

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = В его последнем выпуске нет ничего, что можно установить здесь.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Добавлено

# Line under the title of the Move your apps page.
store-bk-line = Ваши приложения и настройки в одном файле, который можно взять с собой.

# Heading of the card that makes a backup.
store-bk-card = Резервная копия

# Tip of the round button while the backup is being made.
store-bk-making-tip = Создаётся резервная копия

# Main button that makes the backup.
store-bk-make = Создать резервную копию

# Tip of the round button that shows the backup in Folders.
store-bk-show = Показать в «Папках»

# Name of the switch for the list of apps in a backup.
store-bk-apps = Приложения

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] Список вашего { $n } приложения Flatpak.
        [few] Список ваших { $n } приложений Flatpak.
        [many] Список ваших { $n } приложений Flatpak.
       *[other] Список ваших { $n } приложения Flatpak.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } Другой компьютер устанавливает их заново из Flathub, поэтому файл остаётся небольшим.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Собственные данные приложений

# Under the Apps' own data switch.
store-bk-data-sub = Настройки и сохранённая работа каждого приложения, без кэша.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = Сохраняются целиком, так как это отдельные файлы.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Настройки Trindle

# Under the Trindle settings switch.
store-bk-settings-sub = Кольцо, выбор обоев, цвета, клавиатура, питание, приложения по умолчанию и ваша картинка.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Сейчас { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Куда сохраняется

# Under that row: the folder and the file's name.
store-bk-where-sub = В вашу домашнюю папку, как { $name }

# Heading of the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-card = Восстановить из резервной копии

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Выбрать файл резервной копии

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Резервная копия Trindle или созданная в { $flat }.

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = Её приложения устанавливаются, а данные и настройки возвращаются. Ваших документов и изображений в ней нет: скопируйте их сами.

# Said while a backup is being made.
store-bk-making = Создание резервной копии... это может занять несколько минут.

# Said when a backup is made: the file and its size.
store-bk-made = Файл: { $path } ({ $size }). Скопируйте его на флешку или в облачное хранилище, чтобы взять с собой.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Резервная копия не выбрана, потому что у Trindle нет связи с окном выбора файла.

# Title of the window that asks which backup to use.
store-bk-choose-title = Выберите резервную копию

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Резервные копии Trindle и Flat

# Red words when the file chooser gave no answer.
store-bk-no-answer = Резервная копия не выбрана, потому что окно выбора файла не ответило.

# Said while a backup file is read.
store-bk-reading = Чтение резервной копии...

# Said when a backup was put back.
store-bk-logout = Выйдите из системы и войдите снова, чтобы увидеть все настройки.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Выйдите из системы и войдите снова, чтобы увидеть все настройки.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] { $apps } приложение, из них с настройками: { $with }. Запуск...
        [few] { $apps } приложения, из них с настройками: { $with }. Запуск...
        [many] { $apps } приложений, из них с настройками: { $with }. Запуск...
       *[other] { $apps } приложения, из них с настройками: { $with }. Запуск...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Источники производителей пока не добавлены.
        [one] Добавлен { $n } источник производителя, обновляется вместе с обновлениями Debian.
        [few] Добавлено { $n } источника производителей, обновляются вместе с обновлениями Debian.
        [many] Добавлено { $n } источников производителей, обновляются вместе с обновлениями Debian.
       *[other] Добавлено { $n } источника производителей, обновляются вместе с обновлениями Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Добавлен и обновляется.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. Источник добавлен, но программа не установлена.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Установлено другим способом.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = Удалить программу и её источник

# Link to a maker's page, naming the program.
store-src-open = Открыть { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Войти в Tailscale

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = Добавить { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = Можно добавить

# Name of the field for the address of a source.
store-src-address = Адрес

# Under the address field.
store-src-address-sub = Адрес репозитория из инструкции по установке от производителя.

# Name of the field for the suite of a source.
store-src-suite = Выпуск (suite)

# Under the suite field.
store-src-suite-sub = Обычно trixie или stable.

# Name of the field for the components of a source.
store-src-components = Компоненты

# Under the components field.
store-src-components-sub = Обычно main.

# Name of the field for a source's signing key.
# SAFETY: needs a human review before it ships
store-src-key = Ключ подписи

# Under the signing key field.
# SAFETY: needs a human review before it ships
store-src-key-sub = Адрес его ключа подписи.

# Name of the field for the program to install from a source.
store-src-program = Устанавливаемая программа

# Under the program field.
store-src-program-sub = Имя её пакета, если хотите установить её сейчас.

# Placeholder of the program field.
store-src-program-hint = необязательно

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = Добавить этот источник

# Heading of the card that adds another source.
# SAFETY: needs a human review before it ships
store-src-another = Добавить другой источник

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = Источник может устанавливать обновления для всего компьютера, поэтому добавляйте только тех производителей, которым доверяете. Для каждого запрашивается пароль администратора.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = Удалить этот источник

# Heading of the card of sources already on the computer.
store-src-already = Уже на этом компьютере

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Оба адреса должны начинаться с https://

# Red words when the suite or components are not plain words.
store-src-bad-words = Выпуск и компоненты должны быть обычными словами.

# Red words when the program is not a package name.
store-src-bad-package = Это не имя пакета.

# Word on a source's dial while its job starts.
store-src-starting = Запуск

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Открывается страница входа Tailscale...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Войдите на открывшейся странице; остальное завершится само.

# Said under Tailscale's row when it is connected.
store-src-connected = Подключено

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = Ссылка на приложения не зашифрована.

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = Не зашифровано

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = Этот адрес не зашифрован. Кто-то на пути может увидеть отправляемое и подменить резервную копию другой.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = Я понимаю и принимаю этот риск

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = Восстановить

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = Отмена

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = Не удалось подсчитать сохранённые данные приложения.

store-hub-warning = AppImageHub только перечисляет проекты: никто их не проверяет и не сканирует, и подписаны они быть не обязаны. Trindle запускает каждый AppImage в коробке с личной домашней папкой: сначала он может пользоваться интернетом и звуком и не видит ни одного вашего файла.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = Откуда взят AppImage, это подсказка, а не проверка.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = На GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = На GitHub: { $repo }, звёзд: { $stars }

# Where an AppImage comes from: a website, by its name.
store-from-site = Не на GitHub: с сайта { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Источник неизвестен

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Открыть его сайт

# When an app was last updated.
store-age-updated = Последнее обновление: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Последнее обновление: { $date }. Не обновлялся больше шести месяцев, поэтому в нём могут быть неисправленные уязвимости.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Последнее обновление: неизвестно

# When an app's project was made.
store-age-created = Создан { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Создан: неизвестно

# A date the source does not give, on a line of Facts.
store-age-not-known = Неизвестно

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Прежде чем добавить

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] В AppImageHub указан { $count } AppImage.
        [few] В AppImageHub указаны { $count } AppImage.
        [many] В AppImageHub указано { $count } AppImage.
       *[other] В AppImageHub указано { $count } AppImage.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Получение списка AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = Не удалось получить список AppImageHub.

# The same with no internet and no copy kept.
store-browse-offline = Список AppImageHub нельзя получить, пока этот компьютер не подключён к интернету.

# In the list when the kind chosen has none.
store-browse-none = В AppImageHub нет ни одного такого.

# Under the rows: how many are shown of how many.
store-browse-showing = Показано { $shown } из { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Добавить в приложения Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Подробности и файлы



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = проверка обновлений выключена

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT вернётся, когда OpenAI подпишет свою загрузку.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } на выбор из Flathub и AppImageHub; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } на выбор из AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Оба

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage не проверяются. Их никто не просматривает и не сканирует.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle запускает каждый AppImage в изолированной среде с отдельной домашней папкой: сначала ему доступны интернет и звук, а ваших файлов он не видит.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Больше не показывать

# Button on that card that goes on with the install.
store-appimage-warn-ok = ОК

# Button on that card that gives the install up.
store-appimage-warn-cancel = Отмена

