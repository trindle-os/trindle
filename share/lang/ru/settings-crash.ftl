# machine translated (Russian), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Отчёты о сбоях

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = отчёты о сбоях сбои ошибки неполадки отправка отправленные конфиденциальность телеметрия buildmorphic

# Help line at the foot of the Crash reports page.
settings-crash-help = Когда одна из программ Trindle даёт сбой, модуль отчётов о сбоях хранит на этом компьютере короткую заметку. Он ничего не отправляет, пока вы не запустите его команду отправки и не ответите «да» на каждый отчёт; перед этим он показывает отчёт целиком. Ваше имя и имя на GitHub попадают в отчёт, только если вы ввели их здесь. Никогда не отправляются сообщения, имена файлов, ваше местоположение или что-либо о других программах. Удаление модуля останавливает всё.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Сбои

# Under the Crashes switch.
settings-crash-crashes-short = Где в коде Trindle произошёл сбой.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = «Сбои» отправляют название программы, её версию, вид сбоя и место в коде Trindle, где он произошёл.

# Heading and label of the switch for sending errors.
settings-crash-errors = Ошибки

# Under the Errors switch.
settings-crash-errors-short = Постоянное название ошибки и место.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = «Ошибки» отправляют название программы, её версию, постоянное название, которое Trindle даёт ошибке, и место в коде.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = О компьютере

# Under the About this computer switch.
settings-crash-computer-short = Linux, процессор, память, графика, экраны.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = «О компьютере» добавляет к каждому отчёту: версию Linux, вид процессора, объём памяти с округлением вниз, Wayland или X11, производителя графики и число экранов.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Что отправляется

# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } Отчёты не содержат имени, пока вы не введёте своё имя пользователя Buildmorphic в поле ниже; это лишь даёт вам признание.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Ничего не изменено: { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Ничего не изменено: файл, где это хранится, не такой, каким его записал Trindle.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Нет папки параметров.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Не удалось создать папку параметров.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Не удалось записать параметр.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Сбой ({ $signal }) в { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Ошибка { $code } в { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Принят

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Не отправлен: трекер закрыт

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Не отправлен: слишком большой

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Не отправлен: отклонён

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Не отправлен: слишком много одновременно

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Не отправлен

# Said beside the link line once it is saved.
settings-crash-saved = Сохранено

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Очистить список

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = Не удалось очистить список.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Удалено

# Heading of the line that holds the clear button.
settings-crash-kept-here = Хранится только на этом компьютере

# Under the Kept on this computer only heading.
settings-crash-newest-first = Сначала новые, не больше пятидесяти.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Пока ничего

# Under the Nothing yet heading.
settings-crash-listed-here = Каждый отчёт показан здесь точно в том виде, в каком был отправлен.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Показать, что именно было отправлено

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
        [one] { $when }, повторов: { $n }: { $answer }
        [few] { $when }, повторов: { $n }: { $answer }
        [many] { $when }, повторов: { $n }: { $answer }
       *[other] { $when }, повторов: { $n }: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Отправлено на сегодня

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Каждый отчёт показан в том виде, в каком был отправлен, сначала новые, и хранится только на этом компьютере. «Принят» значит, что Buildmorphic его получил. «Не отправлен: трекер закрыт» значит, что трекер ошибок Trindle не принимает отчёты; «отклонён» значит, что отчёт отклонён как неправильно составленный; «слишком много одновременно» значит, что отчётов пришло слишком много сразу.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = После сбоя вас спросят, отправлять ли отчёт, и покажут его целиком.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Ничего не отправляется, и после сбоя вас не спрашивают.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Ничего не отправляется: сбои и ошибки выключены.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Об ошибках программ Trindle сообщается анонимно.
        [both] О сбоях и ошибках программ Trindle сообщается анонимно.
       *[crashes] О сбоях программ Trindle сообщается анонимно.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Спрашивать о проблемах и отправке отчётов

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Если выключено, всё скрывается, а ожидающие отчёты удаляются.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Выключено. Ожидающих отчётов не было.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
        [one] Выключено. Удалён { $n } ожидающий отчёт.
        [few] Выключено. Удалено ожидающих отчётов: { $n }.
        [many] Выключено. Удалено ожидающих отчётов: { $n }.
       *[other] Выключено. Удалено ожидающих отчётов: { $n }.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = О сообществе

# Heading of the line with the community button.
settings-crash-go-to-bm = Отчёты отправляются в Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = Сообщество, которое помогает создавать Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = имя пользователя

# Heading of the line with the username box.
settings-crash-your-username = Ваше имя пользователя в Buildmorphic

# Under the username heading.
settings-crash-username-sub = По желанию. Никогда не адрес электронной почты.

# Heading of a line about joining with an email address.
settings-crash-join = Присоединиться, указав только адрес электронной почты

# Under the Join heading.
settings-crash-join-sub = Получайте награды и уровни за проблемы, о которых вы сообщили первым.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Отчёт содержит только фиксированный список

# Under the fixed list heading.
settings-crash-fixed-sub = Вы видите его заранее, и ничего не уходит без вашего нажатия.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Отчёты, которые вы отправляете

# Help text of the Reports you send card (the information button).
settings-crash-reports-about = Когда одна из программ Trindle даёт сбой, может открыться окно с вопросом, отправлять ли отчёт. Сначала оно показывает точный текст: ваши слова, модель компьютера и графики, ошибку и журнал незадолго до неё. Файлы, пароли и названия сетей Wi-Fi никогда не читаются, а имена и адреса стираются. То же окно можно в любой момент открыть кнопкой «Сообщить о проблеме» в кольце.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = имя на github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Ваше имя на GitHub

# Small words under that heading.
settings-crash-github-about = По желанию. Показывается как «Сообщил».

