
# machine translated (Turkish), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Klasörün en üstünde okunabilen bir { $file } yok ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } içinde [Plugin] bölümü yok.

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n }, bu Trindle'ın bildiği biri değil: Interface={ $known } biliyor.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] için Interface={ $known } gerekir.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] için harf, rakam, nokta ve tireden oluşan bir Id gerekir, örneğin org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] için bir Name gerekir.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] için bir Version gerekir.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] Summary tek satırdır.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } tek satırda düz sözcüklerdir, en çok { $max } karakter.
       *[other] [{ $group }] { $key } tek satırda düz sözcüklerdir, en çok { $max } karakter.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = { $name } diye bir izin yok; izinler şunlardır: { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] için bir Icon gerekir: eklentinin klasöründe bir SVG veya PNG dosyası.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon, eklentinin klasöründe olmayan { $icon } adını veriyor.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] için bir Label gerekir; imleç üzerinde durunca söylenir.

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] için Run gerekir; tıklayınca başlayan şey.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] için Fields gerekir; alanlarının adları.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = { $name } alan adı yalnızca harf, rakam, tire ve alt çizgi içerebilir.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = { $name } alanının [{ $group }] bölümü yok.

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] için bir Label gerekir.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices en çok { $most } tanedir, her biri en çok { $chars } karakterlik düz sözcüklerdir.
       *[other] [{ $group }] Choices en çok { $most } tanedir, her biri en çok { $chars } karakterlik düz sözcüklerdir.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom, eklentinin evinde bir dosya adıdır: harf, rakam, nokta, tire ve alt çizgi, nokta ile başlamaz, klasör yok.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] bir seçimdir, bu yüzden iki veya daha çok Choices ya da bir dosya olarak ChoicesFrom gerekir.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default, Choices içindekilerden biri değil.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] bir nottur, bu yüzden TextFrom gerekir; eklentinin evinde bir dosya adı: harf, rakam, nokta, tire ve alt çizgi, nokta ile başlamaz, klasör yok.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] bir nottur, ayar içermez, bu yüzden Default olmaz.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind { $kind }: switch, text, choice veya note olmalıdır.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] bir anahtardır, bu yüzden Default değeri true veya false olur.

# Red words when the [Background] part is given without the background permission.
# SAFETY: needs a human review before it ships
plugins-background-needs-permission = [Background] için [Plugin] Permissions içinde background izni gerekir.

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = Eklenti hiçbir şeye bağlanmıyor: ona [Ring], [Settings], [Background], [Shortcut] veya bildirim izni verin.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path }, eklentinin klasörü dışına bağlanıyor.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run bir komut olarak okunamıyor.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run, eklentinin klasörü içindeki bir programı adlandırmalı, { $program } olmaz.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run, eklentinin taşımadığı bir programı ({ $program }) adlandırıyor (eksik veya çalıştırılabilir değil).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu içinde { $n } öğe var: en çok { $max } öğeye izin verilir.
       *[other] [Ring] Menu içinde { $n } öğe var: en çok { $max } öğeye izin verilir.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = { $id } menü kimliği yalnızca harf, rakam, tire ve alt çizgi içerebilir.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu, { $id } adını iki kez veriyor.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu, [{ $group }] bölümü olmayan { $id } adını veriyor.

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label en çok { $max } karakterlik tek kısa satırdır.
       *[other] [{ $group }] Label en çok { $max } karakterlik tek kısa satırdır.
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] için Run gerekir; seçilince başlayan şey.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] yalnızca Trindle kataloğundaki, control izni isteyen Flow içindir.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] için Run gerekir; PIN'in verildiği komut.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen en çok { $max } karakterdir.
       *[other] [{ $group }] ShowWhen en çok { $max } karakterdir.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen alan=değer biçiminde yazılır, örneğin ShowWhen=mode=home: başka bir alan ve onun taşıması gereken tek değer.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen alan=değer biçiminde yazılır ve alanın adı yalnızca harf, rakam, tire ve alt çizgi içerebilir.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen için tek satırda, en çok { $max } karakterlik düz sözcüklerden oluşan bir değer gerekir.
       *[other] [{ $group }] ShowWhen için tek satırda, en çok { $max } karakterlik düz sözcüklerden oluşan bir değer gerekir.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen kendi alanını adlandırıyor: başka, daha önceki bir alanı adlandırmalı.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen, bu eklentinin Fields listesinde olmayan { $other } alanını adlandırıyor.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen, Fields içinde kendinden sonra gelen { $other } alanını adlandırıyor: daha önceki bir alan olmalı.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen, kendi ShowWhen değeri olan { $other } alanını adlandırıyor: bir alan yalnızca ShowWhen değeri olmayan bir alana bağlı olabilir.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen, { $other } anahtarını adlandırıyor, bu yüzden değeri true veya false olur.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen değeri { $value }, { $other } alanının Choices değerlerinden biri değil.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen, ayar içermeyen { $other } notunu adlandırıyor: bir alan yalnızca bir anahtara, seçime veya metne bağlı olabilir.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Ağ] Dinleme: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = [Network] Listen için [Plugin] Permissions içinde network izni gerekir.

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = [Network] Listen için [Plugin] Permissions içinde listen izni gerekir.

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = listen izni için [Network] Listen gerekir; izin verilecek bağlantı noktaları.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = plugin.ini dosyası, bu Trindle'ın bildiği bir arayüz için değil.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Dinlenecek hiçbir bağlantı noktası belirtmiyor.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = plugin.ini dosyası okunamadı.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = Dinlemesine izin verilmedi.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = Belirttiği bağlantı noktaları izin verilenler değil. Yeniden izin verilmeleri gerekir.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none = { $name } çalışmak için izninize gerek duyuyor: { $n } izinden 0 tanesi verildi.

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } istediği iznin yalnızca bir kısmına sahip: { $n } izinden { $a } tanesi verildi.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] Hiçbir şey istemiyor.
        [one] İstediği tek şeye izin verildi.
       *[other] İstediği { $n } şeyin hepsine izin verildi.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = İzin ver

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
# SAFETY: needs a human review before it ships
plugins-other-ports = önceki bağlantı noktalarından farklı noktalardan erişilebilir olmak

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Kurulu değil.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = başka bir eklenti

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Bu eklenti kurulu değil.

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = Kaydedilemedi: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Eklentinin dosyaları kopyalanamadı.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Arka plan bölümü çalışmıyor: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = trindle-plugin başlatmak için bulunamadı.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] { $n } saniye içinde başlamadı.
       *[other] { $n } saniye içinde başlamadı.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = katılacağı çalışan bir masaüstü oturumu yok, bu yüzden başlatılmadı. Önce oturum açın.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = hangi masaüstü oturumuna katılacağını belirleyemiyor ({ $sessions }), bu yüzden başlatılmadı. WAYLAND_DISPLAY doğru olana ayarlanmış olarak yeniden çalıştırın.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = dosyanın olması gereken yerde bir klasör var

# Error text when a plugin's id is not an id.
plugins-not-an-id = bir eklenti kimliği değil

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts içinde { $n } kısayol var: en çok { $max } kısayola izin verilir.
       *[other] [Plugin] Shortcuts içinde { $n } kısayol var: en çok { $max } kısayola izin verilir.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = { $id } kısayol kimliği yalnızca harf, rakam, tire ve alt çizgi içerebilir.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts, { $id } adını iki kez veriyor.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts, [{ $group }] bölümü olmayan { $id } adını veriyor.

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys }, zaten Trindle'ın bir kısayolu; başka tuşlar seçin.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys }, eklentinin başka bir kısayolu tarafından kullanılıyor.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] için Keys gerekir, örneğin Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] için Run gerekir; tuşların başlattığı şey.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Bunun için yazılan tuşlar bir kısayol değil. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } zaten Trindle'ın bir kısayolu.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } zaten kendi kısayollarınızdan biri.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } zaten { $holder } tarafından kullanılıyor.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Bu, kurulu bir eklentinin kısayolu değil.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Kullanıma alınamadı ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Kaydedilemedi: hiçbir şey değişmedi, çünkü { $why }.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Kaydedilemedi ({ $why }).



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Erişilebilirlik izni yalnızca katalogdaki Trindle ekran okuyucusu içindir.

