# machine translated (Russian), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle не может прочитать { $file } в корне папки ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = В { $file } нет раздела [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Эта версия Trindle поддерживает Interface={ $known }, а не Interface={ $n }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = В [Plugin] нужно указать Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = В [Plugin] нужен Id из букв, цифр, точек и дефисов, например org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = В [Plugin] нужен Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = В [Plugin] нужен Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = Summary в [Plugin] должен быть в одну строку.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key }: нужен обычный текст в одну строку, не более { $max } символа.
        [few] [{ $group }] { $key }: нужен обычный текст в одну строку, не более { $max } символов.
        [many] [{ $group }] { $key }: нужен обычный текст в одну строку, не более { $max } символов.
       *[other] [{ $group }] { $key }: нужен обычный текст в одну строку, не более { $max } символа.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
# SAFETY: needs a human review before it ships
plugins-check-permission-unknown = Разрешения «{ $name }» не существует. Существуют: { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = В [Ring] нужен Icon: файл SVG или PNG в папке плагина.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = Icon в [Ring] указывает на { $icon }, но этого файла нет в папке плагина.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = В [Ring] нужен Label, он показывается, когда на значке задерживается указатель.

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = В [Settings] нужен Fields: имена его полей.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Имя поля «{ $name }» может содержать только буквы, цифры, дефисы и подчёркивания.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = У поля «{ $name }» нет раздела [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = В [{ $group }] нужен Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices: допускается не более { $most } вариантов. Каждый вариант - обычный текст, не более { $chars } символа.
        [few] [{ $group }] Choices: допускается не более { $most } вариантов. Каждый вариант - обычный текст, не более { $chars } символов.
        [many] [{ $group }] Choices: допускается не более { $most } вариантов. Каждый вариант - обычный текст, не более { $chars } символов.
       *[other] [{ $group }] Choices: допускается не более { $most } вариантов. Каждый вариант - обычный текст, не более { $chars } символа.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom должен быть именем файла в папке плагина. Допустимы только буквы, цифры, точки, дефисы и подчёркивания. Имя не должно начинаться с точки и не должно содержать путь к папке.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] - выбор, поэтому нужен Choices с двумя или больше вариантами, либо ChoicesFrom с файлом.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default не входит в Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] - заметка, поэтому ей нужен TextFrom с именем файла в папке плагина. Допустимы только буквы, цифры, точки, дефисы и подчёркивания. Имя не должно начинаться с точки и не должно содержать путь к папке.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] - заметка, она не хранит настройку, поэтому Default для неё указать нельзя.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind равен { $kind }, а должен быть switch, text, choice или note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] - переключатель, поэтому его Default должен быть true или false.

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = Для [Background] нужно разрешение background в Permissions раздела [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = Плагин ни к чему не подключается: добавьте [Ring], [Settings], [Background], [Shortcut] или разрешение notifications.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
# SAFETY: needs a human review before it ships
plugins-links-outside = Ссылка { $path } ведёт за пределы папки плагина.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run не удаётся прочитать как команду.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run должен указывать на программу внутри папки плагина, а не на { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run указывает на { $program }, но такой программы в плагине нет (она отсутствует или не исполняемая).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] В Menu раздела [Ring] { $n } пункт. Допускается не более { $max }.
        [few] В Menu раздела [Ring] { $n } пункта. Допускается не более { $max }.
        [many] В Menu раздела [Ring] { $n } пунктов. Допускается не более { $max }.
       *[other] В Menu раздела [Ring] { $n } пункта. Допускается не более { $max }.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Id пункта меню «{ $id }» может содержать только буквы, цифры, дефисы и подчёркивания.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = Menu раздела [Ring] называет «{ $id }» дважды.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = Menu раздела [Ring] называет «{ $id }», но у него нет раздела [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label - одна короткая строка, не более { $max } символа.
        [few] [{ $group }] Label - одна короткая строка, не более { $max } символов.
        [many] [{ $group }] Label - одна короткая строка, не более { $max } символов.
       *[other] [{ $group }] Label - одна короткая строка, не более { $max } символа.
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-only-flow = [{ $group }] допустим только для Flow из каталога Trindle, которому нужно разрешение control.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-needs-run = В [{ $group }] нужен Run: команда, которой передаётся PIN-код.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen - не более { $max } символа.
        [few] [{ $group }] ShowWhen - не более { $max } символов.
        [many] [{ $group }] ShowWhen - не более { $max } символов.
       *[other] [{ $group }] ShowWhen - не более { $max } символа.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen нужно записать как поле=значение, например ShowWhen=mode=home. Укажите другое поле и значение, которое оно должно иметь.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen записывается как поле=значение, а имя поля может содержать только буквы, цифры, дефисы и подчёркивания.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] Для ShowWhen нужно обычное текстовое значение в одну строку, не более { $max } символа.
        [few] [{ $group }] Для ShowWhen нужно обычное текстовое значение в одну строку, не более { $max } символов.
        [many] [{ $group }] Для ShowWhen нужно обычное текстовое значение в одну строку, не более { $max } символов.
       *[other] [{ $group }] Для ShowWhen нужно обычное текстовое значение в одну строку, не более { $max } символа.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen указывает на собственное поле. Нужно указать другое, более раннее поле.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen указывает на поле { $other }, которого нет в Fields этого плагина.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen указывает на поле { $other }, которое в Fields идёт позже. Нужно указать более раннее поле.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen указывает на поле { $other }, у которого есть свой ShowWhen. Поле может зависеть только от поля без ShowWhen.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen указывает на переключатель { $other }, поэтому значение должно быть true или false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] У ShowWhen значение { $value }, которого нет среди Choices поля { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen ссылается на заметку { $other }, которая не хранит настройку. Используйте вместо неё переключатель, выбор или текстовое поле.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
# SAFETY: needs a human review before it ships
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = Для [Network] Listen нужно разрешение network в Permissions раздела [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = Для [Network] Listen нужно разрешение listen в Permissions раздела [Plugin].

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = Для разрешения listen в [Network] нужен Listen: список портов, на которых плагин может принимать подключения.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
# SAFETY: needs a human review before it ships
plugins-ports-interface = В его plugin.ini указан интерфейс, который эта версия Trindle не поддерживает.

# Red words when the ports to open for a plugin are asked and it names none.
# SAFETY: needs a human review before it ships
plugins-ports-none = Он не указывает портов для приёма подключений.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
# SAFETY: needs a human review before it ships
plugins-ports-unreadable = Не удалось прочитать его plugin.ini.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = Ему не разрешено принимать подключения.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = Список портов изменился. Нужно снова разрешить подключения на них.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none =
    { $n ->
        [1] Для работы «{ $name }» нужно ваше разрешение. Вы не дали единственного разрешения, которое он запрашивает.
        [one] Для работы «{ $name }» нужно ваше разрешение. Вы пока ничего не разрешили. Запрошено разрешений: { $n }.
        [few] Для работы «{ $name }» нужно ваше разрешение. Вы пока ничего не разрешили. Запрошено разрешений: { $n }.
        [many] Для работы «{ $name }» нужно ваше разрешение. Вы пока ничего не разрешили. Запрошено разрешений: { $n }.
       *[other] Для работы «{ $name }» нужно ваше разрешение. Вы пока ничего не разрешили. Запрошено разрешений: { $n }.
    }

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = Вы разрешили только часть того, что запрашивает «{ $name }»: { $a } из { $n }.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] Он ничего не запрашивает.
        [one] Разрешено всё, что он запрашивает ({ $n }).
        [few] Разрешено всё, что он запрашивает ({ $n }).
        [many] Разрешено всё, что он запрашивает ({ $n }).
       *[other] Разрешено всё, что он запрашивает ({ $n }).
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = Разрешить

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
# SAFETY: needs a human review before it ships
plugins-other-ports = подключения к нему на других портах

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Не установлен.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = другой плагин

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Этот плагин не установлен.

# Red words when a permission could not be kept. { $why } is the reason.
# SAFETY: needs a human review before it ships
plugins-not-saved = Не удалось сохранить: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Не удалось скопировать файлы плагина.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Плагин не работает в фоновом режиме: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = не удалось найти trindle-plugin, чтобы запустить её.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] она не запустилась за { $n } секунду.
        [few] она не запустилась за { $n } секунды.
        [many] она не запустилась за { $n } секунд.
       *[other] она не запустилась за { $n } секунды.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = нет запущенного сеанса рабочего стола, к которому можно присоединиться, поэтому она не запущена. Сначала войдите в систему.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = не удаётся определить, к какому сеансу рабочего стола присоединиться ({ $sessions }), поэтому она не запущена. Запустите её снова, указав в WAYLAND_DISPLAY нужный сеанс.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = на месте файла находится папка

# Error text when a plugin's id is not an id.
plugins-not-an-id = это недопустимый идентификатор

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] В Shortcuts раздела [Plugin] { $n } сочетание. Допускается не более { $max }.
        [few] В Shortcuts раздела [Plugin] { $n } сочетания. Допускается не более { $max }.
        [many] В Shortcuts раздела [Plugin] { $n } сочетаний. Допускается не более { $max }.
       *[other] В Shortcuts раздела [Plugin] { $n } сочетания. Допускается не более { $max }.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = Id сочетания «{ $id }» может содержать только буквы, цифры, дефисы и подчёркивания.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts раздела [Plugin] называет «{ $id }» дважды.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts раздела [Plugin] называет «{ $id }», но у него нет раздела [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } - уже одно из сочетаний клавиш Trindle. Выберите другие клавиши.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } уже используются другим сочетанием этого плагина.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = В [{ $group }] нужен Keys, например Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Это сочетание клавиш недопустимо. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } - уже одно из сочетаний клавиш Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } - уже одно из ваших собственных сочетаний.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } уже используется: { $holder }.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Это не сочетание установленного плагина.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle не смогла его использовать ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Trindle не смогла сохранить это, потому что { $why }. Ничего не изменилось.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Не удалось сохранить: { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = Его порты не удалось закрыть: { $why }.

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle не смог открыть порты, поэтому приём подключений снова выключен: { $why }.

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = Его порты не открылись ({ $why }), и приём подключений тоже не удалось выключить ({ $kept }). Ничего не открыто. Выключите его в «Настройках», в разделе «Разрешения».


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Разрешением на специальные возможности может пользоваться только программа чтения с экрана Trindle из каталога.

# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = Разрешение { $name } указано дважды.

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = Запрашивайте { $home } или network, но не оба сразу: network даёт доступ и к локальной сети, и к интернету.

# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = В [{ $group }] нужен Run, команда для запуска.

# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } Старую версию тоже не удалось вернуть на место: { $back } Обе версии сохранены в папке плагина.
