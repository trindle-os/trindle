
# machine translated (Turkish), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Sorun bildir

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Bir şeyler ters gitti. Rapor gönderilsin mi?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = Bu, hatayı, bilgisayarın bir özetini ve günlüğün son satırlarını gönderir; adlar, e-posta adresleri, ağ adresleri ve Wi-Fi adları çıkarılır. Günlük satırlarında dosya adları ve web adresleri yine de bulunabilir. Dosyalarınızı ve parolalarınızı göndermez.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = Ne oldu?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Hangi uygulama?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Emin değilim

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Başka bir program

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = Masaüstü

# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
# SAFETY: needs a human review before it ships
report-send = Gönder

# Button at the bottom of the window when the person opened it themselves.
# SAFETY: needs a human review before it ships
report-cancel = Vazgeç

# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
# SAFETY: needs a human review before it ships
report-dont-send = Gönderme

# Button that opens the exact text of the report.
report-show-details = Ayrıntıları göster

# Button that closes the exact text of the report.
report-hide-details = Ayrıntıları gizle

# Button under the shown report text that copies it.
report-copy = Kopyala

# Button under the shown report text that saves it as a file in Downloads.
report-save = Kaydet

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Raporlar henüz gönderilemiyor.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Denetleniyor.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Aynı anda çok fazla rapor var.

# Written beside the Send button once the report has been sent.
report-already-sent = Zaten gönderildi.

# Written under the typing box when Save is pressed a second time.
report-already-saved = Zaten kaydedildi.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Bağlantı bekleniyor.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Bağlantı bekleniyor. { $why }

# Under the typing box after the report was sent.
report-sent = Gönderildi. Teşekkürler.

# Under the typing box while the report is being posted.
report-sending = Gönderiliyor.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Downloads/{ $name } olarak kaydedildi.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Buildmorphic kullanıcı adınızı yazın, e-posta adresinizi değil. @ kullanmayın.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Kullanıcı adı 2 ile 31 karakter uzunluğunda olmalı; küçük harf, rakam ya da tire kullanılabilir.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = GitHub adında yalnızca harf, rakam ve tire kullanılabilir.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle raporu daha sonrası için saklayamadı: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Kaydedilemedi: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = izin yok

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = sürücü dolu

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = klasör yok

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = bir sürücü hatası

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl eksik.

# Why a report could not be posted: the post could not be started.
report-not-posted = Gönderilemedi.

# Why a report could not be posted: the computer is offline.
report-no-connection = Bağlantı yok.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Sunucu şu kodu gönderdi: { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Ne olduğu hakkında birkaç sözcük daha yazın.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Günlük okunuyor.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Rapora eklenecek günlük yok, bu yüzden gönderilemez. Bunun yerine kaydedin.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = İsteğe bağlı resim. Yalnızca proje sahibi görür.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = PNG resmi seç

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Ekran görüntüsü al

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Resmi kaldır

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Resminiz, { $width } x { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = PNG resmi seç

report-picture-filter = PNG resimleri

# Under the picture while it is being read.
report-picture-reading = Okunuyor.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Ekran görüntüsü alınamadı.

report-picture-no-chooser = Trindle dosya seçiciyi açamadı.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = Bu bir PNG resmi değil.

report-picture-too-big = { $megabytes } MB'ı aşıyor.

report-picture-too-large = Resim çok büyük. En fazla { $side } piksel genişliğinde ve { $side } piksel yüksekliğinde olabilir.

report-picture-damaged = Resim bozuk.

report-picture-unreadable = Okunamadı.



report-sent-as-bug = Gönderildi. Hata raporu numaranız { $number }. Teşekkürler.


# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle beta aşamasında. Raporlar anonimdir: adlar, e-posta adresleri ve ev klasörleri çıkarılır. Kullanıcı adı isteğe bağlıdır.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic kullanıcı adı (isteğe bağlı)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle raporu gönderemedi. { $why } Yeniden denemek için Gönder'e basın.
