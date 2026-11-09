
# machine translated (German), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Plugin-Updates sind angehalten: Die Plugin-Liste ist beschädigt.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Neu beginnen

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Der Eintrag konnte nicht entfernt werden: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Ein Name enthielt ein Nullzeichen.

# Error text when there is no record name to take away.
plugins-start-no-note = Es gibt keinen Vermerk zum Entfernen.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Dieses Trindle hat keinen Schlüssel, um seinen Plugin-Katalog zu prüfen, und nimmt deshalb keinen an.

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Der Plugin-Katalog von Trindle hat keine Signatur und wurde nicht verwendet.

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Die Signatur des Plugin-Katalogs von Trindle hat nicht die Größe einer Signatur, deshalb wurde der Katalog nicht verwendet.

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Der Plugin-Katalog von Trindle ist nicht mit dem Schlüssel von Trindle signiert und wurde nicht verwendet.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Der Plugin-Katalog von Trindle gibt seine Nummer nicht an und wurde nicht verwendet.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Der Plugin-Katalog von Trindle ist älter als der zuvor angenommene (Nummer { $is }, nach { $after }) und wurde nicht verwendet.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Der Plugin-Katalog von Trindle ist signiert, lässt sich aber nicht lesen, und wurde nicht verwendet: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Der letzte gültige wird angezeigt.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Es ist kein Text.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Der Katalog ist zu groß, um ein Katalog zu sein.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Es ist kein Plugin-Katalog: { $file } hat keinen Abschnitt [Catalogue] mit seinem Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Der Katalog ist neuer, als dieses Trindle lesen kann (Format={ $format }).

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
plugins-unpack-failed = Das Archiv konnte nicht entpackt werden: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar hat es abgelehnt

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Das Archiv enthält keine { $file } auf der obersten Ebene oder in dem einen Ordner dort.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Es konnte kein temporärer Ordner angelegt werden: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl konnte nicht gestartet werden: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Es konnte nicht heruntergeladen werden: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = Die Datei ist nicht die, die der Katalog aufführt (der SHA-256 weicht ab), daher wurde nichts installiert.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = Es besteht die Plugin-Prüfung von Trindle nicht, daher wurde nichts installiert: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Das Archiv enthält { $found }, nicht { $wanted }, daher wurde nichts installiert.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Es wurde eingefügt, aber der Vermerk, dass es aus dem Katalog von Trindle stammt, konnte nicht geschrieben werden: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Es ist nicht installiert, daher gibt es nichts zu aktualisieren.
