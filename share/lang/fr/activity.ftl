
# machine translated (French), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = Activité

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = À propos de cette page

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Réessayer

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Impossible de lire ce que fait l'ordinateur.

# In place of the room when no program is running.
activity-no-programs = Aucun programme n'est encore en cours d'exécution.

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = Rien à lister tant que l'ordinateur ne peut pas être lu.

# The one sentence above the room, saying what it shows.
activity-caption = Chaque bulle est un programme, aussi grande que la mémoire qu'il occupe.

# The one sentence above the list of programs.
activity-list-caption = Appuyez sur un programme pour le voir dans la pièce.

# Heading of the card that lists the programs.
activity-programs = Programmes

# Column heading of the list: the name of each program.
activity-column-program = Programme

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Mémoire

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Processeur

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Programmes, chacun aussi grand que la mémoire qu'il occupe

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = Ce que fait cet ordinateur en ce moment. Chaque programme est une bulle dans le grand cercle, aussi grande que la mémoire qu'il occupe : plus le cercle est plein, plus la mémoire est pleine.

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = Un programme qui travaille brille et bat, plus vite et plus fort quand il travaille davantage : une couleur froide indique peu de travail, l'orange un travail soutenu et le rouge le maximum. Autour du bord, une barre représente chaque fil du processeur ; elle grandit et se réchauffe à mesure qu'il est occupé. Quand toutes sont longues et rouges, l'ordinateur est à plein régime et la bulle la plus vive en est la cause.

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
# SAFETY: needs a human review before it ships
activity-help-3 = Cliquez sur une bulle, ou sur un programme de la liste, pour voir ce qu'il fait en chiffres simples, et pour l'arrêter s'il ne répond plus : le bouton d'arrêt, pressé deux fois, lui demande de se fermer, et ce qui n'y est pas enregistré peut être perdu. Deux bulles grises discrètes contiennent le reste : le bureau Trindle, qui est le compositeur qui dessine l'écran, l'anneau, l'onglet et le fond d'écran, et Linux et autres programmes, c'est-à-dire Linux lui-même et les programmes qui s'occupent de l'ordinateur.

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = La mémoire est comptée de façon équitable, comme le font des outils tels que smem : la mémoire que deux programmes partagent est répartie entre eux, et la mémoire déplacée vers le fichier d'échange est incluse, si bien que rien n'est compté deux fois ni oublié. Les cartes à côté de l'image montrent le remplissage de la mémoire, du processeur, du disque et du fichier d'échange ; un anneau devient orange, puis rouge, seulement quand l'un d'eux est presque plein.

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = En ce moment

# The sentence at the top while the first look is being taken.
activity-looking = Observation…

# The sentence at the top when the computer could not be read at all.
activity-unread = L'ordinateur n'a pas pu être lu à l'instant.

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Votre ordinateur tourne à plein régime : { $a } utilise { $threads }.

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Votre ordinateur tourne à plein régime : { $a } et { $b } utilisent { $threads }.

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Votre ordinateur tourne à plein régime sur { $threads } ; { $a } est le plus actif.

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Votre ordinateur tourne à plein régime sur { $threads }.

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = La mémoire est presque pleine ; { $b } en occupe le plus.

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = La mémoire est presque pleine.

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = L'ordinateur tourne tranquillement ; { $a } est le plus actif.

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = L'ordinateur est occupé.

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = L'ordinateur tourne tranquillement.

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = { $a } fait l'essentiel du travail en ce moment.

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } et { $b } occupent l'ordinateur.

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } travaille dur, mais l'ordinateur a de la marge.

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = Tout est calme, mais la mémoire se remplit ; { $b } en occupe le plus.

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = Tout est calme : rien ne travaille dur.

# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = tous les { $n ->
        [one] { $n } fil
       *[other] { $n } fils
    }

# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = moins de 1 fil sur { $n ->
        [one] { $n } fil
       *[other] { $n } fils
    }

# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = environ { $n } sur { $total ->
        [one] { $total } fil
       *[other] { $total } fils
    }

# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = sur { $n ->
        [one] { $n } fil
       *[other] { $n } fils
    }

# A program's name when it has none that can be read.
activity-program-unnamed = Un programme

# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = moins de 1 %

# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = moins de 1 Mo

# How a thread is described in its tip when it is resting.
activity-resting = au repos

# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = { $percent } d'occupation

# How hard a program is working, when it is resting.
activity-effort-resting = Au repos

# How hard a program is working, when it is working a little.
activity-effort-little = Travaille un peu

# How hard a program is working, when it is working flat out.
activity-effort-flat-out = Travaille à plein régime

# How hard a program is working, when it is working hard.
activity-effort-hard = Travaille dur

# How long something has been running, when it is under a minute.
activity-under-a-minute = Moins d'une minute

# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
        [one] 1 minute
       *[other] { $n } minutes
    }

# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
        [one] 1 heure
       *[other] { $n } heures
    }

# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
        [one] 1 jour
       *[other] { $n } jours
    }

# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }

# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = Travaille le plus

# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort } : { $work }

# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort } : { $work }, et occupe le plus de mémoire

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = Occupe le plus de mémoire

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = Ensuite en mémoire

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Occupe { $amount }, { $how }

# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = { $share } de la mémoire

# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = { $amount } de mémoire libre

# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux lui-même

# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle lui-même

# The name of the bubble for Trindle's own desktop: the compositor, the ring, the tab and the wallpaper.
activity-trindle-desktop = Bureau Trindle

# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux et autres programmes

# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, carte ouverte

# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }

# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = Fil { $n } : { $busy }

# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = En savoir plus sur { $name }

# Words in a card when a program's figures have gone (it has closed).
activity-closed = Fermé

# Button on a card that closes it.
activity-close = Fermer

# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop-again = Cliquez de nouveau pour arrêter { $title }

# Tip of the stop button on a card. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop = Arrêter { $title }

# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 partie

# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n } parties distinctes

# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = En cours depuis

# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = Composé de

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Impossible à lire pour l'instant.

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } utilisés ; Trindle lui-même en occupe { $ours }.

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = Occupation de { $threads } depuis deux minutes.

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } utilisés ; { $free } encore libres.

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = sur { $amount }

# Name of the disk card.
activity-disk = Disque

# Name of the swap file card.
activity-swap-file = Fichier d'échange

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } utilisés ; le reste sert à l'hibernation.

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Température

# Sentence under the temperature figure.
activity-temperature-sentence = La chaleur du processeur en ce moment.

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = sur 100 °C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = Cet ordinateur n'a pas de fichier d'échange, donc rien n'est déplacé hors de la mémoire.



# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = Fermeture forcée : { $title } ne répondait pas

