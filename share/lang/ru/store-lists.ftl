# machine translated (Russian), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Файл Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = Файл .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = Плагин

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = релиз-кандидат

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = Удаляется { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Устанавливается { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = Открыть { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = Установить { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Установлено

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Показать

# How long is left when it is a minute or less.
store-left-minute = осталась около минуты

# How long is left, in minutes.
store-left-minutes = осталось около { $n } мин.

# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] остался около { $n } часа
        [few] осталось около { $n } часов
        [many] осталось около { $n } часов
       *[other] осталось около { $n } часа
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Проверено по списку Threat Intelligence от HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Загружено с сайта { $site }, переход со страницы { $referrer }.

# Where a download came from: the site.
store-origin-site = Загружено с сайта { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Переход со страницы { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle не может определить, откуда это взято.

# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } Сайт { $name } есть в списке Threat Intelligence от HaGeZi, куда входят сайты с вредоносными загрузками и мошенничеством. Устанавливайте это, только если уверены в источнике.

# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } Сайта нет в списке Threat Intelligence от HaGeZi, но список неполный, поэтому это не доказывает безопасность.

# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } Сайт не удалось проверить: список Threat Intelligence от HaGeZi не загрузился.

# Name shown for a review whose writer gave none.
store-review-someone = Кто-то

# Red words when the keyring could not be reached.
store-key-unreached = Не удалось обратиться к связке ключей: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = Сохранённый ключ не является текстом.

# Red words when a file that held the key could not be removed.
store-key-not-removed = Не удалось удалить { $path }: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = Не удалось прочитать { $path }: { $why }
