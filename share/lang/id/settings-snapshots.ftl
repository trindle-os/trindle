
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Snapshot

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = snapshot snapshots pulihkan batalkan kembalikan cadangan backup pembaruan rusak btrfs snapper menu boot grub versi sebelumnya lebih lama file terhapus salinan restore undo rollback

# Help line at the foot of the Snapshots page.
settings-snap-help = Sistem seperti sebelum dan sesudah setiap perubahan perangkat lunak, dan setiap kali Anda membuatnya. Memulihkan satu snapshot mengembalikan seluruh sistem ke keadaan itu mulai dari pengaktifan berikutnya; file Anda sendiri dibiarkan apa adanya jika folder Home berada di bagian disk tersendiri, seperti yang dibuat oleh pemasang Trindle. Jika komputer tidak mau menyala, pilih snapshot Trindle di menu boot. Versi sebelumnya menyimpan salinan file milik setiap orang setiap hari, terpisah dari snapshot sistem.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Salinan tidak dibuat: disk hampir penuh.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = hari ini pukul { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = kemarin pukul { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = pada { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = Komputer tidak mau dimulai ulang. Mulai ulang dari ring.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Membuat snapshot

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Membebaskan...

# Said beside the set aside line once the space is freed.
settings-snap-freed = Dibebaskan

# SAFETY: needs a human review before it ships
# Said beside a snapshot while it is restored.
settings-snap-restoring = Memulihkan

# SAFETY: needs a human review before it ships
# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = Sistem adalah snapshot { $n } mulai dari pengaktifan berikutnya. Apa pun yang berubah sejak itu disisihkan, tidak hilang.

# SAFETY: needs a human review before it ships
# Said beside a snapshot while it is deleted.
settings-snap-deleting = Menghapus

# SAFETY: needs a human review before it ships
# Said beside a snapshot once it is deleted.
settings-snap-deleted = Dihapus

# SAFETY: needs a human review before it ships
# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = Menunggu kata sandi...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Salinan dibuat setiap hari

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Tidak ada salinan baru dibuat

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Disimpan

# SAFETY: needs a human review before it ships
# Said beside the earlier copies while they are cleared.
settings-snap-clearing = Membersihkan

# SAFETY: needs a human review before it ships
# Said beside the earlier copies once they are cleared.
settings-snap-cleared = Dibersihkan

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Belum ada salinan
       *[other] { $n } salinan
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = Salinan pertama dibuat dalam sehari.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Memakan { $size }, diukur { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Ruang yang dipakai diukur sekali sehari.

# SAFETY: needs a human review before it ships
# Tip of the button that clears all earlier copies, and its label while working.
settings-snap-clear-all = Bersihkan semuanya sekarang

# Heading of the card about earlier versions of files.
settings-snap-earlier = Versi sebelumnya

# SAFETY: needs a human review before it ships
# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle dapat menyimpan salinan file semua orang sebagaimana adanya setiap hari, sehingga versi file yang lebih lama, atau yang terhapus karena salah, dapat dibuka atau dikembalikan dari Folders. Setiap orang hanya dapat membaca dalam salinan apa yang dapat dibacanya pada hari itu, jadi folder Home yang pribadi tetap pribadi; file yang saat itu terbuka bagi orang lain tetap terbuka bagi mereka dalam salinan itu sampai salinannya hilang. Salinan ini terpisah dari snapshot sistem: memulihkan sistem tidak pernah mengubah file Anda, dan salinan ini tidak pernah mengubah sistem. Harganya adalah ruang: file yang Anda hapus atau ubah tetap memakan ruangnya sampai salinan terakhir yang menyimpannya hilang, jadi menghapus file tidak langsung membebaskan disk. Membersihkan salinan membebaskannya sekarang, untuk semua orang di komputer ini, dan tidak dapat dibatalkan. Tidak ada salinan yang dibuat saat disk hampir penuh. Mengubah semua ini memerlukan kata sandi administrator.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Simpan versi file Anda sebelumnya

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = File yang dihapus tetap ada selama { $time } sebelum ruangnya dibebaskan.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Simpan salinan tiap hari selama

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Berapa lama salinan tiap hari disimpan

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Memeriksa snapshot…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Snapshot tidak dapat dibaca.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Dimulai dari snapshot { $n }, jadi apa pun yang dilakukan sekarang tidak disimpan.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
       *[other] { $n } snapshot untuk kembali, yang terbaru dari { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Belum ada snapshot. Satu dibuat sebelum setiap perubahan perangkat lunak.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, disimpan sampai dihapus

# SAFETY: needs a human review before it ships
# Tip of the button that restores the system to a snapshot, and its label while working.
settings-snap-restore-this = Pulihkan sistem ke ini

# SAFETY: needs a human review before it ships
# Tip of the button that deletes a snapshot.
settings-snap-delete-this = Hapus snapshot ini

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Mulai ulang sekarang

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Belum disiapkan.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Pasang ulang paket trindle.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Snapshot belum disiapkan. Memasang ulang paket trindle akan menyiapkannya: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = jenis lain

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Snapshot memerlukan Btrfs; komputer ini menggunakan { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Pilih Btrfs saat memasang Debian.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Snapshot memerlukan sistem di Btrfs, dan komputer ini menggunakan { $fs }. Pilih Btrfs untuk sistem saat memasang Debian agar memilikinya.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Tidak ada snapshot di komputer ini.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Tidak ada snapshot

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Tidak ada snapshot di komputer ini

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Untuk memilikinya

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Memeriksa…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Snapshot tidak dapat dibaca

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Belum ada

# Under the None yet heading.
settings-snap-none-line-sub = Satu dibuat sebelum setiap perubahan perangkat lunak.

# SAFETY: needs a human review before it ships
# Help text of the Snapshots card (the information button).
settings-snap-list-about = Setiap snapshot dapat memulihkan seluruh sistem ke keadaan saat itu, mulai dari pengaktifan berikutnya, atau dihapus. Keduanya meminta ditekan dua kali dan memerlukan kata sandi administrator. Setelah pemulihan, tombol mulai ulang muncul di sampingnya.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Buat snapshot sekarang

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Tidak saat dimulai dari snapshot.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Disimpan sampai Anda menghapusnya.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [1] Satu sistem disisihkan oleh pemulihan
       *[other] { $n } sistem disisihkan oleh pemulihan
    }

# Under the set aside heading.
settings-snap-free-space = Bebaskan ruangnya setelah semuanya baik-baik saja.

# SAFETY: needs a human review before it ships
# Tip of the button that frees the space the set aside systems use.
settings-snap-free-them = Bebaskan ruang yang dipakainya

# Heading of the card for taking a snapshot.
settings-snap-take-one = Buat satu

# Help text of the Take one card (the information button).
settings-snap-take-about = Buat snapshot sebelum mencoba sesuatu yang mungkin ingin Anda batalkan; snapshot yang Anda buat disimpan sampai Anda menghapusnya. Saat komputer dimulai dari snapshot, tidak ada yang dibuat akan tersimpan, jadi mulai ulang secara normal terlebih dahulu. Setiap pemulihan menyisihkan sistem sebelumnya, jadi tidak ada yang hilang; bebaskan ruang yang dipakainya setelah semuanya baik-baik saja.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Dimulai dari snapshot { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Apa pun yang dilakukan sekarang tidak disimpan.

# SAFETY: needs a human review before it ships
# Heading of a line on the Before restoring card.
settings-snap-files-too = File Anda juga ada di dalam snapshot

# SAFETY: needs a human review before it ships
# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Pemulihan juga mengembalikan file Anda.

# Heading of the card about what to know before restoring.
settings-snap-before = Sebelum memulihkan

# SAFETY: needs a human review before it ships
# Help text of the Before restoring card (the information button).
settings-snap-before-about = Saat dimulai dari snapshot, apa pun yang dilakukan sekarang tidak disimpan: jika beginilah sistem seharusnya, tekan pulihkan pada snapshot itu di daftar, lalu mulai ulang. Di komputer ini folder Home mungkin bagian dari sistem, dan jika demikian memulihkan snapshot juga mengembalikan file Anda sendiri ke keadaan saat itu, jadi salin dulu semua yang baru ke tempat yang aman.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = File sistem terkunci

# Under the System files locked switch.
settings-snap-locked-sub = Hanya pemasangan dan pembaruan yang membuka kuncinya.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Terkunci

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Tidak terkunci

