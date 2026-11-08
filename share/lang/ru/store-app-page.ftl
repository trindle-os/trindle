# machine translated (Russian), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.

# Under the app's name when only the maker is known.
store-app-line-by = Автор: { $by }.

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, автор: { $by }.

# The state beside the dot while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-state-removing = Удаление

# The state beside the dot while the app is being fetched.
store-app-state-downloading = Загрузка

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = Установка

# The state beside the dot while the app's own update goes in.
store-app-state-updating = Обновление

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = Установлено, обновления приостановлены

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = Установлено, обновления приостановлены на версии { $version }

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = Установлено, { $waits }

# The same, with the installed version.
store-app-state-waiting-version = Установлено, версия { $version }, { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = Установлено, обновлено

# The same, with the installed version.
store-app-state-current-version = Установлено, версия { $version }, обновлено

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = Не установлено

# The same, with the size of the download.
store-app-state-absent-size = Не установлено, загрузка { $size }

# Word beside the spinner while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-busy-removing = Удаление

# Word beside the spinner while the app is being put in.
store-app-busy-installing = Установка

# Tip and spoken name of the round button that gives up a download.
# SAFETY: needs a human review before it ships
store-app-stop = Остановить

# Tip and spoken name of the round button that takes the app off.
# SAFETY: needs a human review before it ships
store-app-remove = Удалить

# Round button beside Update or Release that opens the app.
store-app-open-round = Открыть

# Main button that releases an app whose updates are held back.
store-app-release = Возобновить

# Main button that updates the app.
store-app-update = Обновить

# Main button that opens the app.
store-app-open = Открыть

# Main button that installs the app.
store-app-install = Установить

# Button that opens the app's website.
store-app-website = Сайт

# In the picture frame when the app has no pictures.
store-app-no-pictures = У этого приложения нет изображений.

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = Не удалось загрузить это изображение.

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = Загрузка изображения…

# In the picture frame when the computer has no internet.
store-app-picture-offline = Изображения нельзя загрузить, пока компьютер не подключён к интернету.

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = Изображение { $at } из { $n }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = Изображение { $at } из { $n }: { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = Предыдущее изображение

# Tip of the round button that shows the picture after.
store-app-picture-later = Следующее изображение

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = Проприетарная

# Name of the Facts card.
store-app-facts = Сведения

# Facts line: the size of the download.
store-app-fact-download = Размер загрузки

# Facts line: where the app comes from.
store-app-fact-source = Источник

# Facts line: the version.
store-app-fact-version = Версия

# Facts line: the licence of an open source app.
store-app-fact-licence = Лицензия

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = Лицензия, не открытый код

# Facts line: who made the app.
store-app-fact-made-by = Автор

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  по { $n ->
        [one] { $count } оценке
        [few] { $count } оценкам
        [many] { $count } оценкам
       *[other] { $count } оценкам
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = Оценок пока нет.

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = Нет оценок для показа.

# Under the stars while the ratings are being fetched.
store-app-rating-looking = Поиск оценок…

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = Из открытой службы отзывов GNOME.

# Last line of the Rating card when there is no internet.
store-app-rating-offline = Этот компьютер не подключён к интернету.

# Name of the Rating card.
store-app-rating = Оценка

# Text of the About card when the app says nothing of itself.
store-app-no-description = У этого приложения нет описания.

# Name of the About card.
store-app-about = О приложении

# Heading of a review that has no title.
store-app-review-no-title = Без заголовка

# Under What people say when nobody has written.
store-app-people-none = Об этом приложении пока никто не писал.

# Under What people say when there is no internet.
store-app-people-offline = Отзывы нельзя загрузить, пока компьютер не подключён к интернету.

# Under What people say while reviews are being fetched.
store-app-people-looking = Поиск отзывов…

# Name of the card of reviews.
store-app-people = Что говорят люди

# Facts line: when the app's project was made.
store-app-fact-created = Создано

