
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Layar

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = layar monitor resolusi ukuran skala zoom lebih besar lebih kecil putar segarkan cahaya malam hangat cahaya biru malam mata

# Introduction under the heading of the display page, and the words behind the page i.
# SAFETY: needs a human review before it ships
settings-display-help = Perubahan berlaku seketika. Jika layar menjadi rusak, jangan lakukan apa-apa. Layar kembali sendiri setelah 15 detik kecuali Anda menekan Pertahankan. Cahaya malam membuat layar lebih hangat di malam hari.

# Question along the bottom of the page while a change of displays waits to be kept.
# SAFETY: needs a human review before it ships
settings-display-foot-keep = Pertahankan susunan ini?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Memeriksa layar…

# Sentence at the top of the page when no display was found.
settings-display-none-found = Layar tidak ditemukan.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = Tidak ada layar yang dipakai

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Satu layar dipakai

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
       *[other] { $n } layar dipakai
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = cahaya malam mati

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = cahaya malam menghangatkan layar dari { $from } sampai { $to }

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = cahaya malam menghangatkan layar dari matahari terbenam sampai terbit

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Normal

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Diputar ke kanan

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Diputar ke kiri

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = Terbalik

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = Di kanan

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = Di kiri

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Di atas

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Di bawah

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height } pada { $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (disarankan)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (standar)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Resolusi { $display }

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Ukuran pada { $display }

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Rotasi { $display }

# Heading of the resolution line.
settings-display-resolution = Resolusi

# Heading of the size line.
settings-display-size = Ukuran

# Heading of the rotation line.
settings-display-rotation = Rotasi

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Pakai layar ini

# Small words under that line for the only display in use.
settings-display-use-only = Satu-satunya layar yang dipakai tidak bisa dimatikan.

# Words behind the i of a display's card.
# SAFETY: needs a human review before it ships
settings-display-card-about = Perubahan berlaku seketika. Jika layar menjadi rusak, jangan lakukan apa-apa. Layar kembali sendiri setelah 15 detik kecuali Anda menekan Pertahankan. Resolusi yang disarankan adalah resolusi yang menjadi tujuan pembuatan layar itu. Ukuran standar adalah yang dipilih Trindle untuk layar ini.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Letakkan { $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Letakkan

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } relatif terhadap { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Susunan

# Words behind the i of that card.
settings-display-arrangement-about = Atur letak layar di samping satu sama lain, agar penunjuk bisa berpindah melewati tepi yang dipakai bersama.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Anda perlu dua layar untuk meletakkan yang satu di samping yang lain.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Tidak bisa mengubahnya.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
       *[other] Layar telah berubah. Layar kembali dalam { $seconds } detik kecuali Anda menekan Pertahankan.
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Tidak bisa membatalkan perubahan. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = Susunan sedang dipakai sekarang, tetapi Trindle tidak bisa menyimpannya untuk masuk berikutnya.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Tidak ada yang diubah

# Small words under that line.
settings-display-refused-change = Tidak bisa membuat perubahan itu.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Perubahan berlaku seketika

# Title of the night light card.
settings-display-night-title = Cahaya malam

# Words behind the i of that card.
settings-display-night-about = Layar menjadi lebih hangat di malam hari, yang lebih nyaman bagi mata. Ini mengikuti matahari terbenam dan terbit, dihitung dari zona waktu Anda, atau waktu yang Anda atur sendiri. Jika Trindle tidak bisa mengetahui lokasi Anda dari zona waktu, Trindle memakai waktu yang Anda atur.

# Heading of the night light line, and name of its switch.
settings-display-night = Cahaya malam

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Matahari terbenam dan terbit di { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle memakai waktu yang diatur karena tidak tahu kapan matahari terbenam.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Matahari terbenam sampai terbit

# Choice in the When list: between times the person sets.
settings-display-when-times = Waktu yang diatur

# Heading of the When line, and name of its list.
settings-display-when = Kapan

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = Dari

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = Sampai

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Kehangatan

# Small word under the left end of the warmth slider.
settings-display-little = Sedikit

# Small word under the right end of the warmth slider.
settings-display-very = Sangat hangat

# Spoken description of the strip of colours night light runs through.
settings-display-strip = Dari sedikit hangat sampai sangat hangat

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Tidak ada layar yang dipakai

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
       *[other] { $n } layar, sesuai susunannya
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Tidak ada layar

# Name of the large picture of the displays.
settings-display-your-displays = Layar Anda

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle tidak menemukan layar apa pun.

# Title of the card shown when no display was found.
settings-display-none-title = Tidak ada layar ditemukan

# Heading of the line in that card.
settings-display-none-heading = Layar tidak ditemukan

# Title of the card about the graphics card.
settings-display-graphics-title = Grafis

# Words behind the i of that card.
settings-display-graphics-about = Komputer ini punya kartu grafis NVIDIA yang memakai driver terbuka. Kartu ini berfungsi, tetapi kartu dari sekitar 2022 ke atas akan terasa lambat, karena driver terbuka tidak bisa menaikkan kecepatan kartu. Anda bisa memasang driver NVIDIA sendiri sebagai gantinya.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Kartu NVIDIA dengan driver terbuka

# Small words under that line.
settings-display-nvidia-about = Kartu dari 2022 ke atas mungkin berjalan lambat.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Apa yang bisa dilakukan
