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
settings-ai-limit-net = Разрешить только сайты ниже

# Tip of the button that removes a site from the list AI programs may reach.
# SAFETY: needs a human review before it ships
settings-ai-stop-site = Запретить программам ИИ доступ к этому сайту

# Example in the box for typing a site AI programs may reach.
settings-ai-eg-site = api.anthropic.com

# Tip of the button that adds a site AI programs may reach.
# SAFETY: needs a human review before it ships
settings-ai-let-site = Разрешить программам ИИ доступ к этому сайту

# Heading of the line for adding a site.
settings-ai-add-site = Добавить сайт

# Under the Add a site heading.
settings-ai-add-site-sub = Введите его имя или *.example.com для всех сайтов под ним.

# Heading of the card that limits the network of AI programs.
settings-ai-network = Сеть

# Help text of the Network card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-network-about = Когда выключено, программы ИИ пользуются сетью так же, как вы. Когда включено, они могут обращаться только к сайтам ниже по защищённым соединениям (https), а у изоляции нет собственной сети. Инструменты программы должны использовать выданные им настройки прокси. Добавьте сайт службы ИИ и источник пакетов, нужный для установки.

# Label of the switch that keeps AI programs inside the fence, and heading of its line.
# SAFETY: needs a human review before it ships
settings-ai-keep-inside = Держать программы ИИ в изоляции

# Under the fence switch when it is off.
# SAFETY: needs a human review before it ships
settings-ai-fence-off = Изоляция выключена. Программы ИИ теперь видят всё, что видите вы.

# Heading of the card with the fence switch.
settings-ai-the-fence = Изоляция

# Help text of The fence card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-fence-about = Программы ИИ по-прежнему видят систему и могут ею пользоваться. Вместо вашей домашней папки у них пустая личная, которая сохраняется между запусками. В ней только папки, которыми вы поделились. Другие ваши программы, сеанс, ключи и пароли они не видят. Программа, запущенная иначе, не по имени из этого списка, не ограждена.

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
settings-ai-eg-program = claude

# Tip of the button that adds an AI program, and heading of its line.
settings-ai-add-program = Добавить программу

# Help text of the AI programs card (the information button).
settings-ai-programs-about = Эти программы запускаются внутри изоляции. Если «Использовать мою существующую папку настроек» выключено, программа начинает с новой личной папки. Каждая программа входит в свою службу сама, внутри изоляции. Ваш собственный вход им не передаётся.

# Heading of the line shown when no folder is shared with AI programs.
settings-ai-nothing-shared = Ничего не открыто

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
settings-ai-share-folder-sub = Видна по тому же пути. Программы ИИ могут читать её и записывать в неё.

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
settings-ai-eg-computer = nas.local

# Tip of the button that adds a computer AI programs may reach by ssh.
# SAFETY: needs a human review before it ships
settings-ai-let-computer = Разрешить программам ИИ доступ к этому компьютеру

# Heading of the line for adding a computer.
settings-ai-add-computer = Добавить компьютер

# Note showing the public key to put on each computer: the key is { $key }.
settings-ai-put-key = Поместите этот ключ на каждый компьютер: { $key }

# Tip of the button that makes a key for AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-key = Создать ключ

# Heading of the line for making a key for AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-key-for = Создать ключ для программ ИИ

# Under the Make a key heading.
settings-ai-make-key-sub = Отдельный ключ, хранящийся вне папок, которые видят программы ИИ.

# Heading of the card listing computers AI programs may reach by ssh.
# SAFETY: needs a human review before it ships
settings-ai-ssh = Компьютеры, к которым они могут подключаться по ssh

# Help text of the ssh card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-ssh-about = Внутри изоляции ssh подключается через Trindle. Trindle использует собственный ключ и только для перечисленных здесь компьютеров. Программы ИИ никогда не видят ваши ключи.

# Tip of the button that stops giving AI programs a key.
# SAFETY: needs a human review before it ships
settings-ai-stop-key = Больше не выдавать программам ИИ этот ключ

# Example in the box for typing a key to give AI programs: a name, an equals sign and the value.
settings-ai-eg-key = ИМЯ=значение

# Tip of the button that gives AI programs a key.
# SAFETY: needs a human review before it ships
settings-ai-give-key = Выдать программам ИИ этот ключ

# Heading of the line for adding a key.
# SAFETY: needs a human review before it ships
settings-ai-add-key = Добавить ключ

# Heading of the card listing keys given to AI programs.
# SAFETY: needs a human review before it ships
settings-ai-keys = Ключи, выданные программам ИИ

# Help text of the keys card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-keys-about = Ключи и токены вашего сеанса никогда не попадают к программам ИИ. Им передаётся только ключ, который вы вводите здесь.
