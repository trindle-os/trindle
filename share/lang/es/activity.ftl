
# machine translated (Spanish), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = Actividad

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = Acerca de esta página

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Reintentar

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Trindle no pudo comprobar la actividad del equipo.

# In place of the room when no program is running.
activity-no-programs = Aún no hay programas en ejecución.

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = Trindle no puede mostrar programas hasta que pueda comprobar la actividad del equipo.

# Heading of the card that lists the programs.
activity-programs = Programas

# Column heading of the list: the name of each program.
activity-column-program = Programa

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Memoria

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Procesador

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Burbujas de programas, con el tamaño según su uso de memoria

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = Aquí ves lo que hace tu equipo ahora mismo. Cada programa aparece como una burbuja dentro del círculo. Cuanto más grande es la burbuja, más memoria usa. Cuanto más lleno está el círculo, más memoria hay en uso.

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = La burbuja de un programa brilla y late mientras trabaja. Cuanto más rápidos y brillantes son los latidos, más trabajo hay. Los colores fríos significan poco trabajo, el naranja significa mucho trabajo y el rojo significa trabajo al máximo. Cada barra del borde representa un hilo, una parte del procesador que se encarga del trabajo. Cuanto más largas son las barras y más cálidos los colores, más trabajo hay. Cuando todas las barras son largas y rojas, el equipo está trabajando al máximo. La burbuja más brillante muestra qué programa hace más trabajo.

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
activity-help-3 = Haz clic en una burbuja, o en un programa de la lista, para ver lo que hace con cifras sencillas. También puedes detener un programa que ha dejado de responder. Pulsa dos veces el botón de detener y Trindle le pide al programa que se cierre. Lo que no hayas guardado en él puede perderse. Las dos burbujas grises muestran todo lo demás. El escritorio de Trindle es la parte que dibuja la pantalla, el anillo, la pestaña y el fondo de pantalla. Linux y otros programas es Linux mismo y los programas que cuidan del equipo.

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = Trindle cuenta la memoria con cuidado, igual que herramientas como smem. La memoria que comparten dos programas se reparte entre ellos, y la memoria pasada al archivo de intercambio se incluye, así que nada se cuenta dos veces ni se queda fuera. Las tarjetas junto a la imagen muestran cuán llenos están la memoria, el procesador, el disco y el archivo de intercambio. Un anillo solo se vuelve naranja, y luego rojo, cuando está casi lleno.

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = Ahora mismo

# The sentence at the top while the first look is being taken.
activity-looking = Mirando…

# The sentence at the top when the computer could not be read at all.
activity-unread = Trindle no pudo comprobar el equipo.

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Tu equipo está trabajando al máximo: { $a } está usando { $threads }.

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Tu equipo está trabajando al máximo: { $a } y { $b } están usando { $threads }.

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Tu equipo está trabajando al máximo con { $threads }. { $a } es el más ocupado.

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Tu equipo está trabajando al máximo con { $threads }.

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = La memoria está casi llena. { $b } es el que más usa.

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = La memoria está casi llena.

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = El equipo está tranquilo. { $a } es el que más trabajo hace.

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = El equipo está ocupado.

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = El equipo está tranquilo.

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = { $a } está haciendo la mayor parte del trabajo ahora mismo.

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } y { $b } mantienen ocupado el equipo.

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } trabaja a fondo, pero al equipo le sobra capacidad.

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = El equipo no tiene mucho trabajo, pero el uso de memoria está subiendo. { $b } es el que más usa.

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = Ningún programa está trabajando a fondo.

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
activity-effort-flat-out = Al máximo

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
activity-working-hardest-most = { $effort }: { $work }, y es el que más memoria usa

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = El que más memoria usa

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = El siguiente en memoria

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Usa { $amount }, { $how }

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
activity-made-of = Partes

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Aún no se puede leer.

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } en uso. Trindle mismo usa { $ours }.

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = Cuán ocupados estuvieron { $threads } en los últimos dos minutos.

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } usados y { $free } libres.

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = de { $amount }

# Name of the disk card.
activity-disk = Disco

# Name of the swap file card.
activity-swap-file = Archivo de intercambio

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } en uso.

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Temperatura

# Sentence under the temperature figure.
activity-temperature-sentence = Temperatura del procesador

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = de 100 °C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = Este equipo no tiene archivo de intercambio, así que no se saca nada de la memoria.



# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = { $title } no respondió, así que Trindle forzó su cierre
