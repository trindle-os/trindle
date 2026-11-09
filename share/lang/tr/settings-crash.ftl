
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Çökme raporları

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = çökme raporları hatalar arızalar gönder gönderilen gizlilik telemetri buildmorphic

# SAFETY: needs a human review before it ships
# Help line at the foot of the Crash reports page.
settings-crash-help = Trindle programlarından biri çöktüğünde, çökme raporları eklentisi bu bilgisayarda kısa bir not tutar. Gönderme komutunu çalıştırıp her rapora evet demediğiniz sürece hiçbir şey göndermez; komut önce raporun tamamını gösterir. Adınız ve GitHub adınız yalnızca burada yazdıysanız rapora girer. Asla bir ileti, dosya adı, bulunduğunuz yer ya da diğer programlarla ilgili bir şey gönderilmez. Eklentiyi kaldırmak her şeyi durdurur.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Çökmeler

# Under the Crashes switch.
settings-crash-crashes-short = Trindle kodunun neresinde çöktüğü.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Çökmeler; programı, sürümünü, çökmenin türünü ve Trindle kodunun neresinde olduğunu gönderir.

# Heading and label of the switch for sending errors.
settings-crash-errors = Hatalar

# Under the Errors switch.
settings-crash-errors-short = Hatanın sabit adı ve yeri.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Hatalar; programı, sürümünü, Trindle'ın hataya verdiği sabit adı ve kodunun neresinde olduğunu gönderir.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = Bu bilgisayar hakkında

# Under the About this computer switch.
settings-crash-computer-short = Linux, işlemci, bellek, grafik, ekranlar.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = Bu bilgisayar hakkında, her rapora şunları ekler: çalışan Linux, işlemci türü, aşağı yuvarlanmış bellek, Wayland veya X11, grafiği kimin yaptığı ve kaç ekran olduğu.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Neler gönderilir

# SAFETY: needs a human review before it ships
# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } Aşağıdaki kutuya Buildmorphic kullanıcı adını yazmadıkça raporlar ad taşımaz; bu yalnızca sana övgü kazandırır.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Hiçbir şey değiştirilmedi, çünkü { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Hiçbir şey değiştirilmedi, çünkü onu saklayan dosya Trindle'ın yazdığı gibi değil.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Ayarlar klasörü yok.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Ayarlar klasörü oluşturulamadı.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Ayar yazılamadı.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = { $program } programında çökme ({ $signal })

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = { $program } programında { $code } hatası

# In the list of sent reports: the report was taken.
settings-crash-taken = Alındı

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Gönderilmedi: izleyici kapalı

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Gönderilmedi: çok büyük

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Gönderilmedi: reddedildi

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Gönderilmedi: aynı anda çok fazla

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Gönderilmedi

# Said beside the link line once it is saved.
settings-crash-saved = Kaydedildi

# SAFETY: needs a human review before it ships
# Tip of the button that clears the list of sent reports.
settings-crash-clear = Listeyi temizle

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = Liste temizlenemedi.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Kaldırıldı

# Heading of the line that holds the clear button.
settings-crash-kept-here = Yalnızca bu bilgisayarda saklanır

# Under the Kept on this computer only heading.
settings-crash-newest-first = En yeni olan başta, elliye kadar.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Henüz bir şey yok

# Under the Nothing yet heading.
settings-crash-listed-here = Her rapor burada, tam olarak gönderildiği gibi listelenir.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Tam olarak nelerin gönderildiğini göster

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
        [one] { $when }, { $n } kez: { $answer }
       *[other] { $when }, { $n } kez: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Şimdiye kadar gönderilenler

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Her rapor, tam olarak gönderildiği gibi, en yenisi başta listelenir ve yalnızca bu bilgisayarda saklanır. Alındı, Buildmorphic'in raporu aldığı anlamına gelir. Gönderilmedi: izleyici kapalı, Trindle'ın hata izleyicisinin rapor kabul etmediği anlamına gelir; reddedildi, yanlış oluşturulduğu için reddedildiği; aynı anda çok fazla, aynı anda çok fazla rapor geldiği anlamına gelir.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = Bir çökmeden sonra, raporun tamamı gösterilerek gönderilip gönderilmeyeceği size sorulur.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Hiçbir şey gönderilmez ve bir çökmeden sonra size sorulmaz.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Hiçbir şey gönderilmez: çökmeler ve hatalar ikisi de kapalı.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Trindle programlarındaki hatalar adsız olarak bildirilir.
        [both] Trindle programlarındaki çökmeler ve hatalar adsız olarak bildirilir.
       *[crashes] Trindle programlarındaki çökmeler adsız olarak bildirilir.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Sorunlar ve raporları paylaşma hakkında sor

# SAFETY: needs a human review before it ships
# Under the Ask about problems heading.
settings-crash-ask-sub = Kapalıyken her şey gizlenir ve bekleyen raporlar silinir.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Kapalı. Bekleyen rapor yoktu.

# SAFETY: needs a human review before it ships
# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
settings-crash-off-deleted =
    { $n ->
        [one] Kapalı. { $n } bekleyen rapor silindi.
       *[other] Kapalı. { $n } bekleyen rapor silindi.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = Topluluk hakkında

# Heading of the line with the community button.
settings-crash-go-to-bm = Raporlar Buildmorphic'e gider

# Under the Reports go to Buildmorphic heading.
settings-crash-community = Trindle'ın geliştirilmesine yardım eden topluluk.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = kullanıcı adı

# Heading of the line with the username box.
settings-crash-your-username = Buildmorphic kullanıcı adınız

# Under the username heading.
settings-crash-username-sub = İsteğe bağlı. Asla bir e-posta adresi değil.

# Heading of a line about joining with an email address.
settings-crash-join = Yalnızca bir e-posta adresiyle katılın

# Under the Join heading.
settings-crash-join-sub = İlk bildirdiğiniz sorunlar için kredi ve seviye kazanın.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Bir rapor yalnızca sabit bir liste içerir

# Under the fixed list heading.
settings-crash-fixed-sub = Önce siz görürsünüz ve siz basmadan hiçbir şey gitmez.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Gönderdiğiniz raporlar

# SAFETY: needs a human review before it ships
# Help text of the Reports you send card (the information button).
settings-crash-reports-about = Trindle'ın programlarından biri çöktüğünde, bir rapor gönderilip gönderilmeyeceğini soran bir pencere açılabilir. Önce tam metni gösterir: sözleriniz, bilgisayarın modeli ve grafiği, hata ve hemen öncesine ait günlük. Dosyalar, parolalar ve Wi-Fi adları asla okunmaz; adlar ve adresler silinir. Aynı pencereyi istediğiniz zaman halkadaki Sorun bildir düğmesinden de açabilirsiniz.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = github adı

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = GitHub adınız

# Small words under that heading.
settings-crash-github-about = İsteğe bağlı. Bildiren olarak gösterilir.

