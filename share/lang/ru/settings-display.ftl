# machine translated (Russian), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Экран

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = экран монитор разрешение размер масштаб увеличить уменьшить повернуть частота обновления ночной свет тёплый синий свет вечер глаза display

# Introduction under the heading of the display page, and the words behind the page i.
# SAFETY: needs a human review before it ships
settings-display-help = Изменения применяются сразу. Если с экраном что-то пошло не так, ничего не делайте. Через 15 секунд всё вернётся само, если не нажать «Оставить». Ночной свет делает экран теплее вечером.

# Question along the bottom of the page while a change of displays waits to be kept.
# SAFETY: needs a human review before it ships
settings-display-foot-keep = Оставить такое расположение?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Проверка экранов…

# Sentence at the top of the page when no display was found.
settings-display-none-found = Экран не найден.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = Ни один экран не используется

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Используется один экран

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] Используется { $n } экран
        [few] Используется { $n } экрана
        [many] Используется { $n } экранов
       *[other] Используется { $n } экрана
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = ночной свет выключен

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = ночной свет делает экран теплее с { $from } до { $to }

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = ночной свет делает экран теплее от заката до рассвета

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Обычно

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Повёрнут вправо

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Повёрнут влево

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = Вверх ногами

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = Справа от

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = Слева от

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Выше

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Ниже

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height }, { $hz } Гц

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (рекомендуется)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Гц

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (стандартный)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Разрешение экрана { $display }

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Размер на экране { $display }

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Поворот экрана { $display }

# Heading of the resolution line.
settings-display-resolution = Разрешение

# Heading of the size line.
settings-display-size = Размер

# Heading of the rotation line.
settings-display-rotation = Поворот

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Использовать этот экран

# Small words under that line for the only display in use.
settings-display-use-only = Единственный используемый экран нельзя выключить.

# Words behind the i of a display's card.
# SAFETY: needs a human review before it ships
settings-display-card-about = Изменения применяются сразу. Если с экраном что-то пошло не так, ничего не делайте. Через 15 секунд всё вернётся само, если не нажать «Оставить». Рекомендуемое разрешение: то, для которого сделан экран. Стандартный размер: тот, который Trindle выбрал для этого экрана.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Расположение экрана { $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Расположите его

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } относительно { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Расположение

# Words behind the i of that card.
settings-display-arrangement-about = Задайте, как экраны стоят рядом, чтобы указатель мог переходить с одного на другой через общий край.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Чтобы поставить один экран рядом с другим, нужно два экрана.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Не удалось это изменить.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
        [one] Экраны изменились. Через { $seconds } секунду они вернутся, если не нажать «Оставить».
        [few] Экраны изменились. Через { $seconds } секунды они вернутся, если не нажать «Оставить».
        [many] Экраны изменились. Через { $seconds } секунд они вернутся, если не нажать «Оставить».
       *[other] Экраны изменились. Через { $seconds } секунды они вернутся, если не нажать «Оставить».
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Не удалось отменить изменение. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = Расположение сейчас действует, но Trindle не смог сохранить его до следующего входа.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Ничего не изменено

# Small words under that line.
settings-display-refused-change = Не удалось внести это изменение.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Изменения применяются сразу

# Title of the night light card.
settings-display-night-title = Ночной свет

# Words behind the i of that card.
settings-display-night-about = Вечером экран становится теплее, что легче для глаз. Он следует за закатом и рассветом, вычисленными по вашему часовому поясу, или за временем, которое вы задали сами. Если Trindle не может определить по часовому поясу, где вы находитесь, используется заданное вами время.

# Heading of the night light line, and name of its switch.
settings-display-night = Ночной свет

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Закат и рассвет в поясе { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle использует заданное время, потому что не знает, когда закат.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = От заката до рассвета

# Choice in the When list: between times the person sets.
settings-display-when-times = Заданное время

# Heading of the When line, and name of its list.
settings-display-when = Когда

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = С

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = До

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Теплота

# Small word under the left end of the warmth slider.
settings-display-little = Слегка

# Small word under the right end of the warmth slider.
settings-display-very = Очень тёплый

# Spoken description of the strip of colours night light runs through.
settings-display-strip = От слегка тёплого до очень тёплого

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Ни один экран не используется

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] { $n } экран
        [few] { $n } экрана, как они расположены
        [many] { $n } экранов, как они расположены
       *[other] { $n } экрана, как они расположены
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Нет экрана

# Name of the large picture of the displays.
settings-display-your-displays = Ваши экраны

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle не нашёл ни одного экрана.

# Title of the card shown when no display was found.
settings-display-none-title = Экраны не найдены

# Heading of the line in that card.
settings-display-none-heading = Экраны не найдены

# Title of the card about the graphics card.
settings-display-graphics-title = Графика

# Words behind the i of that card.
settings-display-graphics-about = В этом компьютере установлена видеокарта NVIDIA с открытым драйвером. Она работает, но карта примерно с 2022 года будет работать медленно, потому что открытый драйвер не может повысить её скорость. Вместо него можно установить собственный драйвер NVIDIA.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Карта NVIDIA с открытым драйвером

# Small words under that line.
settings-display-nvidia-about = Карты с 2022 года и новее могут работать медленно.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Что с этим делать
