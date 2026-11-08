
# machine translated (Turkish), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Eklenti güncellemeleri durduruldu: eklenti listesi bozuk.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = Yeniden başla

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Kayıt kaldırılamadı: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Bir adın içinde nul vardı.

# Error text when there is no record name to take away.
plugins-start-no-note = Kaldırılacak bir not yok.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = Bu Trindle'ın eklenti kataloğunu denetleyecek anahtarı yok, bu yüzden hiçbirini kabul etmez.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = Trindle'ın eklenti kataloğunda imza yok, bu yüzden kullanılmadı.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = Trindle'ın eklenti kataloğunun imzası bir imzanın boyutunda değil, bu yüzden katalog kullanılmadı.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = Trindle'ın eklenti kataloğu Trindle'ın anahtarıyla imzalanmamış, bu yüzden kullanılmadı.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle'ın eklenti kataloğu kaçıncı olduğunu belirtmiyor, bu yüzden kullanılmadı.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
# SAFETY: needs a human review before it ships
plugins-refused-older = Trindle'ın eklenti kataloğu daha önce kabul edilenden eski (numara { $is }, öncekinin numarası { $after }), bu yüzden kullanılmadı.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle'ın eklenti kataloğu imzalı ama okunamıyor, bu yüzden kullanılmadı: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Son çalışan katalog gösteriliyor.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Metin değil.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Katalog, bir katalog olamayacak kadar büyük.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Bu bir eklenti kataloğu değil: { $file } içinde Format'lı bir [Catalogue] bölümü yok.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Katalog, bu Trindle'ın okuduğundan daha yeni bir türde (Format={ $format }).

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } dışarıda bırakıldı: { $why ->
        [id] kimliği harf, rakam, nokta ve tireden oluşmuyor
        [needs] bir Name ve bir Version gerekiyor
        [archive] Archive, katalogun yanındaki bir .tar.zst dosyasının adı değil
        [hash] SHA256 değeri 64 onaltılık basamak değil
        [permission] olmayan bir izin istiyor
       *[icon] Icon, katalogun yanındaki bir SVG veya PNG dosyasının adı değil
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar başlatılamadı: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Arşiv açılamadı: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar kabul etmedi

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Arşivin en üstünde veya en üstündeki tek klasörde { $file } yok.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Geçici klasör oluşturulamadı: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl başlatılamadı: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = İndirilemedi: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = Dosyası katalogda listelenen dosya değil (SHA-256 farklı), bu yüzden hiçbir şey kurulmadı.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = Trindle'ın eklenti denetiminden geçmiyor, bu yüzden hiçbir şey kurulmadı: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Arşivde { $wanted } değil { $found } var, bu yüzden hiçbir şey kurulmadı.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Yerine kondu, ancak Trindle'ın kataloğundan geldiğini belirten not yazılamadı: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Kurulu değil, bu yüzden güncellenecek bir şey yok.

