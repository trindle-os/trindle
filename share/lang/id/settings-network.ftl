
# machine translated (Indonesian), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } dimatikan oleh sakelar atau tombol pada komputer itu sendiri. Nyalakan, lalu coba lagi.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } diblokir oleh sistem, dan blokirnya tidak dapat dilepas.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Kata sandi itu tidak diterima. Coba lagi.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Kata sandi yang tersimpan untuk jaringan ini tidak diterima. Lupakan jaringan dengan tong sampah, lalu sambungkan lagi.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Jaringan itu tidak ditemukan. Periksa namanya, dan pastikan jaringan itu dalam jangkauan.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Jaringan mengizinkan komputer ini masuk tetapi tidak memberinya alamat. Coba lagi, atau mulai ulang router.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Jaringan tidak mengizinkan komputer ini bergabung. Dekati jaringan itu, lalu coba lagi.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Kata sandi itu tidak dapat dipakai oleh jaringan Wi-Fi. Periksa, lalu coba lagi.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Jaringan tidak dapat disambungkan.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = buka dengan kata sandi Anda

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = buka dan ubah dengan kata sandi Anda

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = siapa pun di rumah dapat membuka

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = siapa pun di rumah dapat mengubah

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
settings-network-state-configuring = Mengatur

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
        [strong] Tersambung ke { $name }, sinyal kuat.
        [good] Tersambung ke { $name }, sinyal baik.
        [fair] Tersambung ke { $name }, sinyal sedang.
       *[other] Tersambung ke { $name }, sinyal lemah.
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
settings-network-top-gone = Jaringan tidak dapat dilihat: NetworkManager tidak berjalan.

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
settings-network-chip = Chipnya memerlukan Linux yang lebih baru.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Komputer ini tidak punya Wi-Fi yang berfungsi.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Mati. Nyalakan untuk bergabung ke jaringan.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Menyala, dan belum bergabung ke jaringan.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Pakai Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi dan kabel

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi menyambungkan komputer ini ke jaringan tanpa kabel. Jika dimatikan, komputer berhenti mencari jaringan dan sedikit menghemat baterai. Jaringan yang tidak menyiarkan namanya tidak pernah muncul di daftar jaringan dalam jangkauan: sambungkan di sini dengan mengetik namanya persis, termasuk huruf besar. Jika chip Wi-Fi komputer ini belum didukung driver mana pun di Linux-nya, diperlukan Linux yang lebih baru daripada yang dijalankan Trindle. Kabel jaringan punya barisnya sendiri, dengan tombol untuk menyambung atau memutuskannya.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Izinkan komputer lain tersambung

# Small line under that switch.
settings-network-share-switch-sub = Untuk berbagi folder dan printer di rumah.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Mencari folder yang dibagikan…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Folder yang dibagikan tidak dapat dibaca

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Belum dapat dijangkau

# Small line under it.
settings-network-not-reachable-sub = Orang lain tidak dapat membukanya selama ini mati.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Tidak ada folder yang dibagikan

# Small line under it.
settings-network-no-shared-sub = Klik kanan satu folder di Folders untuk membagikannya.

# Heading of the Sharing card.
settings-network-sharing = Berbagi

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Selama Izinkan komputer lain tersambung mati, tidak ada apa pun di jaringan yang dapat menjangkau komputer ini kecuali jika komputer ini yang memulai, sehingga tidak seorang pun di Wi-Fi yang sama dapat mencoba peruntungannya. Nyalakan untuk berbagi folder atau printer dengan komputer lain di rumah. Untuk membagikan folder, klik kanan di Folders dan pilih Bagikan di jaringan. Masing-masing menyebutkan siapa boleh melakukan apa. Dengan kata sandi Anda: orang lain dapat membuka file, atau membuka dan mengubahnya, dengan kata sandi berbagi Anda. Siapa pun di rumah: siapa pun di jaringan dapat membuka file, atau membuka dan mengubahnya, tanpa kata sandi. Komputer lain di rumah tidak dapat membuka satu pun selama sakelar ini mati.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Menghentikan…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Menunggu kata sandi...

# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = Berbagi diizinkan

# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = Dicegah masuk

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi tidak dapat dinyalakan.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi tidak dapat dimatikan.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Putuskan

# Tip of the button that connects a network or a cable.
settings-network-connect = Sambungkan

# Shown under a network or cable while it is being connected.
settings-network-connecting = Menyambungkan...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Tidak ada jaringan dalam jangkauan

# Small line under it.
settings-network-none-in-range-sub = Dicari lagi setiap kali halaman ini dibuka.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Jaringan dalam jangkauan

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Setiap jaringan Wi-Fi yang dapat didengar komputer ini. Saat halaman dibuka, yang sedang dipakai ada di urutan pertama, lalu yang tersimpan di sini, lalu sisanya menurut kekuatan sinyal; baris tetap di tempatnya selama halaman terbuka. Tekan tanda panah untuk bergabung; jaringan yang diamankan meminta kata sandinya saat pertama kali. Tong sampah melupakan jaringan tersimpan beserta kata sandinya. Jaringan terbuka tidak terenkripsi, jadi orang di dekat Anda dapat melihat apa yang dikirim lewat jaringan itu.

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
settings-network-forget-failed = Jaringan tidak dapat dilupakan.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Kata sandi

# Tip of the button that joins the network.
settings-network-join = Bergabung

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Batal

# Shown under a network while it is being joined.
settings-network-joining = Bergabung...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Kata sandi Wi-Fi minimal 8 karakter.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Bergabung ke jaringan tersembunyi

# Small line under it.
settings-network-hidden-sub = Ketik namanya persis, termasuk huruf besar.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Nama jaringan

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Kata sandi, jika ada

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Nama jaringan diperlukan.

# Shown under a network after it was joined.
settings-network-joined = Tergabung

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Jaringan tetap disimpan: lupakan dengan tong sampah sebelum bergabung lagi.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Jaringan tidak mau terputus.
