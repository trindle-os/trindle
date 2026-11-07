# machine translated (Russian), not yet reviewed by a person.

# Name of the Settings page about default apps, in the list and as its heading.
settings-default-apps-title = Приложения по умолчанию

# Words a person might type into the Settings search to find the default apps page; separate them with spaces.
settings-default-apps-keywords = браузер почта электронная почта файлы изображения музыка видео pdf календарь открыть в программе терминал

# Introduction under the heading of the default apps page, and the words behind the page i.
settings-default-apps-help = Какая программа открывает веб-ссылки, электронную почту, изображения, календарь и остальное. Терминал выбирается в ~/.config/xdg-terminals.list; если ничего не выбрано, Trindle использует первый из установленных.

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-web = Веб

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-mail = Почта

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-folders = Папки

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-text = Текст

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pictures = Изображения

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-music = Музыка

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-video = Видео

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pdf = Документы PDF

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-calendar = Календарь

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-web-in-sentence = веб-ссылки

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-mail-in-sentence = почта

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-folders-in-sentence = папки

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-text-in-sentence = текст

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pictures-in-sentence = изображения

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-music-in-sentence = музыка

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-video-in-sentence = видео

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pdf-in-sentence = документы PDF

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-calendar-in-sentence = календарь

# Title of a card on the default apps page.
settings-default-apps-card-internet = Интернет

# Words behind the i of a card on the default apps page.
settings-default-apps-card-internet-about = Программа, которая открывает веб-ссылку, где бы вы по ней ни щёлкнули, программа, которая пишет письмо, когда щёлкают по адресу, и программа, которая открывает приглашение в календарь.

# Title of a card on the default apps page.
settings-default-apps-card-files = Файлы и документы

# Words behind the i of a card on the default apps page.
settings-default-apps-card-files-about = Программа, которая открывает папку, простой текстовый файл и документ PDF, когда вы открываете их из «Папок», с рабочего стола или из другого приложения.

# Title of a card on the default apps page.
settings-default-apps-card-media = Изображения и звук

# Words behind the i of a card on the default apps page.
settings-default-apps-card-media-about = Программа, которая открывает изображение, музыку или видео, когда вы их открываете.

# Sentence at the top of the page when every kind has a program; { $web } is the program that opens web links.
settings-default-apps-all-set-web = Для всего выбрана программа; веб-ссылки открывает { $web }.

# Sentence at the top of the page when every kind has a program and the one for web links is not known.
settings-default-apps-all-set = Для всего выбрана программа.

# Sentence at the top of the page when one kind has no program chosen; { $kind } is its name in lower case.
settings-default-apps-one-missing = Пока не выбрано, чем открывать: { $kind }.

# Sentence at the top of the page when several kinds have no program chosen; { $kinds } is their names as a list joined with "or".
settings-default-apps-many-missing = Пока не выбрано, чем открывать: { $kinds }.

# Small words under a kind that no installed program can open.
settings-default-apps-none-installed = Пока ничего из установленного не может это открыть.

# First choice in a list when no program is chosen for that kind yet.
settings-default-apps-not-chosen = Пока не выбрано

# Small words under a kind with a program list; { $n } is how many programs can open it.
settings-default-apps-can-open =
    { $n ->
        [one] Единственная программа, которая может.
        [few] Открыть это могут программы: { $n }.
        [many] Открыть это могут программы: { $n }.
       *[other] Открыть это могут программы: { $n }.
    }

# Said in green under a kind's list after the choice was kept.
settings-default-apps-saved = Сохранено
