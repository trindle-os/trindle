# machine translated (Hindi), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = प्लगइन अपडेट रुके हुए हैं, क्योंकि प्लगइन सूची खराब है।

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = फिर से शुरू करें

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle रिकॉर्ड नहीं हटा सका: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = एक नाम में null बाइट है।

# Error text when there is no record name to take away.
plugins-start-no-note = हटाने के लिए कोई रिकॉर्ड नहीं है।

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Trindle प्लगइन सूची की जाँच नहीं कर सकता, क्योंकि उसकी सत्यापन कुंजी गायब है।

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle की प्लगइन सूची पर कोई हस्ताक्षर नहीं है, इसलिए Trindle ने उसे इस्तेमाल नहीं किया।

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Trindle की प्लगइन सूची के हस्ताक्षर का आकार गलत है, इसलिए Trindle ने सूची इस्तेमाल नहीं की।

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle की प्लगइन सूची पर Trindle की कुंजी से हस्ताक्षर नहीं हैं, इसलिए Trindle ने उसे इस्तेमाल नहीं किया।

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle की प्लगइन सूची में कोई संस्करण संख्या नहीं है, इसलिए Trindle ने उसे इस्तेमाल नहीं किया।

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle ने इस प्लगइन सूची का इस्तेमाल नहीं किया, क्योंकि उसकी संस्करण संख्या { $is } पहले मानी गई संख्या { $after } से पुरानी है।

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle की प्लगइन सूची पर हस्ताक्षर हैं, पर उसे पढ़ा नहीं जा सकता, इसलिए Trindle ने उसे इस्तेमाल नहीं किया: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } आखिरी सही सूची दिखाई गई है।

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = यह टेक्स्ट नहीं है।

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = सूची बहुत बड़ी है।

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = यह प्लगइन सूची नहीं है, क्योंकि { $file } में Format वाला कोई [Catalogue] भाग नहीं है।

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = इस सूची का फ़ॉर्मैट Trindle के इस संस्करण के पढ़ सकने वाले फ़ॉर्मैट से नया है। Trindle Format={ $format } को पढ़ सकता है।

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } छोड़ दिया गया है: { $why ->
        [id] इसकी id में अक्षर, अंक, बिंदु और डैश के अलावा कुछ और है
        [needs] इसमें Name और Version दोनों चाहिए
        [archive] इसका Archive सूची के बगल वाली किसी .tar.zst फ़ाइल का नाम नहीं है
        [hash] इसका SHA256 64 हेक्स अंकों का नहीं है
        [permission] यह ऐसी अनुमति माँगता है जो मौजूद ही नहीं है
       *[icon] इसका Icon सूची के बगल वाली किसी SVG या PNG फ़ाइल का नाम नहीं है
    }।

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar शुरू नहीं हुआ: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle आर्काइव खोल नहीं सका: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = कारण पता नहीं

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = आर्काइव के सबसे ऊपरी स्तर पर, या सबसे ऊपरी स्तर के एक फ़ोल्डर में, { $file } नहीं है।

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle अस्थायी फ़ोल्डर नहीं बना सका: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl शुरू नहीं हुआ: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = डाउनलोड नहीं हो सका: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = डाउनलोड की गई फ़ाइल सूची के चेकसम (SHA-256) से मेल नहीं खाती, इसलिए Trindle ने कुछ भी इंस्टॉल नहीं किया।

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = प्लगइन जाँच में विफल रहा, इसलिए Trindle ने कुछ भी इंस्टॉल नहीं किया: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = आर्काइव में { $wanted } नहीं, { $found } है, इसलिए Trindle ने कुछ भी इंस्टॉल नहीं किया।

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle ने प्लगइन इंस्टॉल कर दिया, पर यह दर्ज नहीं कर सका कि वह Trindle की सूची से आया है: { $why }।

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = यह इंस्टॉल नहीं है, इसलिए अपडेट करने को कुछ नहीं है।
