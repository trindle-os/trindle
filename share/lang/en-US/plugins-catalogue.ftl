# American English: only the lines that differ from English. The rest is English.

plugins-refused-no-key = Trindle can't check the plugin catalog because its verification key is missing.

plugins-refused-no-signature = Trindle's plugin catalog has no signature, so Trindle didn't use it.

plugins-refused-wrong-size = The signature on Trindle's plugin catalog is the wrong size, so Trindle didn't use the catalog.

plugins-refused-not-signed = Trindle's plugin catalog isn't signed with Trindle's key, so Trindle didn't use it.

plugins-refused-no-number = Trindle's plugin catalog has no version number, so Trindle didn't use it.

plugins-refused-older = Trindle didn't use this plugin catalog because its version number, { $is }, is older than the one accepted before, { $after }.

plugins-refused-unreadable = Trindle's plugin catalog is signed but can't be read, so Trindle didn't use it: { $why }

plugins-catalogue-too-big = The catalog is too big.

plugins-catalogue-not-one = It isn't a plugin catalog, because { $file } has no [Catalog] part with a Format.

plugins-catalogue-newer = This catalog uses a newer format than this version of Trindle can read. Trindle supports Format={ $format }.

plugins-catalogue-left-out =
    { $id } is left out: { $why ->
        [id] its id isn't made of letters, digits, dots and dashes
        [needs] it needs a Name and a Version
        [archive] its Archive isn't the name of a .tar.zst file next to the catalog
        [hash] its SHA256 isn't 64 hex digits
        [permission] it asks for a permission that doesn't exist
       *[icon] its Icon isn't the name of an SVG or PNG file next to the catalog
    }.

plugins-install-hash = The downloaded file doesn't match the catalog's checksum (SHA-256), so Trindle didn't install anything.

plugins-install-note = Trindle installed the plugin but couldn't record that it came from Trindle's catalog: { $why }.
