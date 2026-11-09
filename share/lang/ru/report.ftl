# machine translated (Russian), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Сообщить о проблеме

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Что-то пошло не так. Отправить отчёт?

# SAFETY: needs a human review before it ships
# Under the title: what a report holds, and what it never holds.
report-summary = Отправляются ошибка, сводка о компьютере и последние строки журнала, из которых удалены имена, адреса электронной почты, сетевые адреса и названия сетей Wi-Fi. Имена файлов и веб-адреса всё же могут остаться в строках журнала. Ваши файлы и пароли не отправляются.

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = Что вы делали? (по желанию)

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
report-not-taken = Отчёты пока не принимаются.

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
report-no-at = Укажите имя пользователя Buildmorphic, а не адрес электронной почты, то есть без @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Имя пользователя состоит из 2–31 строчных букв, цифр или дефисов.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Имя GitHub может содержать только буквы, цифры и дефисы.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Не удалось сохранить на потом: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Не удалось сохранить: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = нет прав

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = диск заполнен

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = папка отсутствует

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = диск отказал в записи

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = не найдена программа curl.

# Why a report could not be posted: the post could not be started.
report-not-posted = Не удалось отправить.

# Why a report could not be posted: the computer is offline.
report-no-connection = Нет подключения.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Адрес ответил кодом { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Напишите ещё несколько слов о том, что произошло.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Чтение журнала.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Нет журнала, который можно отправить вместе с ним, поэтому отправить нельзя. Сохраните его вместо этого.



report-sent-as-bug = Отправлено. Принято как ошибка { $number }. Спасибо.


# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle находится в бета-версии, и каждый отчёт помогает нам довести его работу до совершенства. Отчёт анонимный и не содержит ничего личного. Электронная почта не нужна, имя пользователя необязательно. Ничего не отправляется, пока вы не нажмёте «Отправить».

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Имя пользователя Buildmorphic (необязательно)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Не отправлено. { $why } Нажмите «Отправить», чтобы повторить.

