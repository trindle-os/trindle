
# machine translated (Indonesian), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } dimatikan oleh sakelar atau tombol pada komputer itu sendiri. Nyalakan, lalu coba lagi.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = Sistem memblokir { $radio }, dan Trindle tidak bisa melepas blokirnya.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Kata sandi itu tidak diterima. Coba lagi.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Kata sandi tersimpan untuk jaringan ini tidak diterima. Pakai tombol tong sampah untuk melupakan jaringan, lalu sambungkan lagi.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle tidak menemukan jaringan itu. Periksa namanya, dan periksa apakah jaringan itu dalam jangkauan.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Komputer ini sudah bergabung ke jaringan tetapi tidak mendapat alamat. Coba lagi, atau mulai ulang router.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Komputer ini tidak bisa bergabung ke jaringan. Dekati router, lalu coba lagi.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Itu tidak mungkin kata sandi Wi-Fi. Periksa, lalu coba lagi.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Tidak bisa bergabung ke jaringan.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = buka dengan kata sandi Anda

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = buka dan ubah dengan kata sandi Anda

# Small line of a shared folder: anyone on the local network may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = siapa pun di dekat bisa membuka

# Small line of a shared folder: anyone on the local network may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = siapa pun di dekat bisa mengubah

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = Berhenti berbagi { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Tidak dikelola

# State of a network device that cannot be used.
settings-network-state-unavailable = Tidak tersedia

# State of a network device that is not connected.
settings-network-state-disconnected = Terputus

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Bersiap

# State of a network device that is being set up.
settings-network-state-configuring = Menyiapkan

# State of a network device that needs a password.
settings-network-state-needs-password = Perlu kata sandi

# State of a network device that is getting an address.
settings-network-state-address = Mendapatkan alamat

# State of a network device whose connection is being checked.
settings-network-state-checking = Memeriksa

# State of a network device that is waiting.
settings-network-state-waiting = Menunggu

# State of a network device that is connected.
settings-network-state-connected = Tersambung

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Memutuskan

# State of a network device that failed.
settings-network-state-failed = Gagal

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Tersambung lewat kabel, dan ke { $name } lewat Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Tersambung ke { $name }. Sinyalnya kuat.
        [good] Tersambung ke { $name }. Sinyalnya baik.
        [fair] Tersambung ke { $name }. Sinyalnya sedang.
       *[other] Tersambung ke { $name }. Sinyalnya lemah.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Tersambung lewat kabel.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi mati, jadi komputer ini tidak tersambung.

# Headline of the page when not connected.
settings-network-top-none = Tidak tersambung. Pilih jaringan di bawah.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Mencari jaringan…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Trindle tidak bisa menampilkan jaringan karena NetworkManager tidak berjalan.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Bus sistem

# Name of the page in the list of settings pages and its heading.
settings-network-title = Jaringan

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi wireless internet ethernet wired password connect firewall sharing share printer folder nirkabel kabel kata sandi sambung berbagi printer folder jaringan

# Help text at the top of the page.
settings-network-help = Nyalakan atau matikan Wi-Fi, bergabung ke jaringan, lupakan jaringan yang tidak lagi diinginkan, atau izinkan komputer lain tersambung.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Sinyal kuat dari { $name }.
        [good] Sinyal baik dari { $name }.
        [fair] Sinyal sedang dari { $name }.
       *[other] Sinyal lemah dari { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi memerlukan Linux yang lebih baru.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Komputer ini tidak punya Wi-Fi yang berfungsi.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Mati. Nyalakan untuk bergabung ke jaringan.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Menyala, tetapi tidak tersambung ke jaringan.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Pakai Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi dan kabel

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi menyambungkan komputer ini ke jaringan tanpa kabel. Jika dimatikan, komputer berhenti mencari jaringan dan sedikit menghemat baterai. Jaringan yang menyembunyikan namanya tidak pernah muncul di daftar jaringan dalam jangkauan. Untuk bergabung, ketik namanya persis, termasuk huruf besar. Sebagian chip Wi-Fi memerlukan driver yang belum ada di versi Linux yang dipakai Trindle. Untuk chip itu, Anda memerlukan versi Linux yang lebih baru. Kabel jaringan punya barisnya sendiri, dengan tombol untuk menyambung atau memutuskannya.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Izinkan komputer lain tersambung

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Mencari folder yang dibagikan…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Tidak bisa membaca folder yang dibagikan

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Belum dapat dijangkau

# Small line under it.
settings-network-not-reachable-sub = Komputer lain tidak bisa membukanya selama berbagi mati.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Tidak ada folder yang dibagikan

# Heading of the Sharing card.
settings-network-sharing = Berbagi

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Selama Izinkan komputer lain tersambung mati, komputer lain di jaringan hanya bisa menjangkau komputer ini jika komputer ini menghubungi mereka lebih dulu. Ini mencegah siapa pun di Wi-Fi yang sama mencoba masuk. Nyalakan untuk berbagi folder atau printer dengan komputer lain di rumah. Untuk membagikan folder, klik kanan di Folders dan pilih Bagikan di jaringan. Setiap folder yang dibagikan menunjukkan siapa boleh melakukan apa. Dengan kata sandi Anda: orang lain dapat membuka file, atau membuka dan mengubahnya, jika mereka punya kata sandi berbagi Anda. Siapa pun di rumah: siapa pun di jaringan dapat membuka file, atau membuka dan mengubahnya, tanpa kata sandi. Selama sakelar mati, komputer lain di rumah tidak dapat membuka satu pun.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Menghentikan…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Menunggu kata sandi Anda…

# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = Berbagi aktif

# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = Diblokir

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Tidak bisa menyalakan Wi-Fi.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Tidak bisa mematikan Wi-Fi.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Putuskan

# Tip of the button that connects a network or a cable.
settings-network-connect = Sambungkan

# Shown under a network or cable while it is being connected.
settings-network-connecting = Menyambungkan…

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Tidak ada jaringan dalam jangkauan

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Jaringan dalam jangkauan

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Setiap jaringan Wi-Fi yang bisa ditangkap komputer ini. Saat Anda membuka halaman, jaringan yang sedang dipakai ada di urutan pertama, lalu jaringan tersimpan, lalu sisanya menurut kekuatan sinyal. Urutannya tidak berubah selama halaman terbuka. Tekan tanda panah untuk bergabung ke jaringan. Jaringan yang diamankan meminta kata sandinya saat pertama kali. Tombol tong sampah melupakan jaringan tersimpan beserta kata sandinya. Jaringan terbuka tidak terenkripsi, jadi orang di dekat Anda dapat melihat apa yang Anda kirim lewat jaringan itu.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Nyalakan Wi-Fi untuk melihat jaringan dalam jangkauan.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Tersimpan

# Part of a network's small line: it needs a password.
settings-network-secured = Diamankan

# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = Terbuka, tidak terenkripsi

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/dtk

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Lupakan jaringan ini

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = Tidak bisa melupakan jaringan.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Kata sandi

# Tip of the button that joins the network.
settings-network-join = Bergabung

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Batal

# Shown under a network while it is being joined.
settings-network-joining = Bergabung…

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Kata sandi Wi-Fi minimal 8 karakter.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Bergabung ke jaringan tersembunyi

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Nama jaringan

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Kata sandi, jika ada

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Ketik nama jaringan.

# Shown under a network after it was joined.
settings-network-joined = Tergabung

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Jaringan tetap disimpan. Lupakan dengan tombol tong sampah sebelum bergabung lagi.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Tidak bisa memutuskan sambungan.

# Heading of the card that lists the ports opened by hand.
settings-network-ports = Port terbuka

# Help text (i button) of the Open ports card.
settings-network-ports-about = Setiap port tertutup kecuali komputer ini menghubungi komputer lain lebih dulu. Program yang menunggu sambungan dari orang lain, seperti server game atau alat sinkronisasi, tidak dapat dijangkau sampai Anda membuka port-nya di sini. Port yang terbuka mengizinkan komputer mana pun di jaringan lokal yang sama dengan komputer ini, tetapi tidak pernah internet yang lebih luas. Di jaringan kafe, hotel, atau kantor, itu mencakup semua orang di sana. Untuk membuka atau menutup port, Anda memerlukan kata sandi administrator.

# Line in the Open ports card when no port is open.
settings-network-ports-none = Tidak ada port yang dibuka

# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = Tutup { $port }

# Heading of the line that opens a port.
settings-network-port-add = Buka sebuah port

# Placeholder in the box where the port number is typed.
settings-network-port-number = Nomor port

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Protokol

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = Keduanya

# Button that opens the typed port.
settings-network-port-open = Buka

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = Ketik angka dari 1 sampai 65535.

# Red words when the port is already open.
settings-network-port-already = Sudah terbuka.

# Said under the line once the port is open.
settings-network-port-opened = Terbuka

# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = Perlu nama pengguna

# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = Jaringan ini memerlukan nama pengguna selain kata sandi. Trindle belum dapat bergabung ke jaringan seperti itu.
