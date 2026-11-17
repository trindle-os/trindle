## Activity (src/activity): what the computer is doing right now, drawn as a room of bubbles, with four cards of figures.

## The window and its page (src/activity/mod.rs)

# Window title, and the heading at the top of the Activity window.
activity-title = Activity
# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = About this page
# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Try again
# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Trindle couldn't check the computer's activity.
# In place of the room when no program is running.
activity-no-programs = No programs are running yet.
# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = Trindle can't show any programs until it can check the computer's activity.
# Heading of the card that lists the programs.
activity-programs = Programs
# Column heading of the list: the name of each program.
activity-column-program = Program
# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Memory
# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Processor
# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Program bubbles sized by memory use

## The longer words behind the i, one paragraph at a time (src/activity/mod.rs)

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = This shows what your computer is doing right now. Each program appears as a bubble inside the circle. Bigger bubbles mean more memory use. The fuller the circle, the more memory is in use.
# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = A program's bubble glows and pulses while it is working. Faster, brighter pulses mean more work. Cool colours mean little work, orange means heavy use, and red means full use. Each bar around the edge shows one thread, a part of the processor that handles work. Longer bars and warmer colours mean more work. When all the bars are long and red, the computer is working as hard as it can. The brightest bubble shows which program is doing the most work.
# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
activity-help-3 = Click a bubble, or a program in the list, to see what it is doing in plain figures. You can also stop a program that has stopped responding. Press the stop button twice and Trindle asks the program to close. Anything you haven't saved in it may be lost. The two grey bubbles show everything else. Trindle desktop is the part that draws the screen, the ring, the tab and the wallpaper. Linux and other programs is Linux itself and the programs that look after the computer.
# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = Trindle counts memory carefully, the same way tools like smem do. Memory that two programs share is split between them, and memory moved out to the swap file is included, so nothing is counted twice or left out. The cards beside the picture show how full the memory, the processor, the disk and the swap file are. A ring only turns orange, then red, when it is nearly full.

## The sentence at the top (src/activity/mod.rs)

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = Right now
# The sentence at the top while the first look is being taken.
activity-looking = Looking…
# The sentence at the top when the computer could not be read at all.
activity-unread = Trindle couldn't check the computer.
# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Your computer is working as hard as it can: { $a } is using { $threads }.
# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Your computer is working as hard as it can: { $a } and { $b } are using { $threads }.
# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Your computer is working as hard as it can on { $threads }. { $a } is the busiest.
# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Your computer is working as hard as it can on { $threads }.
# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = Memory is nearly full. { $b } is using the most.
# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = Memory is nearly full.
# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = The computer isn't busy. { $a } is doing the most work.
# The sentence at the top when the computer is busy and nothing is named.
activity-busy = The computer is busy.
# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = The computer isn't busy.
# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = { $a } is doing most of the work right now.
# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } and { $b } are keeping the computer busy.
# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } is working hard, but the computer has plenty to spare.
# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = The computer isn't doing much work, but memory use is rising. { $b } is using the most.
# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = No programs are working hard.
# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = all { $n ->
        [one] { $n } thread
       *[other] { $n } threads
    }
# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = less than 1 of { $n ->
        [one] { $n } thread
       *[other] { $n } threads
    }
# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = about { $n } of { $total ->
        [one] { $total } thread
       *[other] { $total } threads
    }
# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = of { $n ->
        [one] { $n } thread
       *[other] { $n } threads
    }

## Figures and words about a program (src/activity/mod.rs)

# A program's name when it has none that can be read.
activity-program-unnamed = A program
# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = under 1%
# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = under 1 MB
# How a thread is described in its tip when it is resting.
activity-resting = idle
# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = { $percent } busy
# How hard a program is working, when it is resting.
activity-effort-resting = Idle
# How hard a program is working, when it is working a little.
activity-effort-little = Working a little
# How hard a program is working, when it is working flat out.
activity-effort-flat-out = Working flat out
# How hard a program is working, when it is working hard.
activity-effort-hard = Working hard
# How long something has been running, when it is under a minute.
activity-under-a-minute = Under a minute
# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
        [one] 1 minute
       *[other] { $n } minutes
    }
# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
        [one] 1 hour
       *[other] { $n } hours
    }
# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
        [one] 1 day
       *[other] { $n } days
    }
# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }
# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = Working hardest
# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }
# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work }, and using the most memory
# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = Using the most memory
# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = Next most memory
# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Using { $amount }, { $how }
# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = { $share } of the memory
# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = { $amount } of memory free
# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux itself
# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle itself
# The name of the bubble for Trindle's own desktop: the compositor, the ring and the wallpaper.
activity-trindle-desktop = Trindle desktop
# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux and other programs
# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, card open
# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }
# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = Thread { $n }: { $busy }
# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = More about { $name }

## The card beside a picked bubble (src/activity/mod.rs)

# Words in a card when a program's figures have gone (it has closed).
activity-closed = Closed
# Button on a card that closes it.
activity-close = Close
# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
activity-stop-again = Click again to stop { $title }
# Tip of the stop button on a card. $title is the program.
activity-stop = Stop { $title }
# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = { $title } didn't respond, so Trindle forced it to quit
# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 part
# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n } separate parts
# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = Running for
# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = Parts

## The four figure cards (src/activity/rings.rs)

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Can't read this yet.
# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } in use. Trindle itself uses { $ours }.
# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = How busy { $threads } were in the last two minutes.
# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } used and { $free } free.
# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = of { $amount }
# Name of the disk card.
activity-disk = Disk
# Name of the swap file card.
activity-swap-file = Swap file
# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } in use.
# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Temperature
# Sentence under the temperature figure.
activity-temperature-sentence = Processor temperature
# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = of 100°C
# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = This computer has no swap file, so nothing is moved out of memory.
