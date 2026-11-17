# machine translated (Indonesian), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
mcp-title-ask = Permintaan asisten

# Heading over the words of what the assistant wants to do.
mcp-ask-heading = Asisten yang terhubung ke Trindle ingin:

# Button that refuses the change the assistant asked for.
# SAFETY: needs a human review before it ships
mcp-no = Tidak

# Button that lets the change the assistant asked for go ahead.
# SAFETY: needs a human review before it ships
mcp-yes = Ya, lanjutkan

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = menampilkan { $name } di depan

# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = mengubah pengaturan { $key } dari { $from } menjadi { $to }

# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = memakai tema { $look }

# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = memakai tema { $look } dengan { $accent } sebagai warna aksen

# The question for installing an app from Flathub; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-install = memasang { $id } dari Flathub

# The question for removing an app; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-remove = menghapus { $id } dan tetap menyimpan berkasnya

# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-network = bergabung dengan jaringan Wi-Fi { $name }

# The question for pairing a Bluetooth device; its address comes in.
# SAFETY: needs a human review before it ships
mcp-ask-pair = berpasangan dengan perangkat Bluetooth { $address }

# The question for putting settings back; the name of the saved point comes in.
# SAFETY: needs a human review before it ships
mcp-ask-undo = mengembalikan pengaturan dan izin aplikasi seperti sebelumnya: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = kembalikan pengaturan, izin aplikasi dan file { $id } seperti sebelumnya: { $note }. { $id } ditutup dulu jika terbuka

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = tutup { $id } lalu mulai lagi. Yang belum disimpan di dalamnya hilang

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = ubah { $key } di { $file } milik { $id } dari { $from } ke { $to }. { $id } ditutup dulu jika terbuka

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = tambahkan { $key } sebagai { $to } di { $file } milik { $id }. { $id } ditutup dulu jika terbuka

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = hapus cache { $id }. { $id } ditutup dulu jika terbuka, dan salinan cache disimpan agar bisa dikembalikan

# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
# SAFETY: needs a human review before it ships
mcp-ask-pair-named = berpasangan dengan perangkat Bluetooth { $name } ({ $address })

# The question for closing a window; the window's title or app name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-close = menutup { $name }

# The question for opening an app; the app's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-open = membuka { $name }

# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
# SAFETY: needs a human review before it ships
mcp-ask-allow = mengizinkan { $id } memakai { $what }

# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
# SAFETY: needs a human review before it ships
mcp-ask-deny = melarang { $id } memakai { $what }
