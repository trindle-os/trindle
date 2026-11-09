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
settings-blocking-waiting = Ожидание пароля...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Блокировка включена

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Блокировка выключена

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Сохранено

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Введите имя сайта, например example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Блокировка выключена. Запросы имён идут на серверы вашей сети, как и раньше.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Блокировка запускается.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Реклама и опасные сайты заблокированы, а запросы имён идут приватно на { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = В этой сети не удаётся связаться с приватным сервером, поэтому пока используются серверы вашей сети. Блокировка по-прежнему работает.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Реклама и опасные сайты заблокированы. Запросы имён идут на серверы вашей сети, и они их видят.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Реклама и опасные сайты заблокированы. Запросы имён идут через VPN на серверы вашего VPN.

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
settings-blocking-tile-fetching = Загрузка

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Пока нет

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = списки сайтов

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = сайтов в списках, загружено { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Пока ничего не заблокировано

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Сайты появляются здесь, новые сверху.

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Всегда пропускать этот сайт

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Недавно заблокированные

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Сайты появляются здесь по мере блокировки, новые сверху. Плюс пропускает сайт с этого момента. Здесь показаны только заблокированные сайты. Пропущенные сайты не сохраняются.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Включите блокировку, чтобы увидеть, что она блокирует.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = например, example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Добавить сайт

# Small line under "Add a site".
settings-blocking-add-sub = Он и все его поддомены никогда не блокируются.

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = Снова блокировать этот сайт

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = Всегда пропускать

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Перечисленные здесь сайты и все их поддомены никогда не блокируются, даже если они есть в списках. Минус снова блокирует сайт.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Блокировка

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = блокировка реклама рекламы трекеры вредоносные мошенники dns приватные запросы quad9 mullvad cloudflare разрешить

# Help text at the top of the page.
settings-blocking-help = Блокирует рекламу, трекеры и известные опасные сайты для каждого браузера и приложения на этом компьютере и отправляет запросы имён по зашифрованному соединению, чтобы сеть, в которой вы находитесь, не могла их прочитать или изменить.

# Text of the switch that turns blocking on or off.
# SAFETY: needs a human review before it ships
settings-blocking-switch = Блокировать рекламу и опасные сайты

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Куда идут запросы

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Используются списки HaGeZi с рекламой, трекерами и известными опасными сайтами; они загружаются раз в день. Пока блокировка выключена, ничего не работает. Запросы имён идут по зашифрованному соединению на выбранный здесь приватный сервер. Quad9 не хранит сведений о том, кто спрашивал. Вариант «Ваша сеть» тоже блокирует, но не приватен: серверы вашей сети видят каждый запрос.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = Списки HaGeZi, загружаются раз в день.

# Heading of the line with the private server dropdown.
settings-blocking-private = Приватный сервер

# Small line under Private server.
settings-blocking-private-sub = Quad9 не хранит сведений о том, кто спрашивал.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Ваша сеть
