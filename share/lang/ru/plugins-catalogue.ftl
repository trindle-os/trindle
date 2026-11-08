# machine translated (Russian), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Обновления плагинов приостановлены: список плагинов повреждён.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Начать заново

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Не удалось удалить запись: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = В имени оказался нулевой символ (nul).

# Error text when there is no record name to take away.
plugins-start-no-note = Нет записи, которую можно удалить.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = В этой Trindle нет ключа для проверки каталога плагинов, поэтому она не принимает никакой каталог.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = У каталога плагинов Trindle нет подписи, поэтому он не использован.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = Подпись каталога плагинов Trindle имеет неверный размер, поэтому каталог не использован.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = Каталог плагинов Trindle не подписан ключом Trindle, поэтому он не использован.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = В каталоге плагинов Trindle не указан его номер, поэтому он не использован.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Каталог плагинов Trindle старше того, что был принят раньше (номер { $is }, а принят был { $after }), поэтому он не использован.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Каталог плагинов Trindle подписан, но его нельзя прочитать, поэтому он не использован: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Показан последний исправный каталог.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Это не текст.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Каталог слишком велик.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Это не каталог плагинов: в { $file } нет раздела [Catalogue] с Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Каталог новее, чем может прочитать эта Trindle (Format={ $format }).

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } пропущен: { $why ->
        [id] его id состоит не только из букв, цифр, точек и дефисов
        [needs] ему нужны Name и Version
        [archive] его Archive - не имя файла .tar.zst рядом с каталогом
        [hash] его SHA256 - не 64 шестнадцатеричные цифры
        [permission] он просит несуществующее разрешение
       *[icon] его Icon - не имя файла SVG или PNG рядом с каталогом
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = Не удалось запустить tar: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Не удалось распаковать архив: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar отказался

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = В архиве нет файла { $file } ни на верхнем уровне, ни в единственной папке на верхнем уровне.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Не удалось создать временную папку: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = Не удалось запустить curl: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Не удалось скачать: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = Файл не тот, что указан в каталоге (SHA-256 отличается), поэтому ничего не установлено.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = Плагин не прошёл проверку Trindle, поэтому ничего не установлено: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
# SAFETY: needs a human review before it ships
plugins-install-other-id = В архиве лежит «{ $found }», а не «{ $wanted }», поэтому ничего не установлено.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Плагин установлен, но не удалось записать пометку, что он из каталога Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Он не установлен, поэтому обновлять нечего.
