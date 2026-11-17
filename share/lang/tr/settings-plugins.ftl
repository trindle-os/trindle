
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Eklentileriniz

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = eklentiler ek bileşenler uzantılar kurulu kaldır ayarlar

# Help line at the foot of the Your plugins page.
# SAFETY: needs a human review before it ships
settings-plugins-help = Eklentiler Trindle'a bir şeyler ekler. Bir eklenti halkaya bir simge, buraya ayarlar, bildirimler ya da oturum açtığınızda başlayan bir şey ekleyebilir. Her eklenti kendi kutusunda çalışır ve İzinler'de izin vermediğiniz sürece hiçbir şeyi kullanamaz. Sayfa başlığının yanındaki düğme İzinler'i açar. Bir eklentiyi kaldırmak için kartındaki çöp kutusuna iki kez basın.

# Red words when the window that asks what a plugin may use could not be opened.
# SAFETY: needs a human review before it ships
settings-plugins-window-failed = Trindle eklentinin neyi kullanabileceğini soran pencereyi açamadı.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Sürüm { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Sürüm { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
# SAFETY: needs a human review before it ships
settings-plugins-asks-nothing = { $name } hiçbir şey istemiyor, bu yüzden hiçbir şeye izin verilmeden kendi kutusunda çalışır.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } şunları istiyor: { $list }. { $asks } şeyden { $allowed } tanesine izin verdiniz. Kendi kutusunda çalışır, yalnızca izin verdiğiniz şeylerle.
       *[other] { $name } şunları istiyor: { $list }. { $asks } şeyden { $allowed } tanesine izin verdiniz. Kendi kutusunda çalışır, yalnızca izin verdiğiniz şeylerle.
    }

# Heading of the line about a plugin on its card.
settings-plugins-about-it = Hakkında

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = Ne kullanabileceğini değiştir

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = Bu eklentiyi kaldır

# Red words when a plugin could not be removed.
# SAFETY: needs a human review before it ships
settings-plugins-not-removed = Kaldırılamadı.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Buraya yazın

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } için kendi ayarları yok.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } için şu an gösterilecek ayar yok.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] Kurulu eklenti yok.
        [one] Bir eklenti kurulu: { $list }.
       *[other] { $n } eklenti kurulu: { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Eklentiler

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Hiçbiri kurulu değil

# Tip of the button at the end of the page's title that opens the Permissions page.
# SAFETY: needs a human review before it ships
settings-plugins-open-permissions = İzinleri aç

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = { $name }'e soruluyor…

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } yanıt vermiyor. Hizmeti durmuş olabilir.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Kaydedildi.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } bunu kabul etmedi.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } bunu yapamadı.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } bunu yapamadı: { $why }.
