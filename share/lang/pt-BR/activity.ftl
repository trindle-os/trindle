
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = Atividade

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = Sobre esta página

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Tentar de novo

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Não foi possível ler o que o computador está fazendo.

# In place of the room when no program is running.
activity-no-programs = Nenhum programa em execução ainda.

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = Nada para listar até que o computador possa ser lido.

# The one sentence above the room, saying what it shows.
activity-caption = Cada bolha é um programa, tão grande quanto a memória que ele ocupa.

# The one sentence above the list of programs.
activity-list-caption = Toque em um programa para vê-lo na sala.

# Heading of the card that lists the programs.
activity-programs = Programas

# Column heading of the list: the name of each program.
activity-column-program = Programa

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Memória

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Processador

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Programas, cada um tão grande quanto a memória que ocupa

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = O que este computador está fazendo agora. Cada programa é uma bolha no círculo grande, tão grande quanto a memória que ele ocupa; quanto mais cheio o círculo, mais cheia a memória.

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = Um programa que está trabalhando brilha e pulsa, mais rápido e mais forte quanto mais trabalha: uma cor fria é pouco trabalho, laranja é trabalho pesado e vermelho é no limite. Em volta da borda há uma barra para cada thread do processador, que cresce e esquenta conforme fica mais ocupada; quando todas estão longas e vermelhas, o computador está no limite e a bolha mais brilhante mostra o motivo.

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
activity-help-3 = Clique em uma bolha, ou em um programa da lista, para ver o que ele está fazendo em números simples e para encerrá-lo se parou de responder: o botão de parar, pressionado duas vezes, pede que ele feche, e o que não foi salvo nele pode ser perdido. Duas bolhas cinzentas e discretas guardam o resto: Área de trabalho Trindle, que é o compositor que desenha a tela, o anel, a aba e o papel de parede, e Linux e outros programas, que é o próprio Linux e os programas que cuidam do computador.

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = A memória é contada do modo justo, como ferramentas como o smem a contam: a memória que dois programas compartilham é dividida entre eles, e a memória movida para o arquivo de swap é incluída, então nada é contado duas vezes nem fica de fora. Os cartões ao lado da imagem mostram o quanto estão cheios a memória, o processador, o disco e o arquivo de swap; um anel fica laranja, depois vermelho, só quando algum está quase cheio.

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = Agora

# The sentence at the top while the first look is being taken.
activity-looking = Olhando…

# The sentence at the top when the computer could not be read at all.
activity-unread = Não foi possível ler o computador agora.

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Seu computador está no limite: { $a } está usando { $threads }.

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Seu computador está no limite: { $a } e { $b } estão usando { $threads }.

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Seu computador está no limite em { $threads }; { $a } é o mais ocupado.

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Seu computador está no limite em { $threads }.

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = A memória está quase cheia; { $b } é quem mais ocupa.

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = A memória está quase cheia.

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = O computador está funcionando tranquilo; { $a } é o mais ocupado.

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = O computador está ocupado.

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = O computador está funcionando tranquilo.

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = { $a } está fazendo a maior parte do trabalho agora.

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } e { $b } estão mantendo o computador ocupado.

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } está trabalhando muito, mas o computador tem folga de sobra.

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = Tudo calmo, mas a memória está enchendo; { $b } é quem mais ocupa.

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = Tudo calmo: nada está trabalhando muito.

# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = todas as { $n ->
        [one] { $n } thread
       *[other] { $n } threads
        [many] { $n } threads
    }

# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = menos de 1 de { $n ->
        [one] { $n } thread
       *[other] { $n } threads
        [many] { $n } threads
    }

# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = cerca de { $n } de { $total ->
        [one] { $total } thread
       *[other] { $total } threads
        [many] { $total } threads
    }

# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = de { $n ->
        [one] { $n } thread
       *[other] { $n } threads
        [many] { $n } threads
    }

# A program's name when it has none that can be read.
activity-program-unnamed = Um programa

# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = menos de 1%

# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = menos de 1 MB

# How a thread is described in its tip when it is resting.
activity-resting = em repouso

# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = { $percent } ocupado

# How hard a program is working, when it is resting.
activity-effort-resting = Em repouso

# How hard a program is working, when it is working a little.
activity-effort-little = Trabalhando um pouco

# How hard a program is working, when it is working flat out.
activity-effort-flat-out = Trabalhando no limite

# How hard a program is working, when it is working hard.
activity-effort-hard = Trabalhando muito

# How long something has been running, when it is under a minute.
activity-under-a-minute = Menos de um minuto

# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
        [one] 1 minuto
       *[other] { $n } minutos
        [many] { $n } minutos
    }

# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
        [one] 1 hora
       *[other] { $n } horas
        [many] { $n } horas
    }

# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
        [one] 1 dia
       *[other] { $n } dias
        [many] { $n } dias
    }

# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }

# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = Quem mais trabalha

# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }

# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work }, e com mais memória

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = Quem mais ocupa memória

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = Próximo em memória

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Ocupando { $amount }, { $how }

# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = { $share } da memória

# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = { $amount } de memória livre

# The name of a program's bubble when it is Linux itself.
activity-linux-itself = O próprio Linux

# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = O próprio Trindle

# The name of the bubble for Trindle's own desktop: the compositor, the ring, the tab and the wallpaper.
activity-trindle-desktop = Área de trabalho Trindle

# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux e outros programas

# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, cartão aberto

# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }

# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = Thread { $n }: { $busy }

# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = Mais sobre { $name }

# Words in a card when a program's figures have gone (it has closed).
activity-closed = Fechado

# Button on a card that closes it.
activity-close = Fechar

# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
activity-stop-again = Clique de novo para encerrar { $title }

# Tip of the stop button on a card. $title is the program.
activity-stop = Encerrar { $title }

# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 parte

# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n } partes separadas

# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = Em execução há

# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = Feito de

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Ainda não foi possível ler.

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } em uso; o próprio Trindle ocupa { $ours }.

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = Quão ocupadas as { $threads } estiveram nos últimos dois minutos.

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } usados; { $free } ainda livres.

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = de { $amount }

# Name of the disk card.
activity-disk = Disco

# Name of the swap file card.
activity-swap-file = Arquivo de swap

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } em uso; o resto é espaço para hibernar.

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Temperatura

# Sentence under the temperature figure.
activity-temperature-sentence = Quão quente o processador está agora.

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = de 100°C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = Este computador não tem arquivo de swap, então nada é movido para fora da memória.



# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = Encerrado à força: { $title } não respondeu

