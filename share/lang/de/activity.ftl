
# machine translated (German), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = Aktivität

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = Über diese Seite

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Erneut versuchen

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Die aktuellen Aktivitäten des Computers konnten nicht ausgelesen werden.

# In place of the room when no program is running.
activity-no-programs = Es laufen noch keine Programme.

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = Keine Einträge, solange die Computeraktivität nicht ausgelesen werden kann.

# The one sentence above the room, saying what it shows.
activity-caption = Jede Blase ist ein Programm, so groß wie der Speicher, den es belegt.

# The one sentence above the list of programs.
activity-list-caption = Tippe auf ein Programm, um es im Raum zu sehen.

# Heading of the card that lists the programs.
activity-programs = Programme

# Column heading of the list: the name of each program.
activity-column-program = Programm

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Speicher

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Prozessor

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Programme, jedes so groß wie der Speicher, den es belegt

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = Was dieser Computer gerade tut. Jedes Programm ist eine Blase im großen Kreis, so groß wie der Speicher, den es belegt. Je voller der Kreis, desto voller der Speicher.

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = Ein Programm, das arbeitet, leuchtet und pulsiert, schneller und heller, je härter es arbeitet: Eine kühle Farbe bedeutet wenig Arbeit, Orange harte Arbeit und Rot Volllast. Am Rand steht für jeden Thread des Prozessors ein Balken, der wächst und wärmer wird, je mehr zu tun ist. Wenn alle lang und rot sind, läuft der Computer auf Volllast, und die hellste Blase ist der Grund.

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
# SAFETY: needs a human review before it ships
activity-help-3 = Klicke auf eine Blase oder ein Programm in der Liste, um in einfachen Zahlen zu sehen, was es tut, und um es zu beenden, wenn es nicht mehr antwortet: Die Stopp-Taste, zweimal gedrückt, bittet es, sich zu schließen, und alles Ungespeicherte darin kann verloren gehen. Zwei ruhige graue Blasen enthalten den Rest: Trindle-Desktop, das ist der Compositor, der den Bildschirm, den Ring, den Tab und den Hintergrund zeichnet, und Linux und andere Programme, das ist Linux selbst und die Programme, die den Computer betreuen.

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = Der Speicher wird auf die faire Weise gezählt, wie es Werkzeuge wie smem tun: Speicher, den sich zwei Programme teilen, wird zwischen ihnen aufgeteilt, und in die Auslagerungsdatei verschobener Speicher ist eingerechnet, damit nichts doppelt gezählt oder ausgelassen wird. Die Karten neben dem Bild zeigen, wie voll Speicher, Prozessor, Festplatte und Auslagerungsdatei sind; ein Ring wird erst orange, dann rot, wenn etwas fast voll ist.

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = Gerade jetzt

# The sentence at the top while the first look is being taken.
activity-looking = Es wird nachgesehen…

# The sentence at the top when the computer could not be read at all.
activity-unread = Der Computer konnte gerade nicht gelesen werden.

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Dein Computer läuft auf Volllast: { $a } nutzt { $threads }.

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Dein Computer läuft auf Volllast: { $a } und { $b } nutzen { $threads }.

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Dein Computer läuft auf Volllast bei { $threads }; { $a } ist am meisten beschäftigt.

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Dein Computer läuft auf Volllast bei { $threads }.

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = Der Speicher ist fast voll; { $b } belegt am meisten.

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = Der Speicher ist fast voll.

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = Der Computer läuft ruhig vor sich hin; { $a } ist am meisten beschäftigt.

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = Der Computer ist beschäftigt.

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = Der Computer läuft ruhig vor sich hin.

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = { $a } leistet gerade den größten Teil der Arbeit.

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } und { $b } halten den Computer beschäftigt.

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } arbeitet hart, aber der Computer hat reichlich Reserve.

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = Alles ruhig, aber der Speicher füllt sich; { $b } belegt am meisten.

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = Alles ruhig: Nichts arbeitet hart.

# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = alle { $n ->
        [one] { $n } Thread
       *[other] { $n } Threads
    }

# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = weniger als 1 von { $n ->
        [one] { $n } Thread
       *[other] { $n } Threads
    }

# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = etwa { $n } von { $total ->
        [one] { $total } Thread
       *[other] { $total } Threads
    }

# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = von { $n ->
        [one] { $n } Thread
       *[other] { $n } Threads
    }

# A program's name when it has none that can be read.
activity-program-unnamed = Ein Programm

# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = unter 1 %

# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = unter 1 MB

# How a thread is described in its tip when it is resting.
activity-resting = ruht

# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = { $percent } ausgelastet

# How hard a program is working, when it is resting.
activity-effort-resting = Ruht

# How hard a program is working, when it is working a little.
activity-effort-little = Arbeitet ein wenig

# How hard a program is working, when it is working flat out.
activity-effort-flat-out = Arbeitet auf Volllast

# How hard a program is working, when it is working hard.
activity-effort-hard = Arbeitet hart

# How long something has been running, when it is under a minute.
activity-under-a-minute = Unter einer Minute

# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
        [one] 1 Minute
       *[other] { $n } Minuten
    }

# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
        [one] 1 Stunde
       *[other] { $n } Stunden
    }

# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
        [one] 1 Tag
       *[other] { $n } Tage
    }

# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }

# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = Arbeitet am härtesten

# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }

# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work } und belegt am meisten Speicher

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = Belegt am meisten Speicher

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = Am zweitmeisten Speicher

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Belegt { $amount }, { $how }

# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = { $share } des Speichers

# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = { $amount } Speicher frei

# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux selbst

# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle selbst

# The name of the bubble for Trindle's own desktop: the compositor, the ring, the tab and the wallpaper.
activity-trindle-desktop = Trindle-Desktop

# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux und andere Programme

# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, Karte geöffnet

# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }

# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = Thread { $n }: { $busy }

# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = Mehr über { $name }

# Words in a card when a program's figures have gone (it has closed).
activity-closed = Geschlossen

# Button on a card that closes it.
activity-close = Schließen

# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop-again = Noch einmal klicken, um { $title } zu beenden

# Tip of the stop button on a card. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop = { $title } beenden

# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 Teil

# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n } getrennte Teile

# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = Läuft seit

# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = Besteht aus

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Konnte noch nicht gelesen werden.

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } belegt; Trindle selbst belegt { $ours }.

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = Wie stark { $threads } in den letzten zwei Minuten ausgelastet waren.

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } belegt; { $free } noch frei.

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = von { $amount }

# Name of the disk card.
activity-disk = Festplatte

# Name of the swap file card.
activity-swap-file = Auslagerungsdatei

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } belegt; der Rest ist Platz für den Ruhezustand.

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Temperatur

# Sentence under the temperature figure.
activity-temperature-sentence = Wie warm der Prozessor gerade ist.

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = von 100°C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = Dieser Computer hat keine Auslagerungsdatei, deshalb wird nichts aus dem Speicher ausgelagert.


# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = Zwangsbeendet: { $title } hat nicht geantwortet

