# machine translated (Russian), not yet reviewed by a person.

# Name of the Region and language page, in the page list and as its heading.
settings-region-title = Регион и язык

# Words people may search for to find the Region and language page, separated by spaces.
settings-region-keywords = язык локаль страна регион форматы дата валюта деньги

# Help line at the foot of the Region and language page.
settings-region-help = Язык и форматы вступят в силу при следующем входе в систему.

# Said beside the Language and Formats lists once a choice is saved.
settings-region-later = Выйдите и войдите снова, чтобы увидеть результат.

# Shown while the page reads the computer.
settings-region-looking = Поиск…

# Under Money and paper: the paper and measures the chosen formats use (paper is a4, letter or other; measures is imperial or metric).
settings-region-paper-measures =
    { $paper ->
        [a4] бумага A4
        [letter] бумага Letter
       *[other] бумага
    }, { $measures ->
        [imperial] дюймы и фунты
       *[other] метрическая система мер
    }

# Under Money and paper: { $money } is the money sign and { $paper } the paper and measures.
settings-region-money-and-paper = { $money }, { $paper }

# Under A date and a number: a date and a number written the way the formats chosen write them.
settings-region-date-and-number = { $date } и { $number }

# Said beside the Add a language line once a language is installed.
settings-region-installed-done = Установлен

# Headline when the language and the formats are the same: { $lang } is the language's name.
settings-region-everything-in = Всё на языке: { $lang }.

# Headline when the formats differ from the language: the names are { $lang } and { $fmt }.
settings-region-menus-in = Меню на языке «{ $lang }»; даты и числа записываются по правилам «{ $fmt }».

# Headline when a change is saved but waits for the next login: { $words } is the sentence so far.
settings-region-with-wait = { $words } Выйдите и войдите снова, чтобы увидеть изменение.

# Said beside the Install another line while a language is made.
settings-region-installing = Установка

# Name of the list of languages for menus and messages, heading of its line and of the card.
settings-region-language = Язык

# Name of the list of formats and heading of its line.
settings-region-formats = Форматы

# Help text of the Language card (the information button).
settings-region-language-about = Язык - это язык меню и сообщений. Форматы определяют, как записываются даты, время, числа, деньги, единицы измерения и размер бумаги; они могут соответствовать обычаям другой страны. И то и другое вступает в силу при следующем входе.

# Under the Language heading.
settings-region-language-sub = Меню и сообщения, со следующего входа.

# Under the Formats heading.
settings-region-formats-sub = Даты, числа, деньги и размер бумаги.

# Heading of the line that shows a date and a number in the chosen formats.
settings-region-date-number = Дата и число

# Heading of the line that shows the money and paper of the chosen formats.
settings-region-money-paper = Деньги и бумага

# Heading of the line that shows the language now in use.
settings-region-in-use = Используется сейчас

# Heading of the card for installing another language.
settings-region-add = Добавить язык

# Help text of the Add a language card (the information button).
settings-region-add-about = Устанавливает ещё один язык, чтобы его можно было выбрать для меню или форматов. Это занимает некоторое время и требует пароля администратора.

# Heading of the line shown when every language is already installed.
settings-region-all-installed = Все языки уже установлены

# Heading of the line saying how many languages are installed.
settings-region-installed = Установлено

# Under the Installed heading: how many languages are installed.
settings-region-count =
    { $n ->
        [one] Установлен { $n } язык.
        [few] Установлено { $n } языка.
        [many] Установлено { $n } языков.
       *[other] Установлено { $n } языка.
    }

# Name of the list of languages that can be installed, and heading of its line.
settings-region-install-another = Установить ещё

# Tip of the button that installs the chosen language.
# SAFETY: needs a human review before it ships
settings-region-install-this = Установить этот язык (запросит пароль)

# Under the Install another heading.
# SAFETY: needs a human review before it ships
settings-region-needs-password = Нужен пароль администратора.

# Heading of the line saying how many more languages can be made.
settings-region-ready = Можно добавить

# Under the Ready to add heading: how many more languages can be made.
settings-region-more =
    { $n ->
        [one] Этот компьютер может создать ещё { $n } язык.
        [few] Этот компьютер может создать ещё { $n } языка.
        [many] Этот компьютер может создать ещё { $n } языков.
       *[other] Этот компьютер может создать ещё { $n } языка.
    }

settings-region-removing = Удаление

settings-region-removed-done = Удалено

settings-region-remove-language = Удалить этот язык

settings-region-kept = Языки на этом компьютере

settings-region-kept-about = Все установленные здесь языки. Удалите ненужные; тот, которым вы пользуетесь, останется. Запрашивает пароль администратора.

settings-region-in-use-stays = Используется, поэтому остаётся

