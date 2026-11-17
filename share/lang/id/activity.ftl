# machine translated (Indonesian), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = Aktivitas

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = Tentang halaman ini

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = Coba lagi

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Trindle tidak dapat memeriksa aktivitas komputer.

# In place of the room when no program is running.
activity-no-programs = Belum ada program yang berjalan.

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = Trindle tidak dapat menampilkan program sebelum bisa memeriksa aktivitas komputer.

# Heading of the card that lists the programs.
activity-programs = Program

# Column heading of the list: the name of each program.
activity-column-program = Program

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = Memori

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = Prosesor

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = Gelembung program berukuran menurut pemakaian memori

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = Ini menunjukkan apa yang sedang dikerjakan komputer Anda sekarang. Setiap program tampil sebagai gelembung di dalam lingkaran. Gelembung yang lebih besar berarti memori yang dipakai lebih banyak. Makin penuh lingkaran, makin banyak memori yang terpakai.

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = Gelembung program bersinar dan berdenyut saat program itu bekerja. Denyut yang lebih cepat dan terang berarti kerja lebih banyak. Warna sejuk berarti sedikit kerja, oranye berarti pemakaian berat, dan merah berarti pemakaian penuh. Setiap batang di sekeliling tepi menunjukkan satu thread, yaitu bagian prosesor yang menangani pekerjaan. Batang yang lebih panjang dan warna yang lebih hangat berarti kerja lebih banyak. Bila semua batang panjang dan merah, komputer bekerja sekeras mungkin. Gelembung paling terang menunjukkan program mana yang paling banyak bekerja.

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
# SAFETY: needs a human review before it ships
activity-help-3 = Klik gelembung, atau program di daftar, untuk melihat apa yang dikerjakannya dalam angka sederhana. Anda juga bisa menghentikan program yang sudah tidak menjawab. Tekan tombol berhenti dua kali dan Trindle meminta program itu menutup diri. Apa pun yang belum Anda simpan di dalamnya mungkin hilang. Dua gelembung abu-abu menampilkan semua yang lain. Trindle desktop adalah bagian yang menggambar layar, cincin, tab, dan gambar latar. Linux dan program lain adalah Linux sendiri dan program yang mengurus komputer.

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = Trindle menghitung memori dengan cermat, dengan cara yang sama seperti alat semacam smem. Memori yang dipakai bersama oleh dua program dibagi di antara keduanya, dan memori yang dipindahkan ke berkas swap ikut dihitung, jadi tidak ada yang dihitung dua kali atau terlewat. Kartu di samping gambar menunjukkan seberapa penuh memori, prosesor, disk, dan berkas swap. Cincin hanya berubah oranye, lalu merah, saat hampir penuh.

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = Saat ini

# The sentence at the top while the first look is being taken.
activity-looking = Memeriksa…

# The sentence at the top when the computer could not be read at all.
activity-unread = Trindle tidak dapat memeriksa komputer.

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = Komputer Anda bekerja sekeras mungkin: { $a } memakai { $threads }.

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = Komputer Anda bekerja sekeras mungkin: { $a } dan { $b } memakai { $threads }.

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = Komputer Anda bekerja sekeras mungkin pada { $threads }. { $a } yang paling sibuk.

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = Komputer Anda bekerja sekeras mungkin pada { $threads }.

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = Memori hampir penuh. { $b } memakai paling banyak.

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = Memori hampir penuh.

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = Komputer tidak sibuk. { $a } paling banyak bekerja.

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = Komputer sedang sibuk.

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = Komputer tidak sibuk.

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = { $a } mengerjakan sebagian besar pekerjaan saat ini.

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } dan { $b } membuat komputer sibuk.

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } bekerja keras, tetapi komputer masih punya banyak sisa tenaga.

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = Komputer tidak banyak bekerja, tetapi pemakaian memori naik. { $b } memakai paling banyak.

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = Tidak ada program yang bekerja keras.

# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = semua { $n ->
       *[other] { $n } thread
    }

# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = kurang dari 1 dari { $n ->
       *[other] { $n } thread
    }

# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = sekitar { $n } dari { $total ->
       *[other] { $total } thread
    }

# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = dari { $n ->
       *[other] { $n } thread
    }

# A program's name when it has none that can be read.
activity-program-unnamed = Sebuah program

# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = di bawah 1%

# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = di bawah 1 MB

# How a thread is described in its tip when it is resting.
activity-resting = menganggur

# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = sibuk { $percent }

# How hard a program is working, when it is resting.
activity-effort-resting = Menganggur

# How hard a program is working, when it is working a little.
activity-effort-little = Bekerja sedikit

# How hard a program is working, when it is working flat out.
activity-effort-flat-out = Bekerja penuh

# How hard a program is working, when it is working hard.
activity-effort-hard = Bekerja keras

# How long something has been running, when it is under a minute.
activity-under-a-minute = Kurang dari semenit

# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
       *[other] { $n } menit
    }

# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
       *[other] { $n } jam
    }

# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
       *[other] { $n } hari
    }

# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }

# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = Paling keras bekerja

# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }

# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work }, dan memakai memori paling banyak

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = Memakai memori terbanyak

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = Memori terbanyak berikutnya

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = Memakai { $amount }, { $how }

# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = { $share } dari memori

# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = { $amount } memori kosong

# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux sendiri

# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle sendiri

# The name of the bubble for Trindle's own desktop: the compositor, the ring, the tab and the wallpaper.
activity-trindle-desktop = Trindle desktop

# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux dan program lain

# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, kartu terbuka

# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }

# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = Thread { $n }: { $busy }

# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = Selengkapnya tentang { $name }

# Words in a card when a program's figures have gone (it has closed).
activity-closed = Tertutup

# Button on a card that closes it.
activity-close = Tutup

# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop-again = Klik lagi untuk menghentikan { $title }

# Tip of the stop button on a card. $title is the program.
# SAFETY: needs a human review before it ships
activity-stop = Hentikan { $title }

# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 bagian

# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n } bagian terpisah

# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = Berjalan selama

# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = Bagian

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = Belum dapat membaca ini.

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } terpakai. Trindle sendiri memakai { $ours }.

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = Seberapa sibuk { $threads } dalam dua menit terakhir.

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } terpakai dan { $free } kosong.

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = dari { $amount }

# Name of the disk card.
activity-disk = Disk

# Name of the swap file card.
activity-swap-file = Berkas swap

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } terpakai.

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = Suhu

# Sentence under the temperature figure.
activity-temperature-sentence = Suhu prosesor

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = dari 100°C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = Komputer ini tidak punya berkas swap, jadi tidak ada yang dipindahkan keluar dari memori.


# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = { $title } tidak menjawab, jadi Trindle memaksanya keluar
