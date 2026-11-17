
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Ekran

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = ekran monitör çözünürlük boyut ölçek yakınlaştır büyük küçük döndür yenileme gece ışığı sıcak mavi ışık akşam gözler

# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = Değişiklikler hemen uygulanır. Ekran bozulursa hiçbir şey yapmayın. Tut düğmesine basmazsanız 15 saniye sonra kendiliğinden eski haline döner. Gece ışığı akşamları ekranı daha sıcak yapar.

# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = Bu düzen tutulsun mu?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Ekranlar denetleniyor…

# Sentence at the top of the page when no display was found.
settings-display-none-found = Ekran bulunamıyor.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = Kullanılan ekran yok

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Bir ekran kullanımda

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] { $n } ekran kullanımda
       *[other] { $n } ekran kullanımda
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = gece ışığı kapalı

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = gece ışığı ekranı { $from } ile { $to } arasında sıcaklaştırır

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = gece ışığı ekranı gün batımından gün doğumuna kadar sıcaklaştırır

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Normal

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Sağa döndürülmüş

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Sola döndürülmüş

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = Ters çevrilmiş

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = Şunun sağında

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = Şunun solunda

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Şunun üstünde

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Şunun altında

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height }, { $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (önerilen)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = %{ $percent }  (standart)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = { $display } çözünürlüğü

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = { $display } üzerindeki boyut

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = { $display } dönüşü

# Heading of the resolution line.
settings-display-resolution = Çözünürlük

# Heading of the size line.
settings-display-size = Boyut

# Heading of the rotation line.
settings-display-rotation = Dönüş

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Bu ekranı kullan

# Small words under that line for the only display in use.
settings-display-use-only = Kullanımdaki tek ekranı kapatamazsınız.

# Words behind the i of a display's card.
settings-display-card-about = Değişiklikler hemen uygulanır. Ekran bozulursa hiçbir şey yapmayın. Tut düğmesine basmazsanız 15 saniye sonra kendiliğinden eski haline döner. Önerilen çözünürlük, ekranın tasarlandığı çözünürlüktür. Standart boyut ise Trindle'ın bu ekran için seçtiği boyuttur.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = { $display } yerleşimi

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Yerleştir

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display }, { $other } ekranına göre

# Title of the card about where displays sit.
settings-display-arrangement-title = Düzen

# Words behind the i of that card.
settings-display-arrangement-about = İmleç ortak kenardan geçebilsin diye ekranların birbirinin yanında nerede durduğunu ayarlayın.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Birini ötekinin yanına yerleştirmek için iki ekran gerekir.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Bu değişiklik yapılamadı.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
        [one] Ekranlar değişti. Tut düğmesine basmazsanız { $seconds } saniye sonra geri dönerler.
       *[other] Ekranlar değişti. Tut düğmesine basmazsanız { $seconds } saniye sonra geri dönerler.
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Değişiklik geri alınamadı. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = Düzen şu anda kullanımda ama Trindle bunu sonraki oturum açma için kaydedemedi.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Hiçbir şey değişmedi

# Small words under that line.
settings-display-refused-change = Bu değişiklik yapılamadı.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Değişiklikler hemen uygulanır

# Title of the night light card.
settings-display-night-title = Gece ışığı

# Words behind the i of that card.
settings-display-night-about = Ekran akşamları daha sıcak olur, bu da gözler için daha rahattır. Saat diliminizden hesaplanan gün batımı ve gün doğumunu ya da kendi belirlediğiniz saatleri izler. Trindle saat diliminizden nerede olduğunuzu anlayamazsa belirlediğiniz saatleri kullanır.

# Heading of the night light line, and name of its switch.
settings-display-night = Gece ışığı

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = { $zone } bölgesinde gün batımı ve gün doğumu.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle gün batımının ne zaman olduğunu bilmediği için belirlenen saatleri kullanıyor.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Gün batımından gün doğumuna

# Choice in the When list: between times the person sets.
settings-display-when-times = Belirlenen saatler

# Heading of the When line, and name of its list.
settings-display-when = Ne zaman

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = Başlangıç

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = Bitiş

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Sıcaklık

# Small word under the left end of the warmth slider.
settings-display-little = Biraz

# Small word under the right end of the warmth slider.
settings-display-very = Çok sıcak

# Spoken description of the strip of colours night light runs through.
settings-display-strip = Biraz sıcaktan çok sıcağa

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Kullanımda ekran yok

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] { $n } ekran
       *[other] { $n } ekran, düzenlendikleri gibi
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Ekran yok

# Name of the large picture of the displays.
settings-display-your-displays = Ekranlarınız

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle hiç ekran bulamadı.

# Title of the card shown when no display was found.
settings-display-none-title = Ekran bulunamadı

# Heading of the line in that card.
settings-display-none-heading = Ekran bulunamadı

# Title of the card about the graphics card.
settings-display-graphics-title = Grafikler

# Words behind the i of that card.
settings-display-graphics-about = Bu bilgisayarda açık sürücüyü kullanan bir NVIDIA kartı var. Çalışıyor ama yaklaşık 2022 ve sonrası bir kart yavaş hissettirir, çünkü açık sürücü kartın hızını yükseltemez. Bunun yerine NVIDIA'nın kendi sürücüsünü kurabilirsiniz.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Açık sürücülü NVIDIA kartı

# Small words under that line.
settings-display-nvidia-about = 2022 ve sonrası kartlar yavaş çalışabilir.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Bunun için ne yapmalı
