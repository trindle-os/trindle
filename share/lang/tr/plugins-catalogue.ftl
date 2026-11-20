
# machine translated (Turkish), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Eklenti listesi bozuk olduğu için eklenti güncellemeleri durduruldu.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = Yeniden başla

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle kaydı kaldıramadı: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Bir adda null bayt var.

# Error text when there is no record name to take away.
plugins-start-no-note = Kaldırılacak bir kayıt yok.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = Doğrulama anahtarı eksik olduğu için Trindle eklenti kataloğunu denetleyemiyor.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = Trindle'ın eklenti kataloğunda imza yok, bu yüzden Trindle onu kullanmadı.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = Trindle'ın eklenti kataloğundaki imzanın boyutu yanlış, bu yüzden Trindle kataloğu kullanmadı.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = Trindle'ın eklenti kataloğu Trindle'ın anahtarıyla imzalanmamış, bu yüzden Trindle onu kullanmadı.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle'ın eklenti kataloğunda sürüm numarası yok, bu yüzden Trindle onu kullanmadı.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
# SAFETY: needs a human review before it ships
plugins-refused-older = Trindle bu eklenti kataloğunu kullanmadı, çünkü daha önce kullandığı katalogdan eski.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle'ın eklenti kataloğu imzalı ama okunamıyor, bu yüzden Trindle onu kullanmadı: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Son çalışan katalog gösteriliyor.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Metin değil.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Katalog çok büyük.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Bu bir eklenti kataloğu değil, çünkü { $file } içinde Format'lı bir [Catalogue] bölümü yok.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Bu katalog, Trindle'ın bu sürümünün okuyabildiğinden daha yeni bir biçim kullanıyor. Trindle'ın desteklediği: Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } dışarıda bırakıldı: { $why ->
        [id] kimliği harf, rakam, nokta ve tireden oluşmuyor
        [needs] bir Name ve bir Version gerekiyor
        [archive] Archive, katalogun yanındaki bir .tar.zst dosyasının adı değil
        [hash] SHA256 değeri 64 onaltılık basamak değil
        [permission] var olmayan bir izin istiyor
       *[icon] Icon, katalogun yanındaki bir SVG veya PNG dosyasının adı değil
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar başlamadı: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle arşivi açamadı: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = neden bilinmiyor

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Arşivin en üst düzeyinde veya en üst düzeydeki tek klasörde { $file } yok.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle geçici bir klasör oluşturamadı: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl başlamadı: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = İndirilemedi: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = İndirilen dosya katalogdaki sağlama toplamıyla (SHA-256) eşleşmiyor, bu yüzden Trindle hiçbir şey kurmadı.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = Eklenti denetimlerden geçemediği için Trindle hiçbir şey kurmadı: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Arşivde { $wanted } değil { $found } var, bu yüzden Trindle hiçbir şey kurmadı.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle eklentiyi kurdu ama Trindle'ın kataloğundan geldiğini kaydedemedi: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Kurulu değil, bu yüzden güncellenecek bir şey yok.
