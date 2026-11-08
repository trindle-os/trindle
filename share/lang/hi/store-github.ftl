# SAFETY: needs a human review before it ships
# machine translated (Hindi), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = सुरक्षित रूप से डाउनलोड हुई, पर इस फ़ाइल को किसी ने जाँचा नहीं है: इसके निर्माता कोई हस्ताक्षर प्रकाशित नहीं करते।

# What a project with nothing to install here is told as.
store-gh-no-files = इस प्रोजेक्ट की रिलीज़ में इस कंप्यूटर के लिए कोई AppImage या Flatpak फ़ाइल नहीं है।

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub पर इस प्रोजेक्ट की कोई रिलीज़ नहीं है।

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = कीरिंग में की खोजी जा रही है

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = एक की रखी है

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = कोई की रखी नहीं है

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = इस कंप्यूटर पर कीरिंग नहीं है, इसलिए कोई की रखी नहीं जा सकती

# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = आपकी GitHub की अब भी इस कंप्यूटर की एक सादी फ़ाइल में है और वहीं से इस्तेमाल हो रही है: उसे कीरिंग में नहीं ले जाया जा सका। { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } इसलिए GitHub से बिना की के पूछा जाता है, चाहे कोई रखी हो या नहीं।

# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = एक GitHub की रखी है, पर उसे भेजा नहीं जा सकता: उसके अंदर स्पेस या नई पंक्ति है, जैसा दो टुकड़ों में चिपकाई गई की में होता है। इसलिए GitHub से बिना की के पूछा जाता है। उसे हटाएँ और की फिर से चिपकाएँ।

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = फ़ाइल हटाई नहीं जा सकी।

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = कीरिंग ने इसे नहीं लिया।

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = की एक पंक्ति की होती है जिसमें स्पेस नहीं होता, और इसके अंदर नई पंक्ति या स्पेस है। केवल की चिपकाएँ।

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub ने अभी के लिए जवाब देना बंद कर दिया है: { $which }। { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub ने अभी के लिए जवाब देना बंद कर दिया है: { $which }। { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = बाद में फिर कोशिश करें।

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] यह लगभग एक मिनट में फिर जवाब देगा।
       *[other] यह लगभग { $n } मिनट में फिर जवाब देगा।
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = यह एक मिनट में 30 खोजों का जवाब देता है

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = यह एक मिनट में 10 खोजों का जवाब देता है

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = यह आपकी की के साथ एक घंटे में 5,000 सवालों का जवाब देता है, और वे खत्म हो चुके हैं

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = यह बिना की के एक पते से एक घंटे में 60 सवालों का जवाब देता है

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = GitHub की इसे 5,000 तक बढ़ा देती है: GitHub खोजें के ऊपर दाईं ओर की चाबी वाला बटन दबाएँ।

# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub ने की स्वीकार नहीं की। नई बनाएँ, या इसे हटा दें: GitHub खोजें के ऊपर दाईं ओर की चाबी वाला बटन दबाएँ।

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub ने { $code } कहा।

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub ने इसे मना कर दिया, जबकि उसके जवाब खत्म नहीं हुए हैं।

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub ने इसे मना कर दिया: { $why }।

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] मिला 1 प्रोजेक्ट देखा नहीं जा सका, इसलिए वह सूची में नहीं है
       *[other] मिले { $n } प्रोजेक्ट देखे नहीं जा सके, इसलिए वे सूची में नहीं हैं
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }।

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }।

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub तक पहुँचा नहीं जा सका।

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = केवल AppImage वाले प्रोजेक्ट खोजे गए: GitHub ने Flatpak की खोज का जवाब नहीं दिया। { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = डाउनलोड हो रहा है

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = इंस्टॉल हो रहा है

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = डाउनलोड पूरा नहीं हुआ। { $why }

# The size of a download that is under a megabyte.
store-size-under = 1 MB से कम
