
# machine translated (Turkish), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Henüz yazıcı yok. Ağınızda bulunanlar aşağıda listelenir.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } duraklatıldı, bu yüzden şimdilik onda hiçbir şey yazdırılmaz.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } yazdırıyor.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = { $name } ile yazdırmaya hazır.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } durumu: { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } kurulu; ilk yazdırılacak yazıcı seçilmedi.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } yazıcı kurulu; ilk yazdırılacak yazıcı seçilmedi.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Bu, CUPS'ın tanıdığı bir iş değil.

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = İptal edildi

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat başarısız oldu

# Words in the ring while the network is searched for printers.
settings-printers-searching = Aranıyor

# Words in the ring when the search is finished.
settings-printers-search-done = Bitti

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Artık varsayılan

# Shown beside a printer after a test page was sent.
settings-printers-sent = Gönderildi

# Shown beside a printer after it was removed.
settings-printers-removed = Kaldırıldı

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Önce yazıcının adresini yazın.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Ekleniyor...

# Shown beside a printer or the address line after it was added.
settings-printers-added = Eklendi

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Sürücüsü aranıyor...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = { $printer } üzerinde { $number } numaralı iş

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = Bu işi iptal et

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Yazdırma işleri

# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = Herhangi bir yazıcıda yazdırılmayı bekleyen her iş, şu anki haliyle. Bu sayfa açıkken liste birkaç saniyede bir yeniden okunur. Çarpı bir işi iptal eder; basılmış kâğıt basılı kalır.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS yanıt vermiyor

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Kuyruk okunamıyor.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Aranıyor…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = Kuyruk.

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Bekleyen yok

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = İşler yazdırılırken burada görünür.

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Duraklatıldı

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Boşta

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Yazdırıyor

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Varsayılan, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Önce bununla yazdır

# Tip of the button that prints a test page.
settings-printers-test-page = Deneme sayfası yazdır

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = Bu yazıcıyı kaldır

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Arama için cups-filters gerekir

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Şununla kurun: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Bu yazıcıyı ekle

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Yeni bir şey bulunamadı

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = Açık mı ve aynı ağda mı?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = Önce duyuranlar, sonra her makine sırayla.

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Ekle

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Adresiyle

# Small line under "By its address".
settings-printers-by-address-sub = Örneğin 192.168.1.50, yazıcının kendi ekranında yazar.

# Heading of the card for adding printers.
settings-printers-add-card = Yazıcı ekle

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Önce kendini duyuran yazıcılar aranır, sonra ağdaki her makineye sırayla sorulur. Yazıcı açık ve aynı ağda olmalıdır. Bazı yazıcılar kendini duyurmaz, bazı ağlar da duyuruyu iletmez: bunun yerine adresiyle ekleyin, örneğin 192.168.1.50. Adresi yazıcının kendi ekranında, ağ ayarlarının altında bulabilirsiniz.

# Name of the page in the list of settings pages.
settings-printers-title = Yazıcılar

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = yazıcı yazdır tarayıcı

# Help text at the top of the page.
settings-printers-help = Ağınızdaki yazıcılar kendiliğinden bulunur. Kullanmak için yanındaki Ekle'ye basın.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = Yazdırma bu bilgisayarda kurulu değil.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Aranıyor…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Yazdırma hizmeti yanıt vermiyor, bu yüzden şu an hiçbir şey yazdırılamaz.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Yazdırma

# Help text (i button) of that card.
settings-printers-printing-about = Yazdırma için CUPS gerekir, o da bu bilgisayarda yok. Kurulduktan sonra ağınızdaki yazıcılar kendiliğinden bulunur.

# Line in that card.
settings-printers-unavailable = Yazdırma kullanılamıyor

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Şununla kurun: sudo apt install cups

# Line in that card.
settings-printers-then-found = Sonra yazıcılar bulunur

# Small line under it.
settings-printers-then-found-sub = Ağınızdakiler, kendiliğinden.

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Yazdırma hizmeti.

# Line in the Printers card when there are none.
settings-printers-none = Henüz yazıcı yok

# Small line under "No printers yet".
settings-printers-none-sub = Bunun yanından bir tane ekleyin.

# Heading of the card that lists the printers.
settings-printers-card = Yazıcılar

# Help text (i button) of the Printers card.
settings-printers-card-about = Bu bilgisayarda kurulu her yazıcı. Yıldız birini ilk kullanılan yazıcı yapar; sayfa simgesi onda deneme sayfası yazdırır; çöp kutusu onu kaldırır ve iki kez sorar. Değişiklikler için hesabınızın lpadmin grubunda olması gerekir.

# Red words under a page range box that cannot be used.
printers-bad-range = 1-3, 5 gibi sayfalar

# Red words under a copies box that cannot be used.
printers-bad-copies = Kopya: 1 ile 99 arası

# Name of the choice to print on one side of the paper.
printers-sides-one = Tek yüz

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = İki yüz, uzun kenardan

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = İki yüz, kısa kenardan

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = Dikey

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Yatay

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = İzin verilmiyor. Hesabınızın lpadmin grubunda olması gerekir; olduktan sonra oturumu kapatıp yeniden açın.

# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS reddetti.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = Bu yazıcı adına CUPS izin vermiyor.

# Red words when printing is not installed on the computer.
printers-not-installed = Yazdırma bu bilgisayarda kurulu değil.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } artık kurulu değil.

# Red words when the printer chosen is paused.
printers-paused = { $name } duraklatıldı.

# Red words when sending the document to CUPS took too long.
printers-too-slow = Belgenin gönderilmesi çok uzun sürdü.
