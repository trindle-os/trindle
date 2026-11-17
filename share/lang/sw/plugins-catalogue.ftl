# machine translated (Swahili), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Masasisho ya programu jalizi yamesimamishwa kwa sababu orodha ya programu jalizi imeharibika.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Anza upya

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle haikuweza kuondoa rekodi: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Jina lina nul byte.

# Error text when there is no record name to take away.
plugins-start-no-note = Hakuna rekodi ya kuondoa.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = Trindle haiwezi kukagua orodha ya programu jalizi kwa sababu ufunguo wake wa uthibitishaji haupo.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = Orodha ya programu jalizi ya Trindle haina saini, kwa hiyo Trindle haikuitumia.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = Saini kwenye orodha ya programu jalizi ya Trindle ina ukubwa usio sahihi, kwa hiyo Trindle haikutumia orodha hiyo.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = Orodha ya programu jalizi ya Trindle haijatiwa saini kwa ufunguo wa Trindle, kwa hiyo Trindle haikuitumia.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
# SAFETY: needs a human review before it ships
plugins-refused-no-number = Orodha ya programu jalizi ya Trindle haina namba ya toleo, kwa hiyo Trindle haikuitumia.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
# SAFETY: needs a human review before it ships
plugins-refused-older = Trindle haikutumia orodha hii ya programu jalizi kwa sababu namba yake ya toleo, { $is }, ni ya zamani kuliko ile iliyokubaliwa awali, { $after }.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
# SAFETY: needs a human review before it ships
plugins-refused-unreadable = Orodha ya programu jalizi ya Trindle imetiwa saini lakini haiwezi kusomwa, kwa hiyo Trindle haikuitumia: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Ya mwisho iliyo nzuri inaonyeshwa.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Si maandishi.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Orodha ni kubwa mno.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Si orodha ya programu jalizi, kwa sababu { $file } haina sehemu ya [Catalogue] yenye Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Orodha hii inatumia umbizo jipya zaidi kuliko toleo hili la Trindle linavyoweza kusoma. Trindle inaunga mkono Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } imeachwa: { $why ->
        [id] id yake haijaundwa kwa herufi, tarakimu, nukta na vistari
        [needs] inahitaji Name na Version
        [archive] Archive yake si jina la faili ya .tar.zst iliyo kando ya orodha
        [hash] SHA256 yake si tarakimu 64 za hex
        [permission] inaomba ruhusa ambayo haipo
       *[icon] Icon yake si jina la faili ya SVG au PNG iliyo kando ya orodha
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar haikuanza: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle haikuweza kufungua kumbukumbu: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = sababu haijulikani

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Kumbukumbu haina { $file } katika kiwango cha juu, wala kwenye folda moja iliyo katika kiwango cha juu.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle haikuweza kutengeneza folda ya muda: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl haikuanza: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Haikuweza kuipakua: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = Faili iliyopakuliwa hailingani na checksum ya orodha (SHA-256), kwa hiyo Trindle haikuweka chochote.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = Trindle haikuweka chochote kwa sababu programu jalizi ilishindwa ukaguzi wake: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Kumbukumbu ina { $found }, si { $wanted }, kwa hiyo Trindle haikuweka chochote.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle imeweka programu jalizi lakini haikuweza kurekodi kwamba inatoka kwenye orodha ya Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Haijawekwa, kwa hiyo hakuna cha kusasisha.
