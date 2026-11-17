
# machine translated (Indonesian), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Itu bukan nama program yang valid.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Tidak dapat disimpan.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Pilih folder yang ada.

# Red words when the whole home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-whole-home = Seluruh folder beranda tetap pribadi.

# Red words when a folder that holds the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-holds-home = Folder ini memuat folder beranda Anda, jadi tetap pribadi.

# Red words when a folder outside the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-outside-home = Hanya folder di dalam folder beranda Anda yang dapat dibagikan.

# Red words when a hidden folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-hidden = Folder tersembunyi selalu tetap pribadi.

# Red words when a folder that keeps passwords or mail is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-passwords = Folder ini menyimpan kata sandi atau surel, jadi tetap pribadi.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Program itu tidak memiliki folder pengaturan yang dapat dipakai.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Ketik nama situs, seperti api.example.com, atau *.example.com untuk setiap situs di bawah nama itu.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Ketik nama komputer, seperti nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Tulis masing-masing sebagai NAME=nilai.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Tulis masing-masing sebagai NAME=nilai, dengan huruf kapital.

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = Sebuah program AI sedang meminta memakai folder ini. Saat ini program itu tidak dapat melihatnya.

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = Sebuah program AI sedang meminta menjalankan ini sebagai administrator. Program itu akan berjalan di luar pagar dan dapat melihat semua yang dapat Anda lihat.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = Program AI

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
# SAFETY: needs a human review before it ships
store-fence-read-only = Hanya izinkan membaca folder ini

# Note under the question about a folder.
store-fence-sees-next = Program itu akan melihat folder ini saat program dimulai berikutnya.

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = Tidak

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = Bagikan untuk sesi ini saja

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = Selalu bagikan

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = Jangan izinkan

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = Izinkan

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Perintah itu terlalu panjang untuk ditampilkan utuh, jadi tidak dijalankan.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Permintaan lain sedang menunggu Anda menjawabnya.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = Anda tidak mengizinkan ini.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } tidak tersedia di dalam pagar.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = Tidak dapat dijalankan di luar pagar.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Berikan path lengkap folder.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = Anda tidak membagikannya.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Dibagikan. Folder itu akan ada di sana saat program ini dijalankan lagi.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Sebutkan komputer yang akan dihubungi.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = Program AI di dalam pagar tidak dapat menjangkau komputer itu dengan ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = Kunci untuk program AI tidak dapat dibuat: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Sebutkan perintah yang akan dijalankan.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } tidak tersedia di sini.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } tidak tersedia di dalam pagar. Minta agar dijalankan di luar.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: jalur permintaan pagar tidak ada.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: permintaan tidak terkirim.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock menolak: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = Pagar tidak dapat dimulai: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = Pagar tidak dapat dimulai: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = Program AI menyimpan login sendiri di sini, terpisah dari login Anda. Masuklah di dalam program saat diminta.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Folder Anda bersifat pribadi bagi program AI. Bagikan satu folder di Pengaturan, Program AI, agar program dapat bekerja di dalamnya.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = Pagar tidak dapat dimulai: filter panggilan sistem tidak dapat dibuat.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = Pagar tidak dapat dimulai: batas jaringan tidak dapat disiapkan.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = Pagar tidak dapat dimulai: bubblewrap tidak mau berjalan: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: tidak ditemukan di luar pagar
