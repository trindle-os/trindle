
# machine translated (Turkish), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Trindle'a hoş geldiniz

# Title of the second page, about opening the ring.
welcome-page-ring = Halkayı açın

# Title of the third page, where the first apps are picked.
welcome-page-apps = Uygulamalarınızı seçin

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Seçilecek daha çoğu

# Title of the fifth page, about plugins.
welcome-page-plugins = Eklentiler daha fazlasını ekler

# Title of the sixth page, about crash reports.
welcome-page-reports = Trindle'ın düzeltilmesine yardım edin

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Noktalar

# Title of the last page, with a few things worth knowing.
welcome-page-know = Bilmeye değer

# Sentence under the title of the first page.
welcome-sentence-hello = İhtiyacınız olan her şey tek bir dairede, halkada.

# Sentence under the title of the second page.
welcome-sentence-ring = Ekranın en kenarına çift tıklayın ya da Super tuşuna basın.

# Sentence under the title of the third page.
welcome-sentence-apps = Henüz neredeyse hiçbir şey kurulu değil, bu yüzden istediğiniz uygulamaları seçin.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Daha fazlasını seçin, sonra seçtiğiniz her şeyi tek tıkla kurun.

# Sentence under the title of the fifth page.
# SAFETY: needs a human review before it ships
welcome-sentence-plugins = Eklentiler, Uygulamalar'dan gelen küçük eklerdir ve her biri yalnızca izin verdiğiniz şeyi yapar.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Bir şey ters giderse, isterseniz bir rapor gönderin.

# Sentence under the title of the dots page.
welcome-sentence-dots = Bir nokta, bir programın size ait bir şeyi kullandığı anlamına gelir.

# Sentence under the title of the last page.
welcome-sentence-know = Trindle'ı farklı kılan birkaç şey.

# Top right of every page: where the person is in the tour.
welcome-step = Adım { $n } / { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Adım { $n } / { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = Hoş geldiniz ekranı görüldü. Tüm uygulamalarda Welcome adıyla bulunur; bir sonraki oturum açışta yeniden görmek için bu dosyayı silin.

# Name of the round button at the foot that closes the tour.
welcome-close = Kapat

# Name of the round button at the foot that goes to the page before.
welcome-back = Geri

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = İleri

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Seçilen uygulamaları kur

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Trindle'ı kullanmaya başla

# Foot of the apps pages while the computer is offline.
welcome-offline = Uygulama kurmak için internete bağlanın.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Yeniden dene

# Foot while apps are going in: their names.
welcome-installing = { $names } kuruluyor.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = { $names } kuruluyor, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } artık halkada.
       *[other] { $names } artık halkada.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Uygulamalar kurulamadı. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak reddetti.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Seçilenler: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } ve { $more } tane daha

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = Halka

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = İçinde neler var

# Name of the first thing the ring holds.
welcome-held-programs = Programlarınız

# Line under it.
welcome-held-programs-line = Dış çevrede durur.

# Name of the second thing the ring holds.
welcome-held-time = Saat ve tarih

# Line under it.
welcome-held-time-line = Ortada durur.

# Name of the third thing the ring holds.
welcome-held-switches = Anahtarlar

# Line under it.
welcome-held-switches-line = Ses, ekran, uyanık tut ve güç.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = İşaretçiyle

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Klavyeyle

# Note under the picture of the Super key.
welcome-ring-super-note = Super tuşunda çoğu zaman bir Windows logosu vardır.

# Head of a card of apps: web browsers.
welcome-kind-browser = Web tarayıcısı

# Head of a card of apps: email programs.
welcome-kind-email = E-posta

# Head of a card of apps: chat programs.
welcome-kind-chat = Sohbet

# Head of a card of apps: office programs.
welcome-kind-office = Ofis

# Head of a card of apps: music and video players.
welcome-kind-media = Müzik ve video

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Resim ve çizim

# Head of a card of apps: notes and writing.
welcome-kind-notes = Not ve yazı

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF ve belgeler

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Bunların hiçbiri şu anda Flathub'da yok.

# Beside an app that has been installed.
welcome-app-installed = Kurulu

# Beside an app that is going in now.
welcome-app-installing = Kuruluyor

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, seçildi

# Name of the round button on an app's row that picks it.
welcome-app-pick = { $name } uygulamasını seç

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = Halkada

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Eklentiler

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = Uygulamalar içinde, Eklentiler altında.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Eklentileri Uygulamalar'da aç

# Said in the plugins card while there are none to show.
# SAFETY: needs a human review before it ships
welcome-plugins-none = Sunulan eklentiler Uygulamalar'da listelenir.

# Head of the card on the crash reports page.
welcome-reports-card = Çökme raporları

# The one question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-question = Bir şey ters gittiğinde rapor göndererek Trindle'ın düzeltilmesine yardım etmek ister misiniz? Raporu her zaman önce siz görürsünüz.

# Note under the question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-note = Siz basmadan hiçbir şey gönderilmez. Raporun tamamını önce görürsünüz ve içinde dosya, parola ya da Wi-Fi adı bulunmaz.

# Second note under the question about crash reports.
welcome-reports-settings = Bunu istediğiniz zaman Ayarlar, Çökme raporları bölümünden değiştirebilirsiniz.

# The answer that turns crash reports off.
# SAFETY: needs a human review before it ships
welcome-reports-no = Hayır

# The answer that turns crash reports on.
# SAFETY: needs a human review before it ships
welcome-reports-yes = Evet, bana sor

# Name the screen reader gives to the choice between the two answers.
# SAFETY: needs a human review before it ships
welcome-reports-choice = Bir şey ters gittiğinde rapor gönder

# Head of a card on the last page.
welcome-know-boxes = Uygulamalar kendi kutularında yaşar

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = Bir uygulama dosyalarınıza, kameranıza ya da mikrofonunuza erişmeden önce sorar.

# Head of a card on the last page.
welcome-know-updates = Güvenlik güncellemeleri kendiliğinden kurulur

# Line in that card.
welcome-know-updates-line = Her gün, sormadan. Diğer güncellemeler sizi bekler.

# Head of a card on the last page.
welcome-know-ring = Ayarlar ve Uygulamalar halkada

# Line in that card.
welcome-know-ring-line = Ortasında, saatin üstünde.

# Head of a card on the last page.
welcome-know-help = Yardım Ayarlar'da

# Line in that card.
welcome-know-help-line = Bu tanıtım, tekrar istediğinizde Tüm uygulamalar içinde Welcome adıyla bulunur.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Üzerinde yalnızca ortada halka olan küçük bir ekran: ortasında saat, çevresinde sık kullanılan uygulamalar.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = Küçük bir ekranda açık halka; ortasında, saatin altında, kurulduktan sonra bir eklentinin simgesinin durduğu yerde birkaç eklentinin simgesi bir sırada.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Noktaların göründüğü yerlerin iki küçük çizimi: halkadaki bir pencerenin sekmesinde ve ekranın üst ortasında.

welcome-dots-where-tab = Bir pencerenin sekmesinde

welcome-dots-where-corner = Ekranın üst ortası

welcome-picture-ring = Küçük bir ekran. İşaretçi en üst kenara gider ve çift tıklar, halka ortada açılır.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = Halkayı açan Super tuşuna basılıyor.

# Title of the first page, which asks how much the person knows.
welcome-page-level = Ne kadar biliyorsunuz?

# Sentence under that title.
welcome-sentence-level = En yakın olanı seçin; tanıtım bildiklerinizi atlar.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Linux'a yeniyim

# Line in that answer's card.
welcome-level-new-line = Her sayfa, her Linux masaüstünün yaptıkları dahil.

# Name of the answer for someone who has used Linux.
welcome-level-linux = Daha önce Linux kullandım

# Line in that answer's card.
welcome-level-linux-line = Yalnızca Trindle'a özgü olanlar. Linux temelleri atlanır.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Daha önce Trindle kullandım

# Line in that answer's card.
welcome-level-trindle-line = Tanıtımı şimdi kapatır ve bir daha göstermez.

# Title of the page about files and folders.
welcome-page-files = Dosyalarınız

# Sentence under that title.
welcome-sentence-files = Size ait her şey ev klasörünüzde durur.

# Title of the page about getting software.
welcome-page-software = Yazılım edinmek

# Sentence under that title.
welcome-sentence-software = Uygulamalar, indirdiğiniz kurulum dosyalarından değil, Uygulamalar'dan gelir.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = Yönetici parolası

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Tüm bilgisayarı değiştiren işlemler önce parola ister.

# Title of the page about snapshots.
welcome-page-snapshots = Anlık görüntüler

# Sentence under that title.
welcome-sentence-snapshots = Trindle sistemin kopyalarını tutar, böylece bir güncelleme geri alınabilir.

# Title of the page about the person's own words.
welcome-page-words = Kendi sözcükleriniz

# Sentence under that title.
welcome-sentence-words = Trindle'ın gösterdiği herhangi bir sözcüğü değiştirin ya da kendi dilinizi ekleyin.

# Head of a card on the files page.
welcome-files-home = Bir ev klasörü

# Line in that card.
welcome-files-home-line = Belgeler, İndirilenler ve Resimler onun içindedir. Folders orada açılır.

# Head of a card on the files page.
welcome-files-drives = Sürücü harfi yok

# Line in that card.
welcome-files-drives-line = Bir USB bellek ya da başka bir disk, taktığınızda Folders içinde görünür.

# Head of a card on the files page.
welcome-files-hidden = Gizli adlar

# Line in that card.
welcome-files-hidden-line = .config gibi nokta ile başlayan bir ad gizlidir. Bir uygulamanın ayarlarını tutar.

# Head of a card on the files page.
welcome-files-bin = Çöp kutusu

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = Silinen bir dosya, siz boşaltana kadar çöp kutusunda bekler.

# Head of a card on the software page.
welcome-software-apps = Uygulamalar için tek yer

# Line in that card.
welcome-software-apps-line = Programları bulmak, kurmak ve kaldırmak için halkadan Uygulamalar'ı açın.

# Head of a card on the software page.
welcome-software-exe = Kurulum dosyası yok

# Line in that card.
welcome-software-exe-line = .exe ile biten bir Windows kurulum dosyası burada çalışmaz. Uygulamayı Uygulamalar'da arayın.

# Head of a card on the software page.
welcome-software-updates = Güncellemeler tek yerde

# Line in that card.
welcome-software-updates-line = Uygulamalar ve Ayarlar her şeyi güncel tutar, bu yüzden hiçbir uygulamanın kendi güncelleyicisine gerek kalmaz.

# Head of a card on the software page.
welcome-software-remove = Kaldırma temizdir

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Bir uygulamayı kaldırmak onu tamamen siler ve geride hiçbir şey bırakmaz.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = Neden sorar

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Herkes için kurmak ya da bir diski değiştirmek her kullanıcıyı etkiler, bu yüzden bilgisayar önce sorar.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = Kimin parolası

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = Bir yöneticinin parolası, genellikle oturum açtığınız hesabınki.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = Ne zaman sormaz

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Yalnızca kendiniz için kurduğunuz uygulamalar ve kendi dosyalarınız buna asla ihtiyaç duymaz.

# Head of a card on the snapshots page.
welcome-snap-taken = Sizin için alınır

# Line in that card.
welcome-snap-taken-line = Btrfs sistemde, her yazılım değişikliğinden önce ve sonra bir tane alınır.

# Head of a card on the snapshots page.
welcome-snap-list = Ayarlar'da listelenir

# Line in that card.
welcome-snap-list-line = Ayarlar, Anlık görüntüler bunları listeler. Kamera simgesi şimdi bir tane alır.

# Head of a card on the snapshots page.
welcome-snap-boot = Başlamazsa kullanılır

# Line in that card.
welcome-snap-boot-line = Önyükleme menüsünde Trindle snapshots seçeneğini, sonra bir anlık görüntü seçin.

# Head of a card on the your own words page.
welcome-words-change = Bir sözcüğü değiştirin

# Line in that card.
welcome-words-change-line = Trindle'ın gösterdiği herhangi bir sözcüğü bulun ve kendi sözcüğünüzü yazın. "Open it, brew", "Click here to open the file" yerine geçebilir.

# Head of a card on the your own words page.
welcome-words-share = Bir sayfa paylaşın

# Line in that card.
welcome-words-share-line = Sözcüklerinizi dosya olarak dışa aktarın ya da bir arkadaşınızın gönderdiğini içe aktarın. Kullanılmadan önce denetlenir.

# Head of a card on the your own words page.
welcome-words-all = Tüm programlar bir arada

# Line in that card.
welcome-words-all-line = Trindle'ın tamamı sözcüklerini tek yerden okur, bu yüzden tek bir değişiklik her yerde görünür.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Ayarlar, Bölge ve dil içindedir. Yardım adım adım anlatır.

# Name of the round button that opens that Settings page.
welcome-words-open = Bölge ve dil'i aç
