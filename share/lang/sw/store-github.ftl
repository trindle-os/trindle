# machine translated (Swahili), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Imepakuliwa kwa usalama, lakini hakuna aliyekagua faili hii: mtengenezaji wake hachapishi sahihi.

# What a project with nothing to install here is told as.
store-gh-no-files = Matoleo ya mradi huu hayana faili ya AppImage au Flatpak kwa kompyuta hii.

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub haina matoleo ya mradi huu.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Inatafuta ufunguo kwenye keyring

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Ufunguo umehifadhiwa

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Hakuna ufunguo uliohifadhiwa

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Hakuna keyring kwenye kompyuta hii, kwa hiyo hakuna ufunguo unaoweza kuhifadhiwa

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = Ufunguo wako wa GitHub bado uko kwenye faili ya kawaida kwenye kompyuta hii, na unatumika kutoka hapo: haukuweza kuhamishiwa kwenye keyring. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Kwa hiyo GitHub inaulizwa bila ufunguo, iwe umehifadhiwa au la.

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = Ufunguo wa GitHub umehifadhiwa, lakini hauwezi kutumwa: una nafasi au mstari mpya ndani yake, kama ufunguo uliobandikwa kwa vipande viwili. Kwa hiyo GitHub inaulizwa bila ufunguo. Uondoe na ubandike ufunguo tena.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Faili haikuweza kuondolewa.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Keyring haikuupokea.

# Red words when a pasted key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-key-spaces = Ufunguo ni mstari mmoja usio na nafasi ndani yake, na huu una mstari mpya au nafasi ndani yake. Bandika ufunguo peke yake.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub imeacha kujibu kwa sasa: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub imeacha kujibu kwa sasa: { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Jaribu tena baadaye.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Itajibu tena baada ya takriban dakika moja.
       *[other] Itajibu tena baada ya takriban dakika { $n }.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = hujibu utafutaji 30 kwa dakika

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = hujibu utafutaji 10 kwa dakika

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = hujibu maswali 5,000 kwa saa kwa ufunguo wako, na yameisha

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = hujibu maswali 60 kwa saa kutoka anwani moja bila ufunguo

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Ufunguo wa GitHub huongeza hadi 5,000: bonyeza ufunguo ulio juu kulia wa Tafuta GitHub.

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub haikukubali ufunguo. Tengeneza mpya, au uuondoe: bonyeza ufunguo ulio juu kulia wa Tafuta GitHub.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub ilijibu { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub ilikataa hili, ingawa majibu yake hayajaisha.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub ilikataa hili: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] Mradi 1 uliopatikana haukuweza kuangaliwa, kwa hiyo haujaorodheshwa
       *[other] Miradi { $n } iliyopatikana haikuweza kuangaliwa, kwa hiyo haijaorodheshwa
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub haikuweza kufikiwa.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Ni miradi yenye AppImage pekee iliyotafutwa: GitHub haikujibu utafutaji wa Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Inapakua

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Inasakinisha

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = Upakuaji haukukamilika. { $why }

# The size of a download that is under a megabyte.
store-size-under = chini ya MB 1
