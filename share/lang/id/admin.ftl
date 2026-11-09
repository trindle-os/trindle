
# machine translated (Indonesian), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Nama login itu tidak valid.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Nama login itu terlalu panjang.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = Tidak ada pengguna bernama { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } adalah akun sistem dan tidak diubah.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = Kata sandi tidak diberikan.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
       *[other] Kata sandi minimal { $n } karakter.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = Kata sandi tidak boleh berisi baris baru.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } sudah ada.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } adalah satu-satunya administrator, jadi tetap menjadi administrator.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Anda tidak dapat menghapus akun Anda sendiri.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = { $name } sudah dihapus, tetapi gambarnya masih tersimpan untuk layar login: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Gambar itu terlalu besar.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Itu bukan gambar buatan Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Folder gambar tidak dapat dibuka untuk layar login: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Itu bukan nama bahasa.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Daftar bahasa Debian tidak dapat dibaca: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } bukan bahasa yang dikenal Debian.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Nama komputer berisi huruf, angka, dan tanda hubung, hingga 63 karakter, dan tidak diawali atau diakhiri tanda hubung.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = Batas aktif untuk saat ini, tetapi akan hilang saat komputer dimatikan: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Pengisian daya penuh untuk saat ini, tetapi batas akan kembali saat dinyalakan berikutnya: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Komputer lain masih dapat terhubung, karena sakelarnya tidak dapat dimatikan: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Itu bukan id plugin.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Orang itu tidak dikenal.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Folder orang itu tidak dikenal.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Daftar plugin yang diizinkan tidak dapat disimpan: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Plugin itu tidak ditemukan.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Tidak ada yang diperbarui: versi Trindle di komputer ini tidak dapat diketahui, jadi pembaruan Trindle sendiri tidak dapat dikecualikan.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Memasang pembaruan

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Merapikan

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Memasang Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Itu bukan nama paket.

# Red words when the package to remove is not installed.
admin-package-not-installed = Tidak dilakukan: paket itu tidak terpasang.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Itu bukan perangkat yang dikenal fwupd.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Tidak dapat diketahui siapa yang meminta Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Pemblokiran tidak mulai tepat waktu.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Pemblokiran belum sepenuhnya mati: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = { $path } tidak dapat dikembalikan: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Itu bukan nama situs.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Server itu tidak ada.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } berhenti sebelum menerima semuanya, jadi ini tidak dapat dianggap selesai: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } gagal.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-file-not-removed = { $path } tidak dapat dihapus: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Tidak ada yang diubah, karena { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Mengembalikan keadaan semula juga gagal: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Itu bukan lama waktu penyimpanan versi sebelumnya.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Folder snapshot tidak ada.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Ada sesuatu yang bukan folder di tempat snapshot disimpan.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Tidak dapat diketahui siapa yang meminta.

# Red words when a step that must go through the administrator's password was started another way.
# SAFETY: needs a human review before it ships
admin-pkexec-only = Ini hanya dijalankan melalui pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Itu bukan nama komputer.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Itu bukan nama folder bersama.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Itu bukan alamat komputer.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Itu bukan path folder bersama.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = Hanya folder bersama milik Anda sendiri yang dapat ditutup di sini.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = Tidak ada folder bersama yang terbuka di sana.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } tidak seperti yang ditinggalkan Trindle, jadi tidak ada yang dipasang di sana.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } tidak seperti yang ditinggalkan Trindle.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Terlalu banyak folder bersama yang terbuka.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } belum terpasang. Program ini ada di paket Debian { $package }.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Login tidak dapat dibaca.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Apa yang terpasang di komputer ini tidak dapat dibaca: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Masih ada program yang membuka sesuatu di dalamnya. Tutup program itu dulu.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Komputer itu tidak menjawab. Folder akan ditutup saat Anda keluar.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Itu bukan id pengguna.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = Grup sambashare tidak dapat dibuat.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Mengunduh Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Menyiapkan berbagi

# Red words in Settings when a sharing password is set before sharing is set up.
# SAFETY: needs a human review before it ships
admin-sharing-not-set-up = Berbagi belum disiapkan. Bagikan sebuah folder dulu.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = Komputer ini masih terlihat oleh yang lain di jaringan: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Itu bukan nomor snapshot.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Dibuat di Pengaturan

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Tidak dapat diketahui sistem berada di disk yang mana.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = Sistem tidak berada di subvolume Btrfs tersendiri, jadi tidak dapat ditukar dengan snapshot.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Kunci penandatanganan tidak dapat diunduh dari { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Yang dikembalikan alamat itu bukan kunci penandatanganan.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Tidak ada yang diubah, karena { $path } tidak dapat dibaca: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Mengunduh kunci penandatanganannya

# Second line of the progress while a source is added.
admin-step-adding-source = Menambahkannya dan membaca daftarnya

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Sumber itu tidak dapat digunakan. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Sumber itu tidak dapat digunakan, jadi dihapus lagi. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = { $user } belum dapat menyalakan atau mematikannya: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = tidak dapat dimulai untuk { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = { $user } tidak dapat diberi izin memakai mesin virtual, yang dibutuhkan Cowork: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = { $user } tidak dapat diberi izin memakainya tanpa kata sandi: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } sudah terpasang, tetapi penyiapannya belum selesai: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Memasangnya

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Itu bukan salah satu sumber siap pakai.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } tidak dibuat untuk jenis komputer ini ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Menyiapkannya

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Alamat harus diawali https:// dan tidak berisi yang lain.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = Suite dan komponen harus berupa kata biasa.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Itu bukan nama paket.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Menghapusnya

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Membaca daftar lagi

# Red words in Apps when the source to remove was not added by Trindle.
# SAFETY: needs a human review before it ships
admin-source-not-ours = Hanya sumber yang ditambahkan Trindle yang dihapus di sini.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = Sumber itu masih ada: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = Snapshot { $n } tidak ada di disk.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Tidak ada yang diubah: snapshot tidak dapat disiapkan. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Sistem saat ini tidak dapat disisihkan: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = sistem saat ini sekarang bernama { $aside } dan bukan { $sub }, dan komputer tidak akan menyala sampai diberi nama { $sub } lagi: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Snapshot tidak dapat dipindahkan: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Snapshot tidak dapat dipasang di tempatnya: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Daftar sistem yang disisihkan tidak dapat diperbarui: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
# SAFETY: needs a human review before it ships
admin-restore-not-removed = Tidak dapat dihapus: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Tidak ada yang membutuhkan ini lagi, jadi dihapus: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Tidak semua pembaruan terpasang. Ditahan: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } mati: drivernya ({ $driver }, DKMS) belum bisa dibangun di kernel baru.

admin-driver-off = Sebuah driver ({ $driver }, DKMS) belum bisa dibangun di kernel baru, jadi fungsinya mati.

# Said once after the drivers that are off.
admin-driver-off-rest = Kernel dan sisa pembaruan sudah terpasang.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = kamera virtual

admin-language-in-use = { $code } adalah bahasa saat komputer ini menyala, jadi bahasa itu tetap ada.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Komputer ini tidak punya pelambatan split-lock yang bisa dimatikan.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Drive itu tidak ditemukan, atau berisi sistem.

# The drive could not be closed where it was open.
admin-games-drive-busy = Drive sedang dipakai. Tutup apa yang terbuka di sana, lalu coba lagi: { $why }

# Another software change has the read-only system open.
admin-system-busy = Perubahan perangkat lunak lain sedang berjalan. Coba lagi setelah selesai.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Tidak ada yang diubah: sistem tidak dapat dibuka kuncinya: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Sistem tidak dapat dikunci kembali. Sistem akan dikunci saat start berikutnya: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = /usr/local tidak dapat dibiarkan bisa ditulis: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Hanya root yang dapat menjalankan ini, dan bukan lewat pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Memerlukan Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = Snapshot sudah terpasang dan komputer akan menyala darinya. Pembersihan sistem lama belum selesai dan akan dicoba lagi pada saat menyala berikutnya: { $why }

