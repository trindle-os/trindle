
# machine translated (Indonesian), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
# SAFETY: needs a human review before it ships
store-gh-unchecked = Diunduh dengan aman, tetapi belum ada yang memeriksa file ini: pembuatnya tidak menerbitkan tanda tangan.

# What a project with nothing to install here is told as.
store-gh-no-files = Rilis proyek ini tidak punya file AppImage atau Flatpak untuk komputer ini.

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub tidak punya rilis untuk proyek ini.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Mencari kunci di gantungan kunci

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Sebuah kunci tersimpan

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Tidak ada kunci tersimpan

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Tidak ada gantungan kunci di komputer ini, jadi kunci tidak bisa disimpan

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = Kunci GitHub Anda masih ada di file biasa di komputer ini dan dipakai dari sana: kunci tidak bisa dipindahkan ke gantungan kunci. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Jadi GitHub ditanyai tanpa kunci, baik ada kunci tersimpan maupun tidak.

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = Sebuah kunci GitHub tersimpan, tetapi tidak bisa dikirim: ada spasi atau pindah baris di dalamnya, seperti kunci yang ditempel dalam dua bagian. Jadi GitHub ditanyai tanpa kunci. Hapus kunci itu dan tempel lagi.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = File tidak bisa dihapus.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Gantungan kunci tidak menerimanya.

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = Kunci adalah satu baris tanpa spasi, dan yang ini punya pindah baris atau spasi di dalamnya. Tempel kuncinya saja.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub berhenti menjawab untuk sementara: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub berhenti menjawab untuk sementara: { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Coba lagi nanti.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
       *[other] GitHub akan menjawab lagi dalam sekitar { $n } menit.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = GitHub menjawab 30 pencarian per menit

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = GitHub menjawab 10 pencarian per menit

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = GitHub menjawab 5.000 pertanyaan per jam dengan kunci Anda, dan jatahnya sudah habis

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = GitHub menjawab 60 pertanyaan per jam dari satu alamat tanpa kunci

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Kunci GitHub menaikkannya menjadi 5.000: tekan kunci di kanan atas Cari di GitHub.

# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub tidak menerima kunci itu. Buat yang baru, atau hapus: tekan kunci di kanan atas Cari di GitHub.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub menjawab { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub menolak ini, padahal jatah jawabannya belum habis.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub menolak ini: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
       *[other] { $n } proyek yang ditemukan tidak bisa dilihat, jadi tidak ditampilkan
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub tidak bisa dijangkau.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Hanya proyek dengan AppImage yang dicari: GitHub tidak menjawab pencarian Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Mengunduh

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Memasang

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = Unduhan tidak selesai. { $why }

# The size of a download that is under a megabyte.
store-size-under = di bawah 1 MB

