
# machine translated (Turkish), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Pil %{ $pct }

# Line under the low battery warning's title.
ring-battery-plug-in = Lütfen şarj aletini kısa süre içinde takın.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = Şarj oluyor

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Şarj aleti çıkarıldı

# Line under the title when the charger comes out.
ring-battery-running = Pille çalışıyor, %{ $pct }

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 dakika

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 saat

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 saat

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = Ertesi sabah

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = 10 dakika ertele

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = 1 saat ertele

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = 6 saat ertele

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Ertesi sabaha ertele

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = İletiye geri dön

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Sabitlemeyi kaldır

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Sabitle

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Ertele

# SAFETY: needs a human review before it ships
# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = Tümünü temizle

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Kopyalandı

# Tip of the button on a notification circle that copies its message.
ring-act-copy = İletiyi kopyala

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Asistanınıza vermek için tıklayın.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = Trindle masaüstü

# Name of the login screen in the crash notice.
ring-crash-greeter = Trindle giriş ekranı

# Name of the lock screen in the crash notice.
ring-crash-lock = Trindle kilit ekranı

# Name of Settings in the crash notice.
ring-crash-settings = Trindle Ayarlar

# Name of Apps in the crash notice.
ring-crash-apps = Trindle Uygulamalar

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = Trindle { $name }

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } beklenmedik biçimde durdu

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = Bu sürücü artık yok

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = { $name } açılamadı

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Uyku

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Derin uyku

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Ekranı kilitle

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Hiçbir şey

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Kapat

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = Kapağı kapatmak, Ayarlar'daki Güç bölümünde belirtileni yapar

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Büyütmek için herhangi bir yere basılı tutun

# Screen reader name of the magnifier's glass.
ring-magnify-name = Büyüteç

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Büyütmeyi durdur

# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = Mikrofonu tüm programlar için kapat

# SAFETY: needs a human review before it ships
# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = Mikrofonu yeniden aç

# SAFETY: needs a human review before it ships
# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = Mikrofonu yalnızca sizin basmanız yeniden açabilir.

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = Mikrofon anahtarı değiştirilemedi: { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = Program bulunamadı

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Başka bir program gönderdi

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Başlatılamadı

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Asistanınız için kopyalandı

# Screen reader name of a notification circle.
ring-notes-name = Bildirim

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Taşınıyor. Bırakmak için fareyi bırakın, geri koymak için Escape'e basın.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Taşındı

# Said when Escape put a held notification back.
ring-notes-put-back = Geri konuldu

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Taşımak için önce sabitleyin

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
        [one] { $n } saniye
       *[other] { $n } saniye
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Siz kapatana kadar

# Time under a notification from yesterday.
ring-notify-yesterday = Dün { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = Sağ alt

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = Sol alt

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = Sol üst

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = Sağ üst

# SAFETY: needs a human review before it ships
# Title of the notice when a plugin that asks for permissions has been given none.
ring-plugin-needs = { $name } izninize ihtiyaç duyor

# SAFETY: needs a human review before it ships
# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
ring-plugin-cannot-work = Henüz çalışamaz: istenen { $asks } izinden 0'ı verildi.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Kopyalandı. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = Ağa ulaştı

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = Dosyalarınızı açık tutuyor

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
        [one] { $names } ve { $more } tane daha
       *[other] { $names } ve { $more } tane daha
    }

