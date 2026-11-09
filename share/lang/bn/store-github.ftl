# machine translated (Bengali), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
# SAFETY: needs a human review before it ships
store-gh-unchecked = নিরাপদে ডাউনলোড হয়েছে, কিন্তু কেউ এই ফাইল যাচাই করেনি: এর নির্মাতা কোনো স্বাক্ষর প্রকাশ করেননি।

# What a project with nothing to install here is told as.
store-gh-no-files = এই প্রজেক্টের রিলিজে এই কম্পিউটারের জন্য কোনো AppImage বা Flatpak ফাইল নেই।

# What a project that has never made a release is told as.
store-gh-no-releases = এই প্রজেক্টের কোনো রিলিজ GitHub-এ নেই।

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = কীরিংয়ে কী খোঁজা হচ্ছে

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = একটি কী রাখা আছে

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = কোনো কী রাখা নেই

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = এই কম্পিউটারে কোনো কীরিং নেই, তাই কী রাখা যাবে না

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = আপনার GitHub কী এখনও এই কম্পিউটারের একটি সাধারণ ফাইলে আছে এবং সেখান থেকেই ব্যবহার হচ্ছে: সেটি কীরিংয়ে সরানো যায়নি। { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } তাই কী থাকুক বা না থাকুক, GitHub-কে কী ছাড়াই জিজ্ঞাসা করা হচ্ছে।

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = একটি GitHub কী রাখা আছে, কিন্তু সেটি পাঠানো যাবে না: এর ভিতরে স্পেস বা লাইন ভাঙা আছে, যেমন দুই টুকরোয় পেস্ট করা কীতে হয়। তাই GitHub-কে কী ছাড়াই জিজ্ঞাসা করা হচ্ছে। সেটি সরিয়ে কীটি আবার পেস্ট করুন।

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = ফাইলটি সরানো যায়নি।

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = কীরিং সেটি নেয়নি।

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = কী হলো এক লাইনে, স্পেস ছাড়া, কিন্তু এতে লাইন ভাঙা বা স্পেস আছে। শুধু কীটি পেস্ট করুন।

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub আপাতত সাড়া দেওয়া বন্ধ করেছে: { $which }। { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub আপাতত সাড়া দেওয়া বন্ধ করেছে: { $which }। { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = পরে আবার চেষ্টা করুন।

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] প্রায় এক মিনিট পরে আবার সাড়া দেবে।
       *[other] প্রায় { $n } মিনিট পরে আবার সাড়া দেবে।
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = এটি প্রতি মিনিটে ৩০টি খোঁজার উত্তর দেয়

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = এটি প্রতি মিনিটে ১০টি খোঁজার উত্তর দেয়

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = আপনার কী দিয়ে এটি প্রতি ঘণ্টায় ৫,০০০টি প্রশ্নের উত্তর দেয়, আর সেগুলো শেষ হয়ে গেছে

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = কী ছাড়া একটি ঠিকানা থেকে এটি প্রতি ঘণ্টায় ৬০টি প্রশ্নের উত্তর দেয়

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = GitHub কী দিলে সীমা ৫,০০০ হয়: Search GitHub-এর উপরে ডানদিকের কী চাপুন।

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub কীটি গ্রহণ করেনি। নতুন একটি বানান, অথবা এটি সরিয়ে দিন: Search GitHub-এর উপরে ডানদিকের কী চাপুন।

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub উত্তর দিয়েছে { $code }।

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub এটি প্রত্যাখ্যান করেছে, যদিও উত্তরের সীমা শেষ হয়নি।

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub এটি প্রত্যাখ্যান করেছে: { $why }।

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] পাওয়া ১টি প্রজেক্ট দেখা যায়নি, তাই তালিকায় নেই
       *[other] পাওয়া { $n }টি প্রজেক্ট দেখা যায়নি, তাই তালিকায় নেই
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }।

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }।

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub-এ পৌঁছানো যায়নি।

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = শুধু AppImage আছে এমন প্রজেক্ট খোঁজা হয়েছে: Flatpak খোঁজার উত্তর GitHub দেয়নি। { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = ডাউনলোড হচ্ছে

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = ইনস্টল হচ্ছে

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = ডাউনলোড শেষ হয়নি। { $why }

# The size of a download that is under a megabyte.
store-size-under = 1 MB-এর কম
