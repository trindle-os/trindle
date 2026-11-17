
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Mulai saat masuk

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay mulai masuk otomatis jeda tunda

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Aplikasi yang terbuka sendiri saat Anda masuk. Anda bisa menambahkan jeda agar aplikasi menunggu jaringan atau aplikasi lain lebih dulu.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = Tidak ada aplikasi yang mulai sendiri saat Anda masuk.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } mulai sendiri saat Anda masuk.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
       *[other] { $n } aplikasi mulai sendiri saat Anda masuk.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
       *[other] { $n } lagi dimatikan.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = Langsung

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Setelah semenit

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = Setelah 2 menit

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
       *[other] Setelah { $n } detik
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Jeda untuk { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Mulai saat masuk

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = Jangan mulai saat masuk

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Trindle tidak bisa menghapusnya, jadi aplikasi ini masih mulai saat masuk.

# Under the name of a launcher that comes with the system, which can only be switched on or off; { $command } is what it runs, cut short.
settings-startup-system = Bawaan sistem: { $command }

# Red words under the add card when something could not be added.
settings-startup-not-added = Tidak bisa menambahkannya.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = Cari aplikasi

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Mulai ini saat masuk

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = Trindle tidak bisa menambahkannya, karena aplikasi tidak lagi terpasang.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Pilih program

# Heading of the line for adding a command.
settings-startup-command = Perintah

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Ketik perintah, atau pilih program.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Pilih program untuk dimulai saat masuk

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Tidak ada yang mulai sendiri

# Title of the card listing what starts at login.
settings-startup-list-title = Mulai saat masuk

# Words behind the i of the card listing what starts at login.
settings-startup-about = Setiap aplikasi dapat menunggu sebentar setelah Anda masuk, agar jaringan atau aplikasi lain siap lebih dulu.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } menjalankan: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Pilih dari aplikasi terpasang

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Ketik perintah, atau pilih program

# Heading of the line for adding an installed app.
settings-startup-app-heading = Aplikasi terpasang

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Perintah atau program

# Words behind the i of the Add an app card.
settings-startup-add-about = Pilih salah satu aplikasi Anda yang terpasang, atau ketik perintah untuk dijalankan, seperti di Terminal. Anda juga bisa memilih program di disk. Aplikasi mulai saat Anda masuk berikutnya. Setelah ada di daftar, Anda bisa memilih jeda untuknya.

# Title of the card for adding an app.
settings-startup-add-title = Tambah aplikasi
