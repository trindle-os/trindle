
# machine translated (Indonesian), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Belum ada printer. Printer yang ditemukan Trindle di jaringan Anda tercantum di bawah.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } dijeda, jadi untuk sementara tidak ada yang akan dicetak di sana.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } sedang mencetak.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Siap mencetak di { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } berstatus { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } sudah disiapkan, tetapi Anda belum memilih printer utama.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } printer sudah disiapkan, tetapi Anda belum memilih printer utama.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Pekerjaan itu sudah tidak ada.

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Dibatalkan

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = Pemeriksaan gagal.

# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = Layanan pencetakan tidak berjalan atau tidak dapat dijangkau.

# Words in the ring while the network is searched for printers.
settings-printers-searching = Mencari

# Words in the ring when the search is finished.
settings-printers-search-done = Selesai

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Sekarang utama

# Shown beside a printer after a test page was sent.
settings-printers-sent = Terkirim

# SAFETY: needs a human review before it ships
# Shown beside a printer after it was removed.
settings-printers-removed = Dihapus

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Ketik alamat printer terlebih dahulu.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Menambahkan…

# Shown beside a printer or the address line after it was added.
settings-printers-added = Ditambahkan

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Mencari drivernya…

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Pekerjaan { $number } di { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# SAFETY: needs a human review before it ships
# Tip of the cross button that cancels a print job.
settings-printers-cancel-job = Batalkan pekerjaan ini

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Pekerjaan cetak

# SAFETY: needs a human review before it ships
# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = Semua pekerjaan yang menunggu dicetak, di printer mana pun. Trindle memperbarui daftar setiap beberapa detik selama halaman ini terbuka. Halaman yang sudah tercetak tidak bisa dibatalkan.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS tidak merespons

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Tidak bisa membaca antrean.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Mencari…

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Tidak ada yang menunggu

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Dijeda

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Siaga

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Mencetak

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Utama, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Jadikan printer utama

# Tip of the button that prints a test page.
settings-printers-test-page = Cetak halaman tes

# SAFETY: needs a human review before it ships
# Tip of the bin button that removes a printer.
settings-printers-remove = Hapus printer ini

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Pencarian memerlukan cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Untuk memasangnya, jalankan: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Tambahkan printer ini

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Tidak ada yang baru ditemukan

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Tambahkan

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Tambahkan dengan alamat

# Heading of the card for adding printers.
settings-printers-add-card = Tambah printer

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trindle lebih dulu mencari printer yang mengumumkan diri, lalu memeriksa setiap perangkat di jaringan secara bergiliran. Printer harus menyala dan berada di jaringan yang sama. Sebagian printer tidak mengumumkan diri, dan sebagian jaringan tidak meneruskan pengumuman. Jika begitu, tambahkan printer dengan alamatnya, misalnya 192.168.1.50. Alamat ini dapat dilihat di layar printer, pada pengaturan jaringan.

# Name of the page in the list of settings pages.
settings-printers-title = Printer

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = printer cetak pemindai scanner print

# Help text at the top of the page.
settings-printers-help = Trindle menemukan printer di jaringan Anda sendiri. Untuk memakai satu printer, tekan Tambahkan printer ini di sampingnya.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = Pencetakan belum terpasang di komputer ini.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Mencari…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Trindle tidak bisa menjangkau layanan pencetakan, jadi saat ini tidak ada yang dapat dicetak.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Pencetakan

# Help text (i button) of that card.
settings-printers-printing-about = Pencetakan memerlukan CUPS, yang tidak ada di komputer ini. Setelah terpasang, Trindle menemukan printer di jaringan Anda sendiri.

# Line in that card.
settings-printers-unavailable = Pencetakan tidak tersedia

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Pasang dengan: sudo apt install cups

# Line in that card.
settings-printers-then-found = Menemukan printer

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Layanan pencetakan

# Line in the Printers card when there are none.
settings-printers-none = Belum ada printer

# Heading of the card that lists the printers.
settings-printers-card = Printer

# SAFETY: needs a human review before it ships
# Help text (i button) of the Printers card.
settings-printers-card-about = Semua printer yang disiapkan di komputer ini. Akun Anda harus berada dalam grup lpadmin untuk membuat perubahan.

# Red words under a page range box that cannot be used.
printers-bad-range = Contoh: 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = 1 sampai 99 salinan.

# Name of the choice to print on one side of the paper.
printers-sides-one = Satu sisi

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Dua sisi, tepi panjang

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Dua sisi, tepi pendek

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
printers-orientation-portrait = Tegak

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Menyamping

# SAFETY: needs a human review before it ships
# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = Anda tidak diizinkan melakukan ini. Akun Anda harus berada dalam grup lpadmin. Setelah itu, keluar lalu masuk kembali.

# SAFETY: needs a human review before it ships
# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS menolak.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = Nama printer itu tidak diizinkan.

# Red words when printing is not installed on the computer.
printers-not-installed = Pencetakan belum terpasang di komputer ini.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } tidak lagi disiapkan.

# Red words when the printer chosen is paused.
printers-paused = { $name } dijeda.

# Red words when sending the document to CUPS took too long.
printers-too-slow = Pengiriman dokumen terlalu lama.

# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = Trindle tidak dapat membatalkan pekerjaan cetak.

# Red words when a printer could not be made the default.
settings-printers-not-default = Trindle tidak dapat menjadikannya printer utama.

# Red words when the test page could not be sent.
settings-printers-not-sent = Trindle tidak dapat mengirim halaman uji.

# Red words when a printer could not be removed.
settings-printers-not-removed = Trindle tidak dapat menghapus printer.

# Red words when a printer could not be added.
settings-printers-not-added = Trindle tidak dapat menambah printer.

# Red words when the list of printers could not be read.
settings-printers-not-read = Trindle tidak dapat membaca daftar printer.
