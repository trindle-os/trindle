# Trindle's plugin catalogue: why it was not used, why a plugin could not be fetched or put in, and the pause with its Start again button (src/plugins/catalogue.rs).

## The pause (src/plugins/catalogue.rs)

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Plugin updates are paused because the plugin list is damaged.
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = Start again
# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle couldn't remove the record: { $why }
# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = A name has a null byte.
# Error text when there is no record name to take away.
plugins-start-no-note = There is no record to remove.

## Why the catalogue was not used (src/plugins/catalogue.rs)

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Trindle can't check the plugin catalogue because its verification key is missing.
# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle's plugin catalogue has no signature, so Trindle didn't use it.
# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = The signature on Trindle's plugin catalogue is the wrong size, so Trindle didn't use the catalogue.
# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle's plugin catalogue isn't signed with Trindle's key, so Trindle didn't use it.
# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle's plugin catalogue has no version number, so Trindle didn't use it.
# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle didn't use this plugin catalogue because its version number, { $is }, is older than the one accepted before, { $after }.
# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle's plugin catalogue is signed but can't be read, so Trindle didn't use it: { $why }
# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } The last good one is shown.
# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = It isn't text.
# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = The catalogue is too big.
# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = It isn't a plugin catalogue, because { $file } has no [Catalogue] part with a Format.
# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = This catalogue uses a newer format than this version of Trindle can read. Trindle supports Format={ $format }.
# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } is left out: { $why ->
        [id] its id isn't made of letters, digits, dots and dashes
        [needs] it needs a Name and a Version
        [archive] its Archive isn't the name of a .tar.zst file next to the catalogue
        [hash] its SHA256 isn't 64 hex digits
        [permission] it asks for a permission that doesn't exist
       *[icon] its Icon isn't the name of an SVG or PNG file next to the catalogue
    }.

## Fetching and putting in a plugin (src/plugins/catalogue.rs)

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar didn't start: { $why }
# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle couldn't unpack the archive: { $why }
# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = reason unknown
# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = The archive has no { $file } at the top level, or in the one folder at the top level.
# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle couldn't make a temporary folder: { $why }
# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl didn't start: { $why }
# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Couldn't download it: { $why }
# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = The downloaded file doesn't match the catalogue's checksum (SHA-256), so Trindle didn't install anything.
# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = Trindle didn't install anything because the plugin failed its checks: { $why }
# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = The archive contains { $found }, not { $wanted }, so Trindle didn't install anything.
# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle installed the plugin but couldn't record that it came from Trindle's catalogue: { $why }.
# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = It isn't installed, so there is nothing to update.
