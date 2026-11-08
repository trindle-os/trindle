# machine translated (Russian), not yet reviewed by a person.

# Name of the AI programs page, in the page list, as its heading and on the card listing the programs.
settings-ai-title = Программы ИИ

# Words people may search for to find the AI programs page, separated by spaces.
settings-ai-keywords = ии программы claude codex cursor gemini aider агент изоляция песочница личные папки доступ ssh ключи токен sudo

# Help line at the foot of the AI programs page.
settings-ai-help = Программы ИИ для написания кода работают в изоляции, скрывающей ваши личные файлы. Открывайте им доступ только к нужным папкам.

# Headline of the AI programs page.
settings-ai-note = Это защищает ваши личные файлы от программ ИИ в повседневной работе, например если одну из них обманом заставят собрать ваши банковские выписки и отправить их наружу. Защита ограниченная и не даёт гарантии: настойчивая программа, ошибка в программном обеспечении компьютера или ваше согласие на запрос либо общий доступ к папке всё же могут что-то пропустить.

# Note under the headline of the AI programs page.
settings-ai-inside = Всё, что запускает программа ИИ, например установленные программы и сценарии, остаётся вместе с ней внутри изоляции.

# Red words when a change to the fence could not be made.
settings-ai-not-changed = Не удалось изменить.

# Said beside a setting on the AI programs page once it is saved.
settings-ai-saved = Сохранено

# Title of the window for choosing a folder to share with AI programs.
# SAFETY: needs a human review before it ships
settings-ai-choose-folder = Выберите папку для общего доступа с программами ИИ

# Red words when the key for AI programs to reach computers by ssh could not be made.
# SAFETY: needs a human review before it ships
settings-ai-key-not-made = Не удалось создать ключ.

# Label of the switch that limits the network of AI programs, and heading of its line.
# SAFETY: needs a human review before it ships
settings-ai-limit-net = Ограничить сеть перечисленными здесь сайтами

# Under the Limit the network heading.
settings-ai-limit-net-sub = Выкл.: сеть открыта. Вкл.: только сайты ниже.

# Tip of the button that removes a site from the list AI programs may reach.
# SAFETY: needs a human review before it ships
settings-ai-stop-site = Запретить программам ИИ доступ к этому сайту

# Example in the box for typing a site AI programs may reach.
settings-ai-eg-site = например, api.anthropic.com

# Tip of the button that adds a site AI programs may reach.
# SAFETY: needs a human review before it ships
settings-ai-let-site = Разрешить программам ИИ доступ к этому сайту

# Heading of the line for adding a site.
settings-ai-add-site = Добавить сайт

# Under the Add a site heading.
settings-ai-add-site-sub = Его имя или *.example.com для всех имён под ним.

# Heading of the card that limits the network of AI programs.
settings-ai-network = Сеть

# Help text of the Network card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-network-about = Для опытных. Когда выключено, программы ИИ пользуются сетью так же, как вы. Когда включено, они могут обращаться только к сайтам ниже по защищённым соединениям (https), а у изоляции нет собственной сети. Инструменты программы должны использовать выданные им настройки прокси. Добавьте сайт службы ИИ и источника пакетов, нужного для установки.

# Label of the switch that keeps AI programs inside the fence, and heading of its line.
# SAFETY: needs a human review before it ships
settings-ai-keep-inside = Держать программы ИИ в изоляции

# Under the fence switch when it is on.
settings-ai-hides-home = Скрывает вашу домашнюю папку, кроме папок, открытых ниже.

# Under the fence switch when it is off.
# SAFETY: needs a human review before it ships
settings-ai-fence-off = Изоляция выключена. Программы ИИ теперь видят всё, что видите вы.

# Heading of the card with the fence switch.
settings-ai-the-fence = Изоляция

# Help text of The fence card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-fence-about = Система остаётся видимой и доступной для программ ИИ. Вместо вашей домашней папки у них пустая личная домашняя папка, которая сохраняется между запусками, и в ней только папки, к которым вы открыли доступ. Они не видят ваши другие программы, ваш сеанс, ваши ключи и пароли. Программа, запущенная не по имени из этого списка, не изолируется. Если вы установили подключение ассистента (trindle-mcp), подключённый ассистент может читать без вашего запроса следующее: названия ваших открытых окон и приложения, которым они принадлежат, установленные приложения, работающие программы и объём занимаемой ими памяти, состояние памяти, диска, батареи, времени работы и сети, ваши настройки Trindle, до 500 строк системного журнала любой службы, ваши последние записи о сбоях, а также названия ваших звуковых устройств, сопряжённых устройств Bluetooth, ближайших сетей Wi-Fi и дисков. Прежде чем что-либо изменить, он всегда спрашивает вас.

# Label of the switch that fences one AI program: its name is { $name }.
# SAFETY: needs a human review before it ships
settings-ai-fence-name = Изолировать { $name }

# Tip of the button that takes one program off the list: its name is { $name }.
settings-ai-take-off = Убрать { $name } из списка

# Label of the switch for using a program's existing settings folder: its name is { $name }.
settings-ai-own-for = Использовать мою существующую папку настроек для { $name }

# Heading of the line with the switch for using a program's existing settings folder.
settings-ai-own = Использовать мою существующую папку настроек

# Under the existing settings folder heading.
settings-ai-own-sub = В ней хранятся прошлые разговоры этой программы.

# Example in the box for typing the name of an AI program.
settings-ai-eg-program = например, claude

# Tip of the button that adds an AI program, and heading of its line.
settings-ai-add-program = Добавить программу

# Under the Add a program heading.
settings-ai-add-program-sub = Введите имя, по которому она запускается.

# Help text of the AI programs card (the information button).
settings-ai-programs-about = Эти имена запускаются внутри изоляции. «Использовать мою существующую папку настроек» выключено: программа начинает с новой личной папки. Программы входят в свои службы сами, внутри: ваш собственный вход им не передаётся.

# Heading of the line shown when no folder is shared with AI programs.
settings-ai-nothing-shared = Ничего не открыто

# Under the Nothing is shared heading.
settings-ai-none-seen = Программы ИИ не видят ни одной из ваших папок.

# Label of the switch that lets AI programs only read a shared folder.
# SAFETY: needs a human review before it ships
settings-ai-only-read = Разрешить только чтение этой папки

# Tip of the button that stops sharing a folder with AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-private = Снова сделать эту папку личной

# Under a shared folder's path, beside the switch that makes it read only.
# SAFETY: needs a human review before it ships
settings-ai-only-read-short = Только чтение

# Tip of the button that shares a folder, and heading of its line.
# SAFETY: needs a human review before it ships
settings-ai-share-folder = Открыть доступ к папке

# Under the Share a folder heading.
# SAFETY: needs a human review before it ships
settings-ai-share-folder-sub = Видна по тому же пути, для чтения и записи.

# Heading of the card listing folders shared with AI programs.
# SAFETY: needs a human review before it ships
settings-ai-shared = Доступно программам ИИ

# Help text of the shared folders card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-shared-about = Всё остальное в вашей домашней папке по правилу остаётся закрытым от программ ИИ. Скрытые папки и папки с ключами, паролями или почтой открыть нельзя никогда. Открытая здесь папка будет доступна при следующем запуске программы.

# Tip of the button that removes a computer from the list AI programs may reach by ssh.
# SAFETY: needs a human review before it ships
settings-ai-stop-computer = Запретить программам ИИ доступ к этому компьютеру

# Example in the box for typing a computer AI programs may reach by ssh.
settings-ai-eg-computer = например, nas.local

# Tip of the button that adds a computer AI programs may reach by ssh.
# SAFETY: needs a human review before it ships
settings-ai-let-computer = Разрешить программам ИИ доступ к этому компьютеру

# Heading of the line for adding a computer.
settings-ai-add-computer = Добавить компьютер

# Under the Add a computer heading.
settings-ai-add-computer-sub = Введите его имя.

# Note showing the public key to put on each computer: the key is { $key }.
settings-ai-put-key = Поместите этот ключ на каждый компьютер: { $key }

# Tip of the button that makes a key for AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-key = Создать ключ

# Heading of the line for making a key for AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-key-for = Создать ключ для программ ИИ

# Under the Make a key heading.
settings-ai-make-key-sub = Отдельный ключ, хранящийся вне папок, которые они видят.

# Heading of the card listing computers AI programs may reach by ssh.
# SAFETY: needs a human review before it ships
settings-ai-ssh = Компьютеры, доступные им по ssh

# Help text of the ssh card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-ssh-about = Внутри изоляции ssh обращается к Trindle, который использует собственный ключ и только для перечисленных здесь компьютеров. Ваши ключи программам ИИ никогда не показываются.

# Tip of the button that stops giving AI programs a key.
# SAFETY: needs a human review before it ships
settings-ai-stop-key = Больше не выдавать программам ИИ этот ключ

# Under the name of a key that AI programs are given.
# SAFETY: needs a human review before it ships
settings-ai-given = Выдан программам ИИ

# Example in the box for typing a key to give AI programs: a name, an equals sign and the value.
settings-ai-eg-key = ИМЯ=значение

# Tip of the button that gives AI programs a key.
# SAFETY: needs a human review before it ships
settings-ai-give-key = Выдать программам ИИ этот ключ

# Heading of the line for adding a key.
# SAFETY: needs a human review before it ships
settings-ai-add-key = Добавить ключ

# Under the Add a key heading.
settings-ai-add-key-sub = Записывается как ИМЯ=значение.

# Heading of the card listing keys given to AI programs.
# SAFETY: needs a human review before it ships
settings-ai-keys = Ключи, выданные программам ИИ

# Help text of the keys card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-keys-about = Ключи и токены вашего сеанса никогда не попадают к программам ИИ. Ключ, введённый здесь, им выдаётся, и только здесь.
