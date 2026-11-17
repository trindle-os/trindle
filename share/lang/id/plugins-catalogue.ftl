# machine translated (Indonesian), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Pembaruan plugin dijeda karena daftar plugin rusak.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Mulai ulang

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle tidak dapat menghapus catatan: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Sebuah nama berisi byte nol.

# Error text when there is no record name to take away.
plugins-start-no-note = Tidak ada catatan untuk dihapus.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = Trindle tidak dapat memeriksa katalog plugin karena kunci verifikasinya tidak ada.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = Katalog plugin Trindle tidak memiliki tanda tangan, jadi Trindle tidak menggunakannya.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = Tanda tangan pada katalog plugin Trindle ukurannya salah, jadi Trindle tidak menggunakan katalog itu.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = Katalog plugin Trindle tidak ditandatangani dengan kunci Trindle, jadi Trindle tidak menggunakannya.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Katalog plugin Trindle tidak memiliki nomor versi, jadi Trindle tidak menggunakannya.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle tidak menggunakan katalog plugin ini karena nomor versinya, { $is }, lebih lama daripada yang diterima sebelumnya, { $after }.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Katalog plugin Trindle sudah ditandatangani tetapi tidak dapat dibaca, jadi Trindle tidak menggunakannya: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Yang terakhir baik ditampilkan.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Bukan teks.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Katalog terlalu besar.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Ini bukan katalog plugin, karena { $file } tidak memiliki bagian [Catalogue] dengan Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Katalog ini memakai format yang lebih baru daripada yang dapat dibaca Trindle versi ini. Trindle mendukung Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out = 
    { $id } dilewati: { $why ->
        [id] ID-nya tidak terdiri dari huruf, angka, titik, dan tanda hubung
        [needs] membutuhkan Name dan Version
        [archive] Archive-nya bukan nama file .tar.zst di sebelah katalog
        [hash] SHA256-nya bukan 64 digit heksadesimal
        [permission] meminta izin yang tidak ada
       *[icon] Icon-nya bukan nama file SVG atau PNG di sebelah katalog
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar tidak berjalan: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle tidak dapat mengekstrak arsip: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = penyebab tidak diketahui

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Arsip tidak memiliki { $file } di tingkat teratas, atau di satu folder di tingkat teratas.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle tidak dapat membuat folder sementara: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl tidak berjalan: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Tidak dapat diunduh: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = File yang diunduh tidak cocok dengan checksum katalog (SHA-256), jadi Trindle tidak memasang apa pun.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = Trindle tidak memasang apa pun karena plugin gagal dalam pemeriksaannya: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Arsip berisi { $found }, bukan { $wanted }, jadi Trindle tidak memasang apa pun.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle memasang plugin itu tetapi tidak dapat mencatat bahwa plugin itu berasal dari katalog Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Plugin belum terpasang, jadi tidak ada yang diperbarui.
