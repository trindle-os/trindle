# machine translated (Turkish), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Oturum açılırken başlat

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay otomatik başlat açılış başlangıç oturum gecikme

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Oturum açtığınızda kendiliğinden açılan uygulamalar. Gecikme, bir uygulamanın önce ağı ya da başka bir uygulamayı beklemesini sağlar.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = Oturum açtığınızda kendiliğinden başlayan bir şey yok.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = Oturum açtığınızda { $name } kendiliğinden başlar.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] Oturum açtığınızda { $n } uygulama kendiliğinden başlar.
       *[other] Oturum açtığınızda { $n } uygulama kendiliğinden başlar.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] Kapalı olarak saklanan bir tane daha var.
       *[other] Kapalı olarak saklanan { $n } tane daha var.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = Hemen

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Bir dakika sonra

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = 2 dakika sonra

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] { $n } saniye sonra
       *[other] { $n } saniye sonra
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = { $name } için gecikme

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Oturum açılırken başlat

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = Oturum açılırken başlamasını durdur

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Kaldırılamadı, bu yüzden oturum açılırken hâlâ başlıyor.

# Red words under the add card when something could not be added.
settings-startup-not-added = Eklenemedi.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = Uygulama bul

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Bunu oturum açılırken başlat

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = Eklenemedi: uygulama artık kurulu değil.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = örn. syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Diskten bir program seç

# Heading of the line for adding a command.
settings-startup-command = Komut

# Small words under the command line.
settings-startup-command-about = Terminal'de yazıldığı gibi ya da diskteki bir program.

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Bir komut yazın ya da bir program seçin.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Oturum açılırken başlayacak programı seçin

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Kendiliğinden başlayan bir şey yok

# Small words under that line.
settings-startup-none-about = Yanındaki düğmeyle bir uygulama ekleyin.

# Title of the card listing what starts at login.
settings-startup-list-title = Oturum açılırken başlayanlar

# Words behind the i of the card listing what starts at login.
settings-startup-about = Her uygulama, ağın ya da başka bir uygulamanın hazır olması için oturum açtıktan sonra bir süre bekleyebilir. Anahtarı onu başlatmadan burada tutar; çöp kutusu ise oturum açılırken başlamasını tamamen durdurur.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } şunu çalıştırır: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Kurulu uygulamalardan seç

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Bir komut yazın ya da bir program seçin

# Heading of the line for adding an installed app.
settings-startup-app-heading = Kurulu bir uygulama

# Small words under that line.
settings-startup-app-about = Uygulama listesinden birini seçin.

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Bir komut ya da program

# Small words under that line.
settings-startup-cmd-about = Yazılır ya da diskten seçilir.

# Words behind the i of the Add an app card.
settings-startup-add-about = Kurulu uygulamalardan seçin ya da çalıştırılacak bir komut verin; komutu Terminal'de yazacağınız gibi yazın ya da diskten bir program seçin. Bir sonraki oturum açılışında hemen başlar; listeye girdikten sonra bunun için bekleme süresi seçilebilir.

# Title of the card for adding an app.
settings-startup-add-title = Uygulama ekle
