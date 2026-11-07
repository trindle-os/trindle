
# machine translated (Spanish), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = Actividad

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = Acerca de esta página

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Reintentar

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = No se pudo leer lo que hace el equipo.

# In place of the room when no program is running.
activity-no-programs = Aún no hay programas en ejecución.

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = No hay nada que listar hasta que se pueda leer el equipo.

# The one sentence above the room, saying what it shows.
activity-caption = Cada burbuja es un programa, tan grande como la memoria que ocupa.

# The one sentence above the list of programs.
activity-list-caption = Pulsa un programa para verlo en la sala.

# Heading of the card that lists the programs.
activity-programs = Programas

# Column heading of the list: the name of each program.
activity-column-program = Programa

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Memoria

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Procesador

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Programas, cada uno tan grande como la memoria que ocupa

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = Lo que hace este equipo ahora mismo. Cada programa es una burbuja dentro del círculo grande, tan grande como la memoria que ocupa, así que cuanto más lleno está el círculo, más llena está la memoria.

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = Un programa que trabaja brilla y late, más rápido y más fuerte cuanto más trabaja: un color frío es poco trabajo, el naranja es mucho trabajo y el rojo es el máximo. Alrededor del borde hay una barra por cada hilo del procesador, que crece y se calienta a medida que está más ocupado; cuando todas son largas y rojas, el equipo va al máximo y la burbuja más brillante es el motivo.

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
activity-help-3 = Haz clic en una burbuja, o en un programa de la lista, para ver lo que hace con cifras sencillas, y para detenerlo si ha dejado de responder: el botón de detener, pulsado dos veces, le pide que se cierre, y lo que no esté guardado en él puede perderse. Dos burbujas grises y tranquilas contienen el resto: el escritorio de Trindle, que es el compositor que dibuja la pantalla, el anillo, la pestaña y el fondo de pantalla, y Linux y otros programas, que son Linux mismo y los programas que cuidan del equipo.

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = La memoria se cuenta de forma justa, como la cuentan herramientas como smem: la memoria que comparten dos programas se reparte entre ellos, y la memoria pasada al archivo de intercambio se incluye, así que nada se cuenta dos veces ni se queda fuera. Las tarjetas junto a la imagen muestran cuán llenos están la memoria, el procesador, el disco y el archivo de intercambio; un anillo se vuelve naranja, y luego rojo, solo cuando uno está casi lleno.

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = Ahora mismo

# The sentence at the top while the first look is being taken.
activity-looking = Mirando…

# The sentence at the top when the computer could not be read at all.
activity-unread = No se pudo leer el equipo en este momento.

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Tu equipo va al máximo: { $a } está usando { $threads }.

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Tu equipo va al máximo: { $a } y { $b } están usando { $threads }.

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Tu equipo va al máximo con { $threads }; { $a } es el más ocupado.

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Tu equipo va al máximo con { $threads }.

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = La memoria está casi llena; { $b } es el que más ocupa.

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = La memoria está casi llena.

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = El equipo va a ritmo tranquilo; { $a } es el más ocupado.

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = El equipo está ocupado.

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = El equipo va a ritmo tranquilo.

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = { $a } está haciendo la mayor parte del trabajo ahora mismo.

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } y { $b } mantienen ocupado el equipo.

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } trabaja a fondo, pero al equipo le sobra capacidad.

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = Todo en calma, pero la memoria se está llenando; { $b } es el que más ocupa.

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = Todo en calma: nada trabaja a fondo.

# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = {
    $n ->
        [one] el hilo
       *[other] los { $n } hilos
    }

# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = menos de 1 de { $n ->
        [one] { $n } hilo
       *[other] { $n } hilos
    }

# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = unos { $n } de { $total ->
        [one] { $total } hilo
       *[other] { $total } hilos
    }

# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = de { $n ->
        [one] { $n } hilo
       *[other] { $n } hilos
    }

# A program's name when it has none that can be read.
activity-program-unnamed = Un programa

# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = menos del 1%

# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = menos de 1 MB

# How a thread is described in its tip when it is resting.
activity-resting = en reposo

# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = { $percent } ocupado

# How hard a program is working, when it is resting.
activity-effort-resting = En reposo

# How hard a program is working, when it is working a little.
activity-effort-little = Trabajando un poco

# How hard a program is working, when it is working flat out.
activity-effort-flat-out = Trabajando al máximo

# How hard a program is working, when it is working hard.
activity-effort-hard = Trabajando a fondo

# How long something has been running, when it is under a minute.
activity-under-a-minute = Menos de un minuto

# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
        [one] 1 minuto
       *[other] { $n } minutos
    }

# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
        [one] 1 hora
       *[other] { $n } horas
    }

# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
        [one] 1 día
       *[other] { $n } días
    }

# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }

# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = El que más trabaja

# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }

# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work }, y es el que más memoria ocupa

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = El que más memoria ocupa

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = El siguiente en memoria

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Ocupa { $amount }, { $how }

# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = { $share } de la memoria

# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = { $amount } de memoria libre

# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux mismo

# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle mismo

# The name of the bubble for Trindle's own desktop: the compositor, the ring, the tab and the wallpaper.
activity-trindle-desktop = Escritorio de Trindle

# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux y otros programas

# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, tarjeta abierta

# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }

# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = Hilo { $n }: { $busy }

# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = Más sobre { $name }

# Words in a card when a program's figures have gone (it has closed).
activity-closed = Cerrado

# Button on a card that closes it.
activity-close = Cerrar

# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
activity-stop-again = Haz clic otra vez para detener { $title }

# Tip of the stop button on a card. $title is the program.
activity-stop = Detener { $title }

# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 parte

# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n } partes separadas

# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = En ejecución desde hace

# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = Formado por

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Aún no se pudo leer.

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } en uso; Trindle mismo ocupa { $ours }.

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = Cuán ocupados han estado { $threads } durante dos minutos.

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } usados; todavía quedan { $free } libres.

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = de { $amount }

# Name of the disk card.
activity-disk = Disco

# Name of the swap file card.
activity-swap-file = Archivo de intercambio

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } en uso; el resto es espacio para hibernar.

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Temperatura

# Sentence under the temperature figure.
activity-temperature-sentence = Cuán caliente está el procesador ahora mismo.

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = de 100 °C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = Este equipo no tiene archivo de intercambio, así que no se saca nada de la memoria.



# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = Cierre forzado: { $title } no respondió

