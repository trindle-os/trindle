# Trindle's plugin catalogue: why it was not used, why a plugin could not be fetched or put in, and the pause with its Start again button (src/plugins/catalogue.rs).

## The pause (src/plugins/catalogue.rs)

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Plugin updates are paused: the plugin list is damaged.
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = Start again
# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = The record could not be taken away: { $why }
# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = A name had a nul in it.
# Error text when there is no record name to take away.
plugins-start-no-note = There is no note to take away.

## Why the catalogue was not used (src/plugins/catalogue.rs)

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = This Trindle has no key to check its plugin catalogue with, so it accepts none.
# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle's plugin catalogue has no signature, so it was not used.
# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = The signature of Trindle's plugin catalogue is not the size a signature is, so the catalogue was not used.
# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle's plugin catalogue is not signed with Trindle's key, so it was not used.
# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle's plugin catalogue does not say which number it is, so it was not used.
# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle's plugin catalogue is older than the one accepted before (number { $is }, after { $after }), so it was not used.
# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle's plugin catalogue is signed but cannot be read, so it was not used: { $why }
# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } The last good one is shown.
# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = It is not text.
# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = The catalogue is too big to be one.
# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = It is not a plugin catalogue: { $file } has no [Catalogue] part with its Format.
# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = The catalogue is of a newer kind than this Trindle reads (Format={ $format }).
# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } is left out: { $why ->
        [id] its id is not letters, digits, dots and dashes
        [needs] it needs a Name and a Version
        [archive] its Archive is not the name of a .tar.zst file beside the catalogue
        [hash] its SHA256 is not 64 hex digits
        [permission] it asks for a permission there is not
       *[icon] its Icon is not the name of an SVG or PNG file beside the catalogue
    }.

## Fetching and putting in a plugin (src/plugins/catalogue.rs)

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar could not be started: { $why }
# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = The archive could not be unpacked: { $why }
# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar refused it
# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = The archive has no { $file } at its top, or in the one folder at its top.
# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = No scratch folder could be made: { $why }
# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl could not be started: { $why }
# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = It could not be downloaded: { $why }
# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = Its file is not the one the catalogue lists (the SHA-256 differs), so nothing was installed.
# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = It does not pass Trindle's plugin check, so nothing was installed: { $why }
# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = The archive holds { $found }, not { $wanted }, so nothing was installed.
# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = It was put in, but the note that it came from Trindle's catalogue could not be written: { $why }.
# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = It is not installed, so there is nothing to update.
