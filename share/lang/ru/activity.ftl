# machine translated (Russian), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = Активность

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = Об этой странице

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Повторить

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Не удалось узнать, чем занят компьютер.

# In place of the room when no program is running.
activity-no-programs = Пока не запущено ни одной программы.

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = Показывать нечего, пока не удастся прочитать данные компьютера.

# The one sentence above the room, saying what it shows.
activity-caption = Каждый пузырь это программа, размером с занимаемую ею память.

# The one sentence above the list of programs.
activity-list-caption = Нажмите на программу, чтобы увидеть её в круге.

# Heading of the card that lists the programs.
activity-programs = Программы

# Column heading of the list: the name of each program.
activity-column-program = Программа

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Память

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Процессор

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Программы, каждая размером с занимаемую ею память

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = Чем сейчас занят этот компьютер. Каждая программа это пузырь в большом круге, размером с занимаемую ею память, поэтому чем полнее круг, тем полнее память.

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = Работающая программа светится и пульсирует, тем быстрее и ярче, чем больше она работает: холодный цвет означает мало работы, оранжевый много, а красный предел. По краю идёт по одной полосе на каждый поток процессора; чем он занятее, тем полоса длиннее и теплее, поэтому когда все полосы длинные и красные, компьютер работает на пределе, а виноват самый яркий пузырь.

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
# SAFETY: needs a human review before it ships
activity-help-3 = Нажмите на пузырь или на программу в списке, чтобы увидеть простыми цифрами, чем она занята, и остановить её, если она перестала отвечать: кнопка остановки, нажатая дважды, просит программу закрыться, и всё несохранённое в ней может быть потеряно. Два тихих серых пузыря содержат остальное: рабочий стол Trindle, то есть композитор, который рисует экран, кольцо, вкладку и обои, и Linux и другие программы, то есть сам Linux и программы, которые присматривают за компьютером.

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = Память считается честным способом, как в таких средствах, как smem: память, общая для двух программ, делится между ними, а память, вытесненная в файл подкачки, включается в подсчёт, поэтому ничто не считается дважды и не теряется. Карточки рядом с картинкой показывают, насколько заполнены память, процессор, диск и файл подкачки; кольцо становится оранжевым, а затем красным, только когда что-то почти заполнено.

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = Прямо сейчас

# The sentence at the top while the first look is being taken.
activity-looking = Смотрим…

# The sentence at the top when the computer could not be read at all.
activity-unread = Сейчас не удалось прочитать данные компьютера.

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Компьютер работает на пределе: { $a } использует { $threads }.

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Компьютер работает на пределе: { $a } и { $b } используют { $threads }.

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Компьютер работает на пределе, занято { $threads }; самая загруженная программа: { $a }.

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Компьютер работает на пределе, занято { $threads }.

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = Память почти заполнена; больше всех занимает { $b }.

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = Память почти заполнена.

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = Компьютер работает в обычном режиме; самая загруженная программа: { $a }.

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = Компьютер занят.

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = Компьютер работает в обычном режиме.

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = Большую часть работы сейчас делает { $a }.

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = Компьютер занимают { $a } и { $b }.

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } усиленно работает, но у компьютера большой запас.

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = Всё спокойно, но память заполняется; больше всех занимает { $b }.

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = Всё спокойно: ничто не нагружает компьютер.

# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = { $n ->
        [one] весь { $n } поток
        [few] все { $n } потока
        [many] все { $n } потоков
       *[other] все { $n } потока
    }

# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = меньше 1 из { $n ->
        [one] { $n } потока
        [few] { $n } потоков
        [many] { $n } потоков
       *[other] { $n } потока
    }

# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = около { $n } из { $total ->
        [one] { $total } потока
        [few] { $total } потоков
        [many] { $total } потоков
       *[other] { $total } потока
    }

# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = из { $n ->
        [one] { $n } потока
        [few] { $n } потоков
        [many] { $n } потоков
       *[other] { $n } потока
    }

# A program's name when it has none that can be read.
activity-program-unnamed = Программа

# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = меньше 1%

# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = меньше 1 МБ

# How a thread is described in its tip when it is resting.
activity-resting = отдыхает

# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = занято на { $percent }

# How hard a program is working, when it is resting.
activity-effort-resting = Отдыхает

# How hard a program is working, when it is working a little.
activity-effort-little = Немного работает

# How hard a program is working, when it is working flat out.
activity-effort-flat-out = Работает на пределе

# How hard a program is working, when it is working hard.
activity-effort-hard = Усиленно работает

# How long something has been running, when it is under a minute.
activity-under-a-minute = Меньше минуты

# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
        [one] { $n } минута
        [few] { $n } минуты
        [many] { $n } минут
       *[other] { $n } минуты
    }

# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
        [one] { $n } час
        [few] { $n } часа
        [many] { $n } часов
       *[other] { $n } часа
    }

# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
        [one] { $n } день
        [few] { $n } дня
        [many] { $n } дней
       *[other] { $n } дня
    }

# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }

# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = Работает усерднее всех

# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }

# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work }, и занимает больше всех памяти

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = Занимает больше всех памяти

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = Следующая по объёму памяти

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Занимает { $amount }, { $how }

# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = { $share } памяти

# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = свободно памяти: { $amount }

# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Сам Linux

# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Сам Trindle

# The name of the bubble for Trindle's own desktop: the compositor, the ring, the tab and the wallpaper.
activity-trindle-desktop = Рабочий стол Trindle

# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux и другие программы

# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, карточка открыта

# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }

# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = Поток { $n }: { $busy }

# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = Подробнее о { $name }

# Words in a card when a program's figures have gone (it has closed).
activity-closed = Закрыта

# Button on a card that closes it.
activity-close = Закрыть

# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop-again = Нажмите ещё раз, чтобы остановить { $title }

# Tip of the stop button on a card. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop = Остановить { $title }

# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 часть

# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n ->
        [one] { $n } отдельная часть
        [few] { $n } отдельные части
        [many] { $n } отдельных частей
       *[other] { $n } отдельной части
    }

# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = Работает уже

# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = Состоит из

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Пока не удалось прочитать.

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = Используется { $used }; из них самому Trindle принадлежит { $ours }.

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = Насколько были заняты { $threads } за последние две минуты.

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = Занято { $used }; ещё свободно { $free }.

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = из { $amount }

# Name of the disk card.
activity-disk = Диск

# Name of the swap file card.
activity-swap-file = Файл подкачки

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = Используется { $used }; остальное место для гибернации.

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Температура

# Sentence under the temperature figure.
activity-temperature-sentence = Насколько сейчас нагрет процессор.

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = из 100°C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = У этого компьютера нет файла подкачки, поэтому ничего не вытесняется из памяти.


# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = Принудительно закрыто: { $title } не отвечает

