# machine translated (Russian), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Клавиатура и мышь

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = раскладка язык ввод набор caps lock экранная клавиатура кнопка кольцо мышь тачпад сенсорная панель указатель скорость касание щелчок прокрутка колесо медленнее быстрее естественная левша модуль плагин сочетания клавиши переназначить заменить поменять ctrl escape super alt mac боковые кнопки нажатие удержание созданные программами keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Всё здесь меняется сразу. Вторую раскладку можно включить, нажав Alt и Shift одновременно.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Ввод: { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt и Shift переключают на: { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock ничего не делает.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock работает как { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = второй Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = На этом компьютере нет списка раскладок клавиатуры.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Клавиатура

# Heading of the line in that card.
settings-keyboard-no-list-heading = Список раскладок клавиатуры недоступен

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Чтобы установить его, выполните: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Вы изменили одну клавишу или кнопку.
        [few] Вы изменили { $n } клавиши и кнопки.
        [many] Вы изменили { $n } клавиш и кнопок.
       *[other] Вы изменили { $n } клавиши и кнопки.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Раскладка

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Раскладка

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Вариант

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Стандартный

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = Нет

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Переключать на

# Words behind the i of the layout card.
settings-keyboard-layout-about = Здесь задаётся, что вводит каждая клавиша. Можно выбрать раскладку и её вариант. Нажмите Alt и Shift одновременно, чтобы включить вторую раскладку, и ещё раз, чтобы вернуться. Изменения применяются сразу.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = Сейчас используется

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Второй Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Ничего

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Использовать Caps Lock как

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Кнопка клавиатуры в кольце

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Другие клавиши

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock может работать как другая клавиша, например как второй Ctrl или Escape. Кнопка клавиатуры находится в середине кольца и открывает клавиатуру на экране. С её помощью можно печатать мышью или на сенсорном экране. Она появится при следующем открытии кольца.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Программа не может это изменить.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Нажмите клавишу или кнопку мыши, которую нужно изменить.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Вы ничего не нажали, поэтому ничего не добавлено.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Введите клавишу, которую нужно использовать вместо неё, например Escape или Ctrl+C, или введите Ничего.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle не удалось определить, какая это клавиша. Выберите клавишу из списка.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Все клавиатуры и мыши

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Вы сами изменили эту клавишу ниже, и действует это изменение.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Изменить { $key } на устройстве

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = Заменить { $key } на

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Убрать это изменение

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = Строка { $n } пропущена: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Не удалось прочитать изменённые клавиши

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Ваши клавиши работают как обычно, пока Trindle не сможет прочитать изменения. { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Выберите клавишу или кнопку для изменения

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Нажмите клавишу или кнопку для изменения

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Изменить клавишу или кнопку

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = Вы уже изменили { $key }.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Клавиши и кнопки

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Любая клавиша или кнопка мыши может делать что-то другое. Она может стать другой клавишей или кнопкой, сочетанием клавиш, например Ctrl+C, или последовательностью клавиш, например H, I. Она также может не делать ничего или делать одно при нажатии и другое при удержании, например Escape при нажатии и Ctrl при удержании. Это работает везде, начиная с экрана входа. Ctrl, Alt и клавиша F всегда открывают текстовую консоль, и команда trindle-settings --keys default, введённая там, возвращает все клавиши обратно.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Поменять местами Ctrl и Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: Escape при нажатии, Ctrl при удержании

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Поменять местами Escape и Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Поменять местами Alt и Super, как на клавиатуре Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Боковые кнопки мыши копируют и вставляют

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
settings-keyboard-made-switch = Доверять { $name }

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Пока нет

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Клавиатуры и мыши, созданные программами

# SAFETY: needs a human review before it ships
# Words behind the i of that card.
settings-keyboard-made-about = Средства переназначения клавиш и игровые лаунчеры могут создавать собственную клавиатуру или мышь. Их клавиши и щелчки по-прежнему могут печатать и играть, но не могут нажимать защищённые кнопки, например «Выключить» или «Разрешить», и отвечать на вопрос, предназначенный для вас. Включите переключатель здесь, чтобы устройство считалось вашим.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Контроллер { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } на { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = Что делает { $title }

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Поменять местами A и B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Поменять местами X и Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Поменять местами вверх и вниз на правом стике

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Сочетания клавиш плагинов

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } не используется: { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Клавиши для: { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Сохранено

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = И для тачпада тоже.

# Title of the mouse card.
settings-keyboard-mouse-title = Мышь

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = Скорость указателя определяет, как далеко он смещается при движении мыши. Она действует и для тачпада. Скорость прокрутки определяет, как далеко сдвигается страница за каждый поворот колеса. Выберите «Медленнее», «Обычная» или «Быстрее» либо задайте любое значение на ползунке. При естественной прокрутке страница движется в ту же сторону, что и колесо, как в телефоне. Режим левши меняет местами левую и правую кнопки.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Скорость указателя

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Скорость прокрутки

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Естественная прокрутка

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Режим левши

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Помощь при щелчках

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Щелчки от устройства с задержкой наведения или переключателя считаются вашими.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Тачпад

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = Скорость прокрутки определяет, как далеко сдвигается страница при прокрутке двумя пальцами. Касание вместо щелчка делает лёгкое касание щелчком. При естественной прокрутке страница движется в ту же сторону, что и пальцы, как в телефоне. Отключение при наборе не даёт ладони двигать указатель во время набора текста.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Касание вместо щелчка

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Отключать при наборе

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Медленнее

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Обычная

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Быстрее

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB-порты

# Words behind the i of that card.
settings-keyboard-usb-about = Если USB-порт не запускается, Trindle один раз выключает и снова включает его, чтобы повторить попытку. Для одного порта это делается не более одного раза.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle также не удалось отменить изменение: { $undo }


# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Снимки экрана, отчёты и быстрые клавиши

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Сделать снимок экрана

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Показать список быстрых клавиш

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Сделать снимок экрана сейчас

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Показать список сейчас

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Перезапуск рабочего стола

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Перезапустить кольцо и вспомогательные программы

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Перезапустить всё

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Клавиши для «{ $label }»

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Перезапустить сейчас. Открытые окна остаются как есть.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Спросить, затем перезапустить всё

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Перезапущено

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } уже используется другой быстрой клавишей.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Не сохранено. { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Сообщить о проблеме

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Сообщить о проблеме сейчас

# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = Не сохранено. Файл с изменёнными клавишами не удалось прочитать, поэтому он остался как есть.
