
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Klavye ve fare

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = düzen dil yazma caps lock ekran klavyesi düğme halka fare dokunmatik yüzey touchpad işaretçi hız dokunma tıklama kaydırma tekerlek yavaş hızlı doğal sol elli eklenti kısayol tuşlar yeniden eşle değiştir takas ctrl escape super alt mac yan düğmeler dokunuş basılı tutma programların yaptığı keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Buradaki her şey hemen değişir. Alt ve Shift tuşlarına birlikte basarak ikinci bir düzene geçebilirsiniz.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Yazım düzeni: { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt ve Shift ile şuna geçilir: { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock hiçbir şey yapmaz.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock tuşu { $key } olarak çalışıyor.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = ikinci bir Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = Klavye düzenleri listesi bu bilgisayarda yok.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Klavye

# Heading of the line in that card.
settings-keyboard-no-list-heading = Klavye düzeni listesi kullanılamıyor

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Kurmak için şunu çalıştırın: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Bir tuşu ya da düğmeyi değiştirdiniz.
       *[other] { $n } tuşu ve düğmeyi değiştirdiniz.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Düzen

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Düzen

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Çeşit

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Standart

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = Yok

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Şuna geç

# Words behind the i of the layout card.
settings-keyboard-layout-about = Bu, her tuşun ne yazacağını belirler. Bir düzen ve onun bir çeşidini seçebilirsiniz. İkinci bir düzene geçmek için Alt ve Shift tuşlarına birlikte basın, geri dönmek için yine basın. Değişiklikler hemen uygulanır.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = Şu an kullanımda

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = İkinci bir Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Hiçbir şey

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Caps Lock tuşunu şu olarak kullan

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Halkadaki klavye düğmesi

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Diğer tuşlar

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock bunun yerine ikinci bir Ctrl ya da Escape gibi başka bir tuş olarak çalışabilir. Klavye düğmesi halkanın ortasında durur ve ekranda bir klavye açar. Fareyle ya da dokunmatik ekranla yazmak için kullanabilirsiniz. Halka bir sonraki açılışında görünür.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Bunu bir program değiştiremez.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Değiştirmek için tuşa ya da fare düğmesine basın.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Hiçbir şeye basmadınız, bu yüzden hiçbir şey eklenmedi.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Yerine kullanılacak tuşu yazın, örneğin Escape ya da Ctrl+C; ya da Nothing yazın.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle bunun hangi tuş olduğunu anlayamadı. Bunun yerine tuşu listeden seçin.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Her klavye ve fare

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Bu tuşu aşağıda kendiniz değiştirdiniz ve o değişiklik kullanılır.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = { $key } tuşunu şu aygıtta değiştir

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = { $key } tuşunu şuna değiştir

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Bu değişikliği kaldır

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = { $n }. satır yok sayıldı: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Değiştirilen tuşlar okunamadı

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Trindle değişiklikleri okuyabilene kadar tuşlarınız normal çalışır. { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Değiştirilecek tuşu ya da düğmeyi seçin

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Değiştirmek için tuşa ya da düğmeye basın

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Bir tuşu ya da düğmeyi değiştir

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } tuşunu zaten değiştirdiniz.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Tuşlar ve düğmeler

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Her tuş ya da fare düğmesi başka bir iş yapabilir. Başka bir tuş ya da düğme olabilir, Ctrl+C gibi birlikte basılan tuşlar ya da H, I gibi art arda basılan tuşlar olabilir. Hiçbir şey de yapmayabilir ya da dokunulunca bir şey, basılı tutulunca başka bir şey yapabilir; örneğin dokunulunca Escape, basılı tutulunca Ctrl. Bu, giriş ekranından başlayarak her yerde çalışır. Ctrl, Alt ve bir F tuşu her zaman bir metin konsolu açar; orada trindle-settings --keys default yazmak her tuşu eski haline getirir.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Ctrl ile Caps Lock'u değiştir

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: dokununca Escape, basılı tutunca Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Escape ile Caps Lock'u değiştir

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Alt ile Super'i değiştir, Mac klavyesindeki gibi

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Farenin yan düğmeleri kopyalar ve yapıştırır

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = { $name } aygıtına güven

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Şimdilik yok

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Programların yaptığı klavyeler ve fareler

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = Yeniden eşleme araçları ve oyun başlatıcıları kendi klavyelerini ya da farelerini yapabilir. Tuşları ve tıklamaları yine yazabilir ve oynayabilir, ama Kapat ya da İzin ver gibi korumalı düğmelere basamaz ve size yönelik bir soruyu yanıtlayamaz. Sizin yerinize sayılmasına izin vermek için buradan birini açın.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Oyun kumandası { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $title } için { $name }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = { $title } kumandasının yaptığı iş

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = A ile B'yi değiştir

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = X ile Y'yi değiştir

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Sağ çubuktaki yukarı ve aşağıyı ters çevir

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Eklenti kısayolları

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } kullanılmıyor: { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Tuşlar: { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Kaydedildi

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = Dokunmatik yüzey için de.

# Title of the mouse card.
settings-keyboard-mouse-title = Fare

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = İşaretçi hızı, fareyi oynattığınızda işaretçinin ne kadar hareket ettiğidir. Dokunmatik yüzey için de geçerlidir. Kaydırma hızı, tekerleğin her dönüşünde sayfanın ne kadar gideceğidir. Daha yavaş, Normal ya da Daha hızlı'yı seçin ya da sürgüde istediğiniz yere ayarlayın. Doğal kaydırmada sayfa, telefondaki gibi tekerleğin döndüğü yöne gider. Sol elli seçeneği sol ve sağ düğmeleri değiştirir.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = İşaretçi hızı

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Kaydırma hızı

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Doğal kaydırma

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Sol elli

# Heading and switch name for the clicking helper.
# SAFETY: needs a human review before it ships
settings-keyboard-clicker = Tıklama yardımcısı

# Small words under the clicking helper line.
# SAFETY: needs a human review before it ships
settings-keyboard-clicker-about = Bekleme ya da anahtar tıklayıcısından gelen tıklamalar sizin kendi tıklamalarınız sayılır.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Dokunmatik yüzey

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = Kaydırma hızı, iki parmakla kaydırdığınızda sayfanın ne kadar gideceğidir. Dokunarak tıklama, hafif bir dokunuşu tıklama sayar. Doğal kaydırmada sayfa, telefondaki gibi parmaklarınızın gittiği yöne gider. Yazarken kapalı, yazarken avucunuzun işaretçiyi oynatmasını engeller.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Dokunarak tıkla

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Yazarken kapalı

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Daha yavaş

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Daha hızlı

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB bağlantı noktaları

# Words behind the i of that card.
settings-keyboard-usb-about = Bir USB bağlantı noktası başlamazsa Trindle onu yeniden denemek için bir kez kapatıp açar. Bunu bir bağlantı noktası için birden fazla kez asla yapmaz.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle ayrıca değişikliği geri alamadı: { $undo }


# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Ekran görüntüleri, raporlar ve kısayollar

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Ekran görüntüsü al

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Kısayol listesini göster

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Şimdi ekran görüntüsü al

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Listeyi şimdi göster

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Masaüstünü yeniden başlat

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Halkayı ve yardımcıları yeniden başlat

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Her şeyi yeniden başlat

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = { $label } için tuşlar

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Şimdi yeniden başlat. Açık pencereler olduğu gibi kalır.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Önce sor, sonra her şeyi yeniden başlat

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Yeniden başlatıldı

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } başka bir kısayol tarafından zaten kullanılıyor.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Kaydedilmedi. { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Sorun bildir

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Sorunu şimdi bildir
settings-keyboard-keys-not-saved-unread = Kaydedilmedi. Değiştirilen tuşların dosyası okunamadı, bu yüzden olduğu gibi bırakıldı.
