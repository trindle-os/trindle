
# machine translated (Urdu), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = یہ فائل محفوظ طریقے سے ڈاؤن لوڈ ہوئی، لیکن اسے کسی نے جانچا نہیں، کیونکہ اس کا بنانے والا کوئی دستخط شائع نہیں کرتا۔

# What a project with nothing to install here is told as.
store-gh-no-files = اس منصوبے کے ریلیزز میں اس کمپیوٹر کے لیے کوئی AppImage یا Flatpak فائل نہیں ہے۔

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub پر اس منصوبے کی کوئی ریلیز نہیں ہے۔

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = کی رنگ میں کلید تلاش کی جا رہی ہے

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = ایک کلید محفوظ ہے

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = کوئی کلید محفوظ نہیں

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = اس کمپیوٹر میں کی رنگ نہیں ہے، اس لیے وہ کلید محفوظ نہیں کر سکتا

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = آپ کی GitHub کلید ابھی اس کمپیوٹر پر ایک پرانی سادہ فائل میں ہے، جسے Trindle اب نہیں پڑھتا۔ { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } اس لیے Trindle GitHub کو بغیر کلید کے استعمال کرتا ہے، چاہے کوئی کلید محفوظ ہو۔

# SAFETY: needs a human review before it ships
# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = Trindle آپ کی محفوظ GitHub کلید نہیں بھیج سکتا، کیونکہ اس میں خالی جگہ یا نئی سطر ہے۔ اس لیے Trindle GitHub کو بغیر کلید کے استعمال کرتا ہے۔ کلید ہٹائیں اور اسے دوبارہ چسپاں کریں۔

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = فائل ہٹائی نہیں جا سکی۔

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = کی رنگ نے اسے محفوظ نہیں کیا، اس لیے Trindle GitHub کو کلید کے بغیر استعمال کرتا ہے۔

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = کلید ایک سطر ہوتی ہے جس میں خالی جگہ نہیں ہوتی۔ اس کے اندر نئی سطر یا خالی جگہ ہے۔ صرف کلید چسپاں کریں۔

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub نے فی الحال جواب دینا بند کر دیا ہے، کیونکہ { $which }۔ { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub نے فی الحال جواب دینا بند کر دیا ہے، کیونکہ { $which }۔ { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = بعد میں دوبارہ کوشش کریں۔

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] یہ تقریباً ایک منٹ میں دوبارہ جواب دے گا۔
       *[other] یہ تقریباً { $n } منٹ میں دوبارہ جواب دے گا۔
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = یہ فی منٹ 30 تلاشوں کی اجازت دیتا ہے

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = یہ فی منٹ 10 تلاشوں کی اجازت دیتا ہے

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = آپ اپنی کلید کی اجازت والی فی گھنٹہ 5,000 درخواستیں استعمال کر چکے ہیں

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = یہ بغیر کلید کے ایک پتے سے فی گھنٹہ 60 درخواستوں کی اجازت دیتا ہے

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = GitHub کلید اسے 5,000 تک بڑھا دیتی ہے۔ تلاش کے اوپر دائیں جانب کلید کا بٹن دبائیں۔

# SAFETY: needs a human review before it ships
# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub نے کلید قبول نہیں کی۔ تلاش کے اوپر دائیں جانب کلید کے بٹن سے نئی بنائیں، یا اسے ہٹا دیں۔

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub سے { $code } موصول ہوا۔

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub نے اسے روک دیا، حالانکہ آپ حد تک نہیں پہنچے۔

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub نے اسے روک دیا: { $why }۔

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] Trindle ملا ہوا 1 منصوبہ نہیں دیکھ سکا، اس لیے وہ فہرست میں نہیں ہے
       *[other] Trindle ملے ہوئے { $n } منصوبے نہیں دیکھ سکا، اس لیے وہ فہرست میں نہیں ہیں
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }۔

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }۔

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub تک رسائی نہیں ہو سکی۔

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Trindle نے صرف AppImage والے منصوبے تلاش کیے، کیونکہ GitHub نے Flatpak کی تلاش کا جواب نہیں دیا۔ { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = ڈاؤن لوڈ ہو رہی ہے

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = انسٹال ہو رہی ہے

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = ڈاؤن لوڈ مکمل نہیں ہوا۔ { $why }

# The size of a download that is under a megabyte.
store-size-under = 1 MB سے کم
