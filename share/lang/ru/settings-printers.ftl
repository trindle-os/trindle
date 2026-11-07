# machine translated (Russian), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Принтеров пока нет. Найденные в вашей сети перечислены ниже.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } приостановлен, поэтому пока на нём ничего не напечатается.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } печатает.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Готово к печати на { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name }: { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } настроен; принтер для печати по умолчанию не выбран.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = Настроено принтеров: { $n }; принтер для печати по умолчанию не выбран.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = CUPS не знает такого задания.

# SAFETY: needs a human review before it ships
# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Отменено

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat завершилась с ошибкой

# Words in the ring while the network is searched for printers.
settings-printers-searching = Поиск

# Words in the ring when the search is finished.
settings-printers-search-done = Готово

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Теперь по умолчанию

# Shown beside a printer after a test page was sent.
settings-printers-sent = Отправлено

# SAFETY: needs a human review before it ships
# Shown beside a printer after it was removed.
settings-printers-removed = Удалён

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Сначала введите адрес принтера.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Добавляется...

# Shown beside a printer or the address line after it was added.
settings-printers-added = Добавлен

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Поиск драйвера...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Задание { $number } на { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# SAFETY: needs a human review before it ships
# Tip of the cross button that cancels a print job.
settings-printers-cancel-job = Отменить это задание

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Задания печати

# SAFETY: needs a human review before it ships
# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = Все задания, ожидающие печати, на любом принтере, в их нынешнем виде. Пока эта страница открыта, список перечитывается каждые несколько секунд. Крестик отменяет задание; уже напечатанная бумага остаётся напечатанной.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS не отвечает

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Очередь нельзя прочитать.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Просмотр…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = Очередь.

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Ничего не ждёт

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = Задания появляются здесь, пока печатаются.

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Приостановлен

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Простаивает

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Печатает

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = По умолчанию, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Печатать сначала на этом

# Tip of the button that prints a test page.
settings-printers-test-page = Напечатать пробную страницу

# SAFETY: needs a human review before it ships
# Tip of the bin button that removes a printer.
settings-printers-remove = Удалить этот принтер

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Для поиска нужен cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Установите его командой: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Добавить этот принтер

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Новых не найдено

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = Он включён и в той же сети?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = Сначала те, что объявляют о себе, затем каждый компьютер по очереди.

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Добавить

# Heading of the line where a printer's address is typed.
settings-printers-by-address = По адресу

# Small line under "By its address".
settings-printers-by-address-sub = Например 192.168.1.50, он указан на экране самого принтера.

# Heading of the card for adding printers.
settings-printers-add-card = Добавить принтер

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Сначала ищутся принтеры, которые объявляют о себе, затем по очереди опрашивается каждая машина в сети. Принтер должен быть включён и находиться в той же сети. Некоторые принтеры о себе не объявляют, а некоторые сети не передают это объявление: тогда добавьте принтер по адресу, например 192.168.1.50. Его можно найти на экране самого принтера в сетевых настройках.

# Name of the page in the list of settings pages.
settings-printers-title = Принтеры

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = принтер печать сканер

# Help text at the top of the page.
settings-printers-help = Принтеры в вашей сети находятся сами. Нажмите «Добавить» рядом с нужным, чтобы пользоваться им.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = Печать не установлена на этом компьютере.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Просмотр…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Служба печати не отвечает, поэтому сейчас ничего нельзя напечатать.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Печать

# Help text (i button) of that card.
settings-printers-printing-about = Для печати нужен CUPS, которого нет на этом компьютере. После его установки принтеры в вашей сети находятся сами.

# Line in that card.
settings-printers-unavailable = Печать недоступна

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Установите его командой: sudo apt install cups

# Line in that card.
settings-printers-then-found = Тогда принтеры находятся

# Small line under it.
settings-printers-then-found-sub = Те, что в вашей сети, сами.

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Служба печати.

# Line in the Printers card when there are none.
settings-printers-none = Принтеров пока нет

# Small line under "No printers yet".
settings-printers-none-sub = Добавьте один рядом.

# Heading of the card that lists the printers.
settings-printers-card = Принтеры

# SAFETY: needs a human review before it ships
# Help text (i button) of the Printers card.
settings-printers-card-about = Все принтеры, настроенные на этом компьютере. Звёздочка делает принтер используемым первым; значок страницы печатает на нём пробную страницу; корзина удаляет его, дважды спрашивая. Для изменений ваша учётная запись должна входить в группу lpadmin.

# Red words under a page range box that cannot be used.
printers-bad-range = Страницы, например 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = Копий: от 1 до 99

# Name of the choice to print on one side of the paper.
printers-sides-one = Одна сторона

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Обе, по длинному краю

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Обе, по короткому краю

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = Книжная

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Альбомная

# SAFETY: needs a human review before it ships
# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = Не разрешено. Ваша учётная запись должна входить в группу lpadmin; после этого выйдите из системы и войдите снова.

# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS отказал.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = CUPS не допускает такое имя принтера.

# Red words when printing is not installed on the computer.
printers-not-installed = Печать не установлена на этом компьютере.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } больше не настроен.

# Red words when the printer chosen is paused.
printers-paused = { $name } приостановлен.

# Red words when sending the document to CUPS took too long.
printers-too-slow = Документ отправлялся слишком долго.
