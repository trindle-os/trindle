
# machine translated (Urdu), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = پلگ ان اپڈیٹس رکی ہوئی ہیں: پلگ ان کی فہرست خراب ہے۔

# SAFETY: needs a human review before it ships
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = دوبارہ شروع کریں

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = ریکارڈ ہٹایا نہیں جا سکا: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = ایک نام میں nul تھا۔

# Error text when there is no record name to take away.
plugins-start-no-note = ہٹانے کے لیے کوئی نوٹ نہیں ہے۔

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = اس Trindle کے پاس پلگ ان فہرست جانچنے کی کلید نہیں ہے، اس لیے یہ کوئی فہرست قبول نہیں کرتا۔

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle کی پلگ ان فہرست پر دستخط نہیں ہے، اس لیے اسے استعمال نہیں کیا گیا۔

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Trindle کی پلگ ان فہرست کے دستخط کا سائز دستخط جیسا نہیں ہے، اس لیے فہرست استعمال نہیں کی گئی۔

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle کی پلگ ان فہرست پر Trindle کی کلید سے دستخط نہیں ہیں، اس لیے اسے استعمال نہیں کیا گیا۔

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle کی پلگ ان فہرست یہ نہیں بتاتی کہ اس کا نمبر کیا ہے، اس لیے اسے استعمال نہیں کیا گیا۔

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle کی پلگ ان فہرست پہلے قبول کی گئی فہرست سے پرانی ہے (نمبر { $is }، پہلے { $after })، اس لیے اسے استعمال نہیں کیا گیا۔

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle کی پلگ ان فہرست پر دستخط ہیں لیکن پڑھی نہیں جا سکتی، اس لیے استعمال نہیں کی گئی: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } آخری درست فہرست دکھائی جا رہی ہے۔

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = یہ متن نہیں ہے۔

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = فہرست اتنی بڑی ہے کہ فہرست نہیں ہو سکتی۔

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = یہ پلگ ان فہرست نہیں ہے: { $file } میں Format کے ساتھ [Catalogue] حصہ نہیں ہے۔

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = فہرست اس Trindle کے پڑھنے سے نئی قسم کی ہے (Format={ $format })۔

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } چھوڑ دیا گیا: { $why ->
        [id] اس کی id حروف، ہندسوں، نقطوں اور ڈیشوں پر مشتمل نہیں
        [needs] اسے Name اور Version چاہییں
        [archive] اس کا Archive فہرست کے ساتھ والی .tar.zst فائل کا نام نہیں
        [hash] اس کا SHA256 64 ہیکس ہندسے نہیں
        [permission] یہ ایسی اجازت مانگتا ہے جو موجود نہیں
       *[icon] اس کا Icon فہرست کے ساتھ والی SVG یا PNG فائل کا نام نہیں
    }۔

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar شروع نہیں ہو سکا: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = آرکائیو کھولا نہیں جا سکا: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar نے انکار کیا

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = آرکائیو کے اوپر، یا اوپر کے ایک فولڈر میں، { $file } نہیں ہے۔

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = عارضی فولڈر نہیں بن سکا: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl شروع نہیں ہو سکا: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = ڈاؤن لوڈ نہیں ہو سکا: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = اس کی فائل وہ نہیں جو فہرست میں ہے (SHA-256 مختلف ہے)، اس لیے کچھ انسٹال نہیں ہوا۔

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = یہ Trindle کی پلگ ان جانچ پاس نہیں کرتا، اس لیے کچھ انسٹال نہیں ہوا: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = آرکائیو میں { $wanted } نہیں بلکہ { $found } ہے، اس لیے کچھ انسٹال نہیں ہوا۔

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = یہ انسٹال ہو گیا، لیکن یہ نوٹ نہیں لکھا جا سکا کہ یہ Trindle کی فہرست سے آیا: { $why }۔

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = یہ انسٹال نہیں ہے، اس لیے اپڈیٹ کرنے کو کچھ نہیں۔
