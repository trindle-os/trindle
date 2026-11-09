
# machine translated (Turkish), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Güvenli olarak indirildi, ama bu dosyayı kimse denetlemedi: yapımcısı imza yayımlamıyor.

# What a project with nothing to install here is told as.
store-gh-no-files = Bu projenin sürümlerinde bu bilgisayar için AppImage ya da Flatpak dosyası yok.

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub'da bu proje için sürüm yok.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Anahtar için anahtar halkasına bakılıyor

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Bir anahtar kayıtlı

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Kayıtlı anahtar yok

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Bu bilgisayarda anahtar halkası yok, bu yüzden anahtar saklanamaz

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = GitHub anahtarınız hâlâ bu bilgisayarda düz bir dosyada ve oradan kullanılıyor: anahtar halkasına taşınamadı. { $why }

# Red words: the keyring could not say; why is given first.
# SAFETY: needs a human review before it ships
store-gh-keyis-hidden = { $why } Bu yüzden anahtar kayıtlı olsun olmasın GitHub'a anahtarsız soruluyor.

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = Bir GitHub anahtarı kayıtlı, ama gönderilemiyor: iki parça halinde yapıştırılan bir anahtardaki gibi içinde boşluk ya da satır sonu var. Bu yüzden GitHub'a anahtarsız soruluyor. Anahtarı kaldırıp yeniden yapıştırın.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Dosya kaldırılamadı.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Anahtar halkası anahtarı almadı.

# Red words when a pasted key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-key-spaces = Anahtar boşluksuz tek bir satırdır; bunun içinde satır sonu ya da boşluk var. Yalnızca anahtarı yapıştırın.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub şimdilik yanıt vermeyi bıraktı: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub şimdilik yanıt vermeyi bıraktı: { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Daha sonra yeniden deneyin.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Yaklaşık bir dakika sonra yeniden yanıt verecek.
       *[other] Yaklaşık { $n } dakika sonra yeniden yanıt verecek.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = dakikada 30 aramaya yanıt veriyor

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = dakikada 10 aramaya yanıt veriyor

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = anahtarınızla saatte 5.000 soruya yanıt veriyor ve bunlar tükendi

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = anahtarsız, tek bir adresten saatte 60 soruya yanıt veriyor

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = GitHub anahtarı bunu 5.000'e çıkarır: GitHub'da Ara sayfasının sağ üstündeki anahtar düğmesine basın.

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub anahtarı kabul etmedi. Yeni bir anahtar oluşturun ya da bunu kaldırın: GitHub'da Ara sayfasının sağ üstündeki anahtar düğmesine basın.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub { $code } yanıtını verdi.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub bunu reddetti, oysa yanıt hakkı tükenmedi.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub bunu reddetti: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] Bulunan 1 projeye bakılamadı, bu yüzden listelenmedi
       *[other] Bulunan { $n } projeye bakılamadı, bu yüzden listelenmedi
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub'a ulaşılamadı.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Yalnızca AppImage içeren projeler arandı: GitHub Flatpak aramasına yanıt vermedi. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = İndiriliyor

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Kuruluyor

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = İndirme bitmedi. { $why }

# The size of a download that is under a megabyte.
store-size-under = 1 MB altı

