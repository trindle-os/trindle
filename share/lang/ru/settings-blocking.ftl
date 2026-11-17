# machine translated (Russian), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = только что

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] { $n } минуту назад
        [few] { $n } минуты назад
        [many] { $n } минут назад
       *[other] { $n } минуты назад
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] { $n } час назад
        [few] { $n } часа назад
        [many] { $n } часов назад
       *[other] { $n } часа назад
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] { $n } день назад
        [few] { $n } дня назад
        [many] { $n } дней назад
       *[other] { $n } дня назад
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = Ожидание пароля…

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Блокировка включена

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Блокировка выключена

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Сохранено

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Введите имя сайта, например example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Блокировка выключена. Адреса сайтов запрашиваются у серверов вашей сети, как и раньше.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Блокировка запускается.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Реклама и опасные сайты заблокированы, а адреса сайтов запрашиваются приватно через { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Trindle не может связаться с приватным сервером в этой сети, поэтому пока использует серверы вашей сети. Блокировка по-прежнему работает.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Реклама и опасные сайты заблокированы. Адреса сайтов запрашиваются у серверов вашей сети, и они видят каждый сайт, который вы посещаете.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Реклама и опасные сайты заблокированы. Адреса сайтов запрашиваются у серверов вашего VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Реклама и опасные сайты заблокированы.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Запуск

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Выкл.

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = блокировка запускается

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = блокировка выключена

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Заблокировано сегодня

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = запросов заблокировано сегодня

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Списки

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Обновление

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Пока нет

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = списки сайтов

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = сайтов в списках, обновлено { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Пока ничего не заблокировано

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Всегда пропускать этот сайт

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Недавно заблокированные

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Сайты появляются здесь по мере блокировки, новые сверху. Нажмите плюс, чтобы с этого момента пропускать сайт. Trindle не хранит записей о пропущенных сайтах.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Включите блокировку, чтобы увидеть, что она блокирует.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Добавить сайт

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = Снова блокировать этот сайт

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = Всегда пропускать

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Эти сайты и все адреса под ними никогда не блокируются, даже если они есть в списках. Нажмите минус, чтобы снова блокировать сайт.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Блокировка

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = блокировка реклама рекламы трекеры вредоносные мошенники dns приватные запросы quad9 mullvad cloudflare разрешить

# Help text at the top of the page.
settings-blocking-help = Блокирует рекламу, трекеры и известные опасные сайты для каждого браузера и приложения на этом компьютере. Кроме того, адреса сайтов запрашиваются по зашифрованному соединению, поэтому сеть, в которой вы находитесь, не может увидеть или изменить, какие сайты вы открываете.

# Text of the switch that turns blocking on or off.
# SAFETY: needs a human review before it ships
settings-blocking-switch = Блокировать рекламу и опасные сайты

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Куда идут запросы

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Используются списки HaGeZi с рекламой, трекерами и известными опасными сайтами. Они обновляются раз в день. Пока блокировка выключена, ничего не работает. Адреса сайтов запрашиваются по зашифрованному соединению через выбранный здесь приватный сервер. Quad9 не хранит сведений о том, кто спрашивал. Вариант «Ваша сеть» тоже блокирует, но не приватен, потому что серверы вашей сети видят каждый сайт, который вы запрашиваете.

# Heading of the line with the private server dropdown.
settings-blocking-private = Приватный сервер

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Ваша сеть

# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = Брандмауэр

# Help text (i button) of the Firewall card.
settings-firewall-about = Когда брандмауэр включён, другие компьютеры не могут подключиться к этому, если соединение не начал сам этот компьютер. Поэтому никто в той же сети Wi-Fi не сможет попытаться проникнуть. Сайты, обновления и принтеры продолжают работать. Программе, которая ждёт подключений от других компьютеров, например игровому серверу, нужно открыть порт ниже. Когда брандмауэр выключен, любой компьютер в сети может добраться до каждой программы, ждущей подключений. Чтобы включить или выключить его, нужен пароль администратора.

# Name of the firewall's on and off switch.
settings-firewall-switch = Брандмауэр

# Small line under the switch while the firewall is off.
settings-firewall-off-sub = Другие компьютеры могут подключаться к программам, которые ждут подключений.

# Said beside the switch once the firewall is on.
settings-firewall-done-on = Брандмауэр включён

# Said beside the switch once the firewall is off.
settings-firewall-done-off = Брандмауэр выключен

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = Блокировка



# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = Брандмауэр включён, но не работает, потому что его правила не загрузились при запуске компьютера. Выключите и включите снова.
