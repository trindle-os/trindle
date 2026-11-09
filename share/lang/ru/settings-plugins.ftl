# machine translated (Russian), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Ваши плагины

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = плагины дополнения расширения установленные удалить настройки

# Help line at the foot of the Your plugins page.
settings-plugins-help = Плагины расширяют Trindle: значок в кольце, настройки здесь, уведомления или то, что запускается при входе. Каждый работает в изоляции и ничего не может, пока вы не разрешите это в разделе «Разрешения», который открывает кнопка в конце заголовка этой страницы. В карточке каждого плагина здесь его собственные настройки; корзина на карточке, нажатая дважды, удаляет плагин.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Не удалось открыть окно, где спрашивается, что плагину разрешено.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Версия { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Версия { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = «{ $name }» ничего не запрашивает, поэтому работает в изоляции, и ему ничего не разрешено.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [one] «{ $name }» запрашивает: { $list }; разрешено { $allowed } из { $asks }. Он работает в изоляции и только с тем, что вы разрешили.
        [few] «{ $name }» запрашивает: { $list }; разрешено { $allowed } из { $asks }. Он работает в изоляции и только с тем, что вы разрешили.
        [many] «{ $name }» запрашивает: { $list }; разрешено { $allowed } из { $asks }. Он работает в изоляции и только с тем, что вы разрешили.
       *[other] «{ $name }» запрашивает: { $list }; разрешено { $allowed } из { $asks }. Он работает в изоляции и только с тем, что вы разрешили.
    }

# Under the line that tells a plugin needs more allowed, beside its Allow button.
# SAFETY: needs a human review before it ships
settings-plugins-needs-more = Кнопка «Разрешить» показывает каждый запрос с переключателем.

# Heading of the line about a plugin on its card.
settings-plugins-about-it = О плагине

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = Изменить, что ему разрешено

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = Удалить этот плагин

# Red words when a plugin could not be removed.
settings-plugins-not-removed = Не удалось удалить.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Введите здесь

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = У «{ $name }» нет собственных настроек.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = У «{ $name }» сейчас нет настроек для показа.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] Плагины не установлены.
        [one] Установлено плагинов: { $n } ({ $list }).
        [few] Установлено плагинов: { $n } ({ $list }).
        [many] Установлено плагинов: { $n } ({ $list }).
       *[other] Установлено плагинов: { $n } ({ $list }).
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Плагины

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Ничего не установлено

# Under the None installed heading.
settings-plugins-none-sub = Плагины можно добавить в разделе «Приложения».

# Tip of the button at the end of the page's title that opens the Permissions page.
# SAFETY: needs a human review before it ships
settings-plugins-open-permissions = Открыть «Разрешения»

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = Спрашиваем { $name }.

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } не отвечает. Возможно, его служба остановлена.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Сохранено.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } не принял это.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } не смог это сделать.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } не смог это сделать: { $why }.

