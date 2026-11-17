
# machine translated (Urdu), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = پلگ ان اپڈیٹس رکی ہوئی ہیں کیونکہ پلگ ان کی فہرست خراب ہے۔

# SAFETY: needs a human review before it ships
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = دوبارہ شروع کریں

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle ریکارڈ ہٹا نہیں سکا: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = ایک نام میں null بائٹ ہے۔

# Error text when there is no record name to take away.
plugins-start-no-note = ہٹانے کے لیے کوئی ریکارڈ نہیں ہے۔

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = تصدیقی کلید موجود نہ ہونے کی وجہ سے Trindle پلگ ان فہرست کی جانچ نہیں کر سکتا۔

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle کی پلگ ان فہرست پر دستخط نہیں ہے، اس لیے Trindle نے اسے استعمال نہیں کیا۔

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Trindle کی پلگ ان فہرست کے دستخط کا سائز غلط ہے، اس لیے Trindle نے فہرست استعمال نہیں کی۔

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle کی پلگ ان فہرست پر Trindle کی کلید سے دستخط نہیں ہیں، اس لیے Trindle نے اسے استعمال نہیں کیا۔

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle کی پلگ ان فہرست میں ورژن نمبر نہیں ہے، اس لیے Trindle نے اسے استعمال نہیں کیا۔

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle نے یہ پلگ ان فہرست استعمال نہیں کی کیونکہ اس کا ورژن نمبر { $is } پہلے قبول کی گئی فہرست کے نمبر { $after } سے پرانا ہے۔

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle کی پلگ ان فہرست پر دستخط ہیں لیکن وہ پڑھی نہیں جا سکتی، اس لیے Trindle نے اسے استعمال نہیں کیا: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } آخری درست فہرست دکھائی جا رہی ہے۔

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = یہ متن نہیں ہے۔

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = فہرست بہت بڑی ہے۔

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = یہ پلگ ان فہرست نہیں ہے: { $file } میں Format کے ساتھ [Catalogue] حصہ نہیں ہے۔

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = یہ فہرست اس ورژن کے Trindle کے پڑھنے سے نئے فارمیٹ میں ہے۔ Trindle Format={ $format } کو سپورٹ کرتا ہے۔

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
plugins-unpack-tar = tar شروع نہیں ہوا: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle آرکائیو کھول نہیں سکا: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = وجہ معلوم نہیں

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = آرکائیو کی اوپری سطح پر، یا اوپری سطح کے ایک فولڈر میں، { $file } نہیں ہے۔

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle عارضی فولڈر نہیں بنا سکا: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl شروع نہیں ہوا: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = ڈاؤن لوڈ نہیں ہو سکا: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = ڈاؤن لوڈ کی گئی فائل فہرست کے چیک سم (SHA-256) سے میل نہیں کھاتی، اس لیے Trindle نے کچھ انسٹال نہیں کیا۔

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = پلگ ان اپنی جانچ میں ناکام رہا، اس لیے Trindle نے کچھ انسٹال نہیں کیا: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = آرکائیو میں { $wanted } نہیں بلکہ { $found } ہے، اس لیے Trindle نے کچھ انسٹال نہیں کیا۔

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle نے پلگ ان انسٹال کر لیا لیکن یہ ریکارڈ نہیں کر سکا کہ یہ Trindle کی فہرست سے آیا: { $why }۔

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = یہ انسٹال نہیں ہے، اس لیے اپڈیٹ کرنے کو کچھ نہیں۔
