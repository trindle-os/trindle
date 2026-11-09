
# machine translated (Indonesian), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Baterai { $pct }%

# Line under the low battery warning's title.
ring-battery-plug-in = Segera colokkan pengisi daya.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = Mengisi daya

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Pengisi daya dicabut

# Line under the title when the charger comes out.
ring-battery-running = Memakai baterai, sisa { $pct }%

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 menit

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 jam

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 jam

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = Besok pagi

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = Tunda selama 10 menit

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = Tunda selama 1 jam

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = Tunda selama 6 jam

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Tunda sampai besok pagi

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = Kembali ke pesan

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Lepas sematan

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Sematkan

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Tunda

# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = Hapus semua

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Disalin

# Tip of the button on a notification circle that copies its message.
ring-act-copy = Salin pesan

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Klik untuk menyerahkannya ke asisten Anda.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = Desktop Trindle

# Name of the login screen in the crash notice.
ring-crash-greeter = Layar masuk Trindle

# Name of the lock screen in the crash notice.
ring-crash-lock = Layar kunci Trindle

# Name of Settings in the crash notice.
ring-crash-settings = Pengaturan Trindle

# Name of Apps in the crash notice.
ring-crash-apps = Aplikasi Trindle

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = { $name } Trindle

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } berhenti secara tak terduga

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = Drive itu sudah tidak ada

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = { $name } tidak dapat dibuka

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Tidur

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Hibernasi

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Kunci layar

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Tidak melakukan apa-apa

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Matikan

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = Menutup tutup laptop melakukan apa yang diatur di Pengaturan, Daya

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Tahan di mana saja untuk memperbesar

# Screen reader name of the magnifier's glass.
ring-magnify-name = Kaca pembesar

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Berhenti memperbesar

# Tip of the switch in the ring that turns the microphone off for every program.
# SAFETY: needs a human review before it ships
ring-mic-off-tip = Matikan mikrofon untuk semua program

# Tip of the switch in the ring that turns the microphone back on.
# SAFETY: needs a human review before it ships
ring-mic-on-tip = Nyalakan mikrofon kembali

# Line in the ring when something other than the person pressed the microphone switch on.
# SAFETY: needs a human review before it ships
ring-mic-own-press = Hanya tekanan Anda sendiri yang dapat menyalakan mikrofon kembali.

# Line in the ring when the microphone switch could not be changed; the reason follows.
# SAFETY: needs a human review before it ships
ring-mic-not-changed = Sakelar mikrofon tidak dapat diubah: { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = Program tidak ditemukan

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Dikirim oleh program lain

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Tidak dapat memulainya

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Disalin untuk asisten Anda

# Screen reader name of a notification circle.
ring-notes-name = Notifikasi

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Sedang dipindah. Lepaskan untuk menjatuhkan, Escape untuk mengembalikan.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Dipindahkan

# Said when Escape put a held notification back.
ring-notes-put-back = Dikembalikan

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Sematkan agar dapat dipindah

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
       *[other] { $n } detik
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Sampai Anda menutupnya

# Time under a notification from yesterday.
ring-notify-yesterday = Kemarin { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = Kanan bawah

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = Kiri bawah

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = Kiri atas

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = Kanan atas

# Title of the notice when a plugin that asks for permissions has been given none.
# SAFETY: needs a human review before it ships
ring-plugin-needs = { $name } memerlukan izin Anda

# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
# SAFETY: needs a human review before it ships
ring-plugin-cannot-work = Belum dapat berfungsi: 0 dari { $asks } diizinkan.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Disalin. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = Menjangkau jaringan

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = Membuka file Anda

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
       *[other] { $names } dan { $more } lainnya
    }

