# machine translated (Russian), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Добро пожаловать в Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Откройте кольцо

# Title of the third page, where the first apps are picked.
welcome-page-apps = Выберите приложения

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Ещё на выбор

# Title of the fifth page, about plugins.
welcome-page-plugins = Плагины добавляют возможности

# Title of the sixth page, about crash reports.
welcome-page-reports = Помогите улучшить Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Точки

# Title of the last page, with a few things worth knowing.
welcome-page-know = Полезно знать

# Sentence under the title of the first page.
welcome-sentence-hello = Всё нужное находится в одном круге, в кольце.

# Sentence under the title of the second page.
welcome-sentence-ring = Дважды щёлкните у самого края экрана или нажмите клавишу Super.

# Sentence under the title of the third page.
welcome-sentence-apps = Пока почти ничего не установлено, поэтому выберите нужные приложения.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Выберите ещё, затем установите всё выбранное одним нажатием.

# Sentence under the title of the fifth page.
# SAFETY: needs a human review before it ships
welcome-sentence-plugins = Плагины - это небольшие дополнения из «Приложений», и каждое делает только то, что вы разрешите.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Если хотите, отправляйте отчёт, когда что-то пошло не так.

# Sentence under the title of the dots page.
welcome-sentence-dots = Точка означает, что программа использует что-то ваше.

# Sentence under the title of the last page.
welcome-sentence-know = Несколько вещей, которыми Trindle отличается.

# Top right of every page: where the person is in the tour.
welcome-step = Шаг { $n } из { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Шаг { $n } из { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
# SAFETY: needs a human review before it ships
welcome-done-file-note = Приветствие уже показано. Оно есть среди всех приложений под названием «Приветствие»; удалите этот файл, чтобы увидеть его снова при следующем входе.

# Name of the round button at the foot that closes the tour.
welcome-close = Закрыть

# Name of the round button at the foot that goes to the page before.
welcome-back = Назад

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Далее

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Установить выбранные приложения

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Начать работу с Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = Подключитесь к интернету, чтобы устанавливать приложения.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Повторить

# Foot while apps are going in: their names.
welcome-installing = Установка: { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Установка: { $names }, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } теперь в кольце.
        [few] { $names } теперь в кольце.
        [many] { $names } теперь в кольце.
       *[other] { $names } теперь в кольце.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Не удалось установить приложения. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak отказал.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Выбрано: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } и ещё { $more }

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = Кольцо

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = Что в нём

# Name of the first thing the ring holds.
welcome-held-programs = Ваши программы

# Line under it.
welcome-held-programs-line = Расположены по внешнему краю.

# Name of the second thing the ring holds.
welcome-held-time = Время и дата

# Line under it.
welcome-held-time-line = Расположены посередине.

# Name of the third thing the ring holds.
welcome-held-switches = Переключатели

# Line under it.
welcome-held-switches-line = Звук, экран, не засыпать и питание.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Указателем

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = С клавиатуры

# Note under the picture of the Super key.
welcome-ring-super-note = На клавише Super часто нарисован логотип Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = Веб-браузер

# Head of a card of apps: email programs.
welcome-kind-email = Электронная почта

# Head of a card of apps: chat programs.
welcome-kind-chat = Чат

# Head of a card of apps: office programs.
welcome-kind-office = Офис

# Head of a card of apps: music and video players.
welcome-kind-media = Музыка и видео

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Изображения и рисование

# Head of a card of apps: notes and writing.
welcome-kind-notes = Заметки и тексты

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF и документы

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Сейчас ни одного из них нет на Flathub.

# Beside an app that has been installed.
welcome-app-installed = Установлено

# Beside an app that is going in now.
welcome-app-installing = Установка

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, выбрано

# Name of the round button on an app's row that picks it.
welcome-app-pick = Выбрать { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = В кольце

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Плагины

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = В «Приложениях», раздел «Плагины».

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Открыть плагины в «Приложениях»

# Said in the plugins card while there are none to show.
welcome-plugins-none = Доступные плагины перечислены в «Приложениях».

# Head of the card on the crash reports page.
welcome-reports-card = Отчёты о сбоях

# The one question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-question = Помочь улучшить Trindle, отправляя отчёт, когда что-то пошло не так? Вы всегда увидите его первым.

# Note under the question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-note = Ничего не отправляется без вашего нажатия. Вы видите весь отчёт заранее, в нём нет файлов, паролей и имён сетей Wi-Fi.

# Second note under the question about crash reports.
welcome-reports-settings = Это можно изменить в любое время в «Настройках», в разделе «Отчёты о сбоях».

# The answer that turns crash reports off.
# SAFETY: needs a human review before it ships
welcome-reports-no = Нет

# The answer that turns crash reports on.
# SAFETY: needs a human review before it ships
welcome-reports-yes = Да, спрашивать меня

# Name the screen reader gives to the choice between the two answers.
# SAFETY: needs a human review before it ships
welcome-reports-choice = Отправлять отчёт, когда что-то пошло не так

# Head of a card on the last page.
welcome-know-boxes = Приложения живут в своих коробках

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = Приложение спрашивает разрешения, прежде чем получить доступ к вашим файлам, камере или микрофону.

# Head of a card on the last page.
welcome-know-updates = Обновления безопасности устанавливаются сами

# Line in that card.
welcome-know-updates-line = Каждый день, без вопросов. Остальные обновления ждут вас.

# Head of a card on the last page.
welcome-know-ring = «Настройки» и «Приложения» находятся в кольце

# Line in that card.
welcome-know-ring-line = В его середине, над временем.

# Head of a card on the last page.
welcome-know-help = Справка находится в «Настройках»

# Line in that card.
welcome-know-help-line = Этот обзор есть среди всех приложений под названием «Приветствие», когда он снова понадобится.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Небольшой экран, на котором есть только кольцо посередине: время в центре, избранные приложения вокруг.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = Открытое кольцо на небольшом экране; в его середине под временем в ряд значки нескольких плагинов, там, где появляется значок плагина после установки.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Два маленьких рисунка с местами, где показываются точки: на вкладке окна в кольце и вверху по центру экрана.

welcome-dots-where-tab = На вкладке окна

welcome-dots-where-corner = Вверху по центру экрана

welcome-picture-ring = Небольшой экран. Указатель перемещается к самому верхнему краю и дважды щёлкает, и кольцо открывается посередине.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = Нажатие клавиши Super, которая открывает кольцо.

# Title of the first page, which asks how much the person knows.
welcome-page-level = Насколько вы знакомы с системой?

# Sentence under that title.
welcome-sentence-level = Выберите ближайший вариант, и обзор пропустит то, что вы уже знаете.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Новичок в Linux

# Line in that answer's card.
welcome-level-new-line = Все страницы, включая то, что делает любой рабочий стол Linux.

# Name of the answer for someone who has used Linux.
welcome-level-linux = Уже работал(а) в Linux

# Line in that answer's card.
welcome-level-linux-line = Только то, что относится к самой Trindle. Основы Linux пропускаются.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Уже работал(а) в Trindle

# Line in that answer's card.
welcome-level-trindle-line = Закрывает обзор сейчас и больше его не показывает.

# Title of the page about files and folders.
welcome-page-files = Ваши файлы

# Sentence under that title.
welcome-sentence-files = Всё ваше хранится в домашней папке.

# Title of the page about getting software.
welcome-page-software = Установка программ

# Sentence under that title.
welcome-sentence-software = Приложения берутся из раздела «Приложения», а не из скачанных установочных файлов.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = Пароль администратора

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Изменения, затрагивающие весь компьютер, сначала запрашивают пароль.

# Title of the page about snapshots.
welcome-page-snapshots = Снимки

# Sentence under that title.
welcome-sentence-snapshots = Trindle хранит копии системы, поэтому обновление можно отменить.

# Title of the page about the person's own words.
welcome-page-words = Ваши собственные слова

# Sentence under that title.
welcome-sentence-words = Измените любое слово в Trindle или добавьте свой язык.

# Head of a card on the files page.
welcome-files-home = Домашняя папка

# Line in that card.
welcome-files-home-line = Документы, загрузки и изображения находятся внутри неё. «Папки» открываются там.

# Head of a card on the files page.
welcome-files-drives = Без букв дисков

# Line in that card.
welcome-files-drives-line = USB-накопитель или другой диск появляется в «Папках», когда вы его подключаете.

# Head of a card on the files page.
welcome-files-hidden = Скрытые имена

# Line in that card.
welcome-files-hidden-line = Имя, начинающееся с точки, например .config, скрыто. В нём хранятся настройки приложения.

# Head of a card on the files page.
welcome-files-bin = Корзина

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = Удалённый файл лежит в корзине, пока вы её не очистите.

# Head of a card on the software page.
welcome-software-apps = Одно место для приложений

# Line in that card.
welcome-software-apps-line = Откройте «Приложения» в круге, чтобы найти, установить и удалить программы.

# Head of a card on the software page.
welcome-software-exe = Без установочных файлов

# Line in that card.
welcome-software-exe-line = Установочный файл Windows с расширением .exe здесь не запускается. Ищите приложение в разделе «Приложения».

# Head of a card on the software page.
welcome-software-updates = Обновления в одном месте

# Line in that card.
welcome-software-updates-line = «Приложения» и «Настройки» поддерживают всё в актуальном состоянии, поэтому отдельным приложениям свои обновления не нужны.

# Head of a card on the software page.
welcome-software-remove = Чистое удаление

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Удаление приложения убирает его целиком и ничего не оставляет.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = Зачем он нужен

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Установка для всех или изменение диска затрагивает каждого пользователя, поэтому компьютер сначала спрашивает.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = Чей пароль

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = Пароль администратора, обычно того, под кем вы входите.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = Когда он не нужен

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Приложениям, которые вы ставите для себя, и вашим файлам он не нужен никогда.

# Head of a card on the snapshots page.
welcome-snap-taken = Создаются сами

# Line in that card.
welcome-snap-taken-line = В системе с Btrfs снимок создаётся до и после каждого изменения программ.

# Head of a card on the snapshots page.
welcome-snap-list = Список в настройках

# Line in that card.
welcome-snap-list-line = «Настройки», «Снимки» показывают их список. Кнопка с камерой создаёт снимок сразу.

# Head of a card on the snapshots page.
welcome-snap-boot = Помогают, если система не запускается

# Line in that card.
welcome-snap-boot-line = Выберите «Снимки Trindle» в меню загрузки, затем нужный снимок.

# Head of a card on the your own words page.
welcome-words-change = Изменить слово

# Line in that card.
welcome-words-change-line = Найдите любое слово в Trindle и введите своё. «Открыть, пожалуйста» может заменить «Нажмите здесь, чтобы открыть файл».

# Head of a card on the your own words page.
welcome-words-share = Поделиться набором

# Line in that card.
welcome-words-share-line = Экспортируйте свои слова в файл или импортируйте файл от друга. Перед использованием он проверяется.

# Head of a card on the your own words page.
welcome-words-all = Все программы сразу

# Line in that card.
welcome-words-all-line = Вся Trindle берёт слова из одного места, поэтому одно изменение видно везде.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Это находится в разделе «Настройки», «Регион и язык». «Справка» объясняет всё по шагам.

# Name of the round button that opens that Settings page.
welcome-words-open = Открыть «Регион и язык»
