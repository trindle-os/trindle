# machine translated (Russian), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Сообщить о проблеме

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Что-то пошло не так. Отправить отчёт?

# SAFETY: needs a human review before it ships
# Under the title: what a report holds, and what it never holds.
report-summary = Отправляются ошибка, сводка о вашем компьютере и последние строки журнала. Имена, адреса электронной почты, сетевые адреса и названия сетей Wi-Fi удаляются. Имена файлов и веб-адреса всё же могут остаться в строках журнала. Ваши файлы и пароли не отправляются.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = Что произошло?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Какое приложение?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Не уверен

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Другая программа

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = Рабочий стол

# SAFETY: needs a human review before it ships
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = Отправить

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when the person opened it themselves.
report-cancel = Отмена

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = Не отправлять

# Button that opens the exact text of the report.
report-show-details = Показать подробности

# Button that closes the exact text of the report.
report-hide-details = Скрыть подробности

# Button under the shown report text that copies it.
report-copy = Копировать

# Button under the shown report text that saves it as a file in Downloads.
report-save = Сохранить

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Отчёты пока нельзя отправить.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Проверка.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Слишком много отчётов одновременно.

# Written beside the Send button once the report has been sent.
report-already-sent = Уже отправлено.

# Written under the typing box when Save is pressed a second time.
report-already-saved = Уже сохранено.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Ожидание подключения.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Ожидание подключения. { $why }

# Under the typing box after the report was sent.
report-sent = Отправлено. Спасибо.

# Under the typing box while the report is being posted.
report-sending = Отправка.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Сохранено как Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Укажите имя пользователя Buildmorphic, а не адрес электронной почты. Символ @ не нужен.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Имя пользователя должно быть длиной от 2 до 31 символа и состоять из строчных букв, цифр или дефисов.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Имя GitHub может содержать только буквы, цифры и дефисы.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle не удалось сохранить отчёт на потом: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Не удалось сохранить: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = нет прав

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = диск заполнен

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = папка отсутствует

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = ошибка диска

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = не найдена программа curl.

# Why a report could not be posted: the post could not be started.
report-not-posted = Не удалось отправить.

# Why a report could not be posted: the computer is offline.
report-no-connection = Нет подключения.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Сервер ответил кодом { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Напишите ещё несколько слов о том, что произошло.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Чтение журнала.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Нет журнала, который можно отправить вместе с отчётом, поэтому отправить его нельзя. Сохраните отчёт вместо этого.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Картинка по желанию. Её видит только владелец проекта.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = Выбрать картинку PNG

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Сделать снимок экрана

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Убрать картинку

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Ваша картинка, { $width } на { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = Выберите картинку PNG

report-picture-filter = Картинки PNG

# Under the picture while it is being read.
report-picture-reading = Чтение.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Не удалось сделать снимок экрана.

report-picture-no-chooser = Trindle не смог открыть окно выбора файла.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = Это не картинка PNG.

report-picture-too-big = Больше { $megabytes } МБ.

report-picture-too-large = Снимок слишком большой. Он может быть до { $side } пикселей в ширину и { $side } в высоту.

report-picture-damaged = Картинка повреждена.

report-picture-unreadable = Не удалось прочитать.



report-sent-as-bug = Отправлено. Номер вашего отчёта об ошибке: { $number }. Спасибо.


# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle находится в бета-версии. Отчёты анонимны: имена, адреса электронной почты и домашние папки удаляются. Имя пользователя указывать необязательно.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Имя пользователя Buildmorphic (необязательно)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle не удалось отправить отчёт. { $why } Нажмите «Отправить», чтобы повторить.
