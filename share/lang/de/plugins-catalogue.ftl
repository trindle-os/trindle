
# machine translated (German), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Plugin-Updates sind angehalten, weil die Plugin-Liste beschädigt ist.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Neu beginnen

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle konnte den Eintrag nicht entfernen: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Ein Name enthält ein Nullbyte.

# Error text when there is no record name to take away.
plugins-start-no-note = Es gibt keinen Eintrag zum Entfernen.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Trindle kann den Plugin-Katalog nicht prüfen, weil der Prüfschlüssel fehlt.

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Der Plugin-Katalog von Trindle hat keine Signatur, daher hat Trindle ihn nicht verwendet.

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Die Signatur des Plugin-Katalogs von Trindle hat die falsche Größe, daher hat Trindle den Katalog nicht verwendet.

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Der Plugin-Katalog von Trindle ist nicht mit dem Schlüssel von Trindle signiert, daher hat Trindle ihn nicht verwendet.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Der Plugin-Katalog von Trindle hat keine Versionsnummer, daher hat Trindle ihn nicht verwendet.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
plugins-refused-older = Trindle hat diesen Plugin-Katalog nicht verwendet, weil er älter ist als ein zuvor verwendeter.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Der Plugin-Katalog von Trindle ist signiert, lässt sich aber nicht lesen, daher hat Trindle ihn nicht verwendet: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Der letzte gültige wird angezeigt.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Es ist kein Text.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Der Katalog ist zu groß.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Es ist kein Plugin-Katalog, weil { $file } keinen Abschnitt [Catalogue] mit einem Format hat.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Dieser Katalog verwendet ein neueres Format, als diese Version von Trindle lesen kann. Trindle unterstützt Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } wird ausgelassen: { $why ->
        [id] seine ID besteht nicht aus Buchstaben, Ziffern, Punkten und Bindestrichen
        [needs] es braucht Name und Version
        [archive] sein Archive ist nicht der Name einer .tar.zst-Datei neben dem Katalog
        [hash] sein SHA256 hat nicht 64 Hexadezimalziffern
        [permission] es verlangt eine Berechtigung, die es nicht gibt
       *[icon] sein Icon ist nicht der Name einer SVG- oder PNG-Datei neben dem Katalog
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar konnte nicht gestartet werden: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle konnte das Archiv nicht entpacken: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = Grund unbekannt

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Das Archiv enthält keine { $file } auf der obersten Ebene oder in dem einen Ordner dort.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle konnte keinen temporären Ordner anlegen: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl konnte nicht gestartet werden: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Es konnte nicht heruntergeladen werden: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = Die heruntergeladene Datei passt nicht zur Prüfsumme des Katalogs (SHA-256), daher hat Trindle nichts installiert.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = Trindle hat nichts installiert, weil das Plugin seine Prüfungen nicht bestanden hat: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Das Archiv enthält { $found }, nicht { $wanted }, daher hat Trindle nichts installiert.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle hat das Plugin installiert, konnte aber nicht festhalten, dass es aus dem Katalog von Trindle stammt: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Es ist nicht installiert, daher gibt es nichts zu aktualisieren.
