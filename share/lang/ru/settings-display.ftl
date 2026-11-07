# machine translated (Russian), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Экран

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = экран монитор разрешение размер масштаб увеличить уменьшить повернуть частота обновления ночной свет тёплый синий свет вечер глаза display

# Introduction under the heading of the display page, and the words behind the page i.
# SAFETY: needs a human review before it ships
settings-display-help = Изменения применяются сразу. Если с экраном что-то пошло не так, ничего не делайте: через 15 секунд всё вернётся само, если не нажать «Оставить». Ночной свет делает экран теплее вечером.

# Question along the bottom of the page while a change of displays waits to be kept.
# SAFETY: needs a human review before it ships
settings-display-foot-keep = Оставить такое расположение?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Осмотр экранов…

# Sentence at the top of the page when no display was found.
settings-display-none-found = Экраны не найдены.

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

# Small words under that line.
settings-display-resolution-about = Сколько точек он показывает и как часто.

# Heading of the size line.
settings-display-size = Размер

# Small words under that line.
settings-display-size-about = Насколько крупно всё выглядит на этом экране.

# Heading of the rotation line.
settings-display-rotation = Поворот

# Small words under that line.
settings-display-rotation-about = Для экрана, поставленного на бок.

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Использовать этот экран

# Small words under that line for the only display in use.
settings-display-use-only = Единственный используемый экран нельзя выключить.

# Small words under that line otherwise.
settings-display-use-off = Выключен, на нём ничего не показывается.

# Words behind the i of a display's card.
# SAFETY: needs a human review before it ships
settings-display-card-about = Изменения применяются сразу. Если с экраном что-то пошло не так, ничего не делайте: через 15 секунд всё вернётся само, если не нажать «Оставить». Рекомендуемое разрешение родное для экрана; стандартный размер выбран Trindle для этого экрана.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Расположение экрана { $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Расположите его

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } относительно { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Расположение

# Words behind the i of that card.
settings-display-arrangement-about = Как экраны стоят рядом, чтобы указатель переходил с одного на другой через общий край.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Включите два экрана, чтобы расположить один рядом с другим.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Рабочий стол отказал.

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
settings-display-not-kept = Расположение сейчас действует, но его не удалось сохранить до следующего входа.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Ничего не изменено

# Small words under that line.
settings-display-refused-change = Рабочий стол отказал в изменении.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Изменения применяются сразу

# Small words under that line; { $seconds } is how long Keep is asked for.
settings-display-keep-asked =
    { $seconds ->
        [one] Вопрос «Оставить?» внизу окна задаётся { $seconds } секунду.
        [few] Вопрос «Оставить?» внизу окна задаётся { $seconds } секунды.
        [many] Вопрос «Оставить?» внизу окна задаётся { $seconds } секунд.
       *[other] Вопрос «Оставить?» внизу окна задаётся { $seconds } секунды.
    }

# Title of the night light card.
settings-display-night-title = Ночной свет

# Words behind the i of that card.
settings-display-night-about = Вечером экран становится теплее, что легче для глаз. Он следует за закатом и рассветом, вычисленными по часовому поясу, или за вашим временем; если часовой пояс не показывает, где находится компьютер, используется заданное время.

# Heading of the night light line, and name of its switch.
settings-display-night = Ночной свет

# Small words under that line.
settings-display-night-about-line = Более тёплые цвета вечером.

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Закат и рассвет в поясе { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Часовой пояс не указывает места: заданное время.

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

# Small words under the warmth line.
settings-display-warmth-about = Насколько тёплым экран становится ночью.

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
        [one] { $n } экран, рядом, как они расположены
        [few] { $n } экрана, рядом, как они расположены
        [many] { $n } экранов, рядом, как они расположены
       *[other] { $n } экрана, рядом, как они расположены
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Нет экрана

# Name of the large picture of the displays.
settings-display-your-displays = Ваши экраны

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Рабочий стол не перечислил ни одного экрана.

# Title of the card shown when no display was found.
settings-display-none-title = Экраны не найдены

# Heading of the line in that card.
settings-display-none-heading = Нечего показывать

# Title of the card about the graphics card.
settings-display-graphics-title = Графика

# Words behind the i of that card.
settings-display-graphics-about = В этом компьютере карта NVIDIA с открытым драйвером. Всё работает. На карте примерно с 2022 года он будет работать медленно, потому что открытый драйвер не может поднять скорость карты. Можно установить собственный драйвер NVIDIA.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Карта NVIDIA с открытым драйвером

# Small words under that line.
settings-display-nvidia-about = Работает; карта с 2022 года работает медленно.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Что с этим делать

