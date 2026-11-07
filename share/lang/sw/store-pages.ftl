# machine translated (Swahili), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] programu { $count }
       *[other] programu { $count }
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] sasisho { $count } linasubiri
       *[other] masasisho { $count } yanasubiri
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } za kuchagua kwenye Flathub; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = kila kitu chako kimesasishwa

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Hakuna programu zinazolingana na “{ $text }”.

# Line under the title of an empty page of apps.
store-grid-none-here = Hakuna programu hapa bado.

# Advice in the empty page of a search.
store-grid-try-shorter = Jaribu neno fupi zaidi, au tazama aina upande wa kushoto.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Tafuta kwa jina hapo juu, au tazama aina nyingine upande wa kushoto.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = Inaonyesha { $shown } kati ya { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = Onyesha { $next } zaidi

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Inapakua orodha ya programu kutoka Flathub. Mara ya kwanza huchukua dakika moja.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Programu hazikuweza kufikia Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Kompyuta hii haijaunganishwa kwenye intaneti. Programu hujaribu tena zenyewe mara itakapounganishwa.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Programu za kuchagua hutoka Flathub, kupitia intaneti.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } na mipangilio ya programu, upande wa kushoto, hufanya kazi bila hiyo.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } na mipangilio ya programu, upande wa kushoto, hufanya kazi bila Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } na mipangilio ya programu, upande wa kushoto, hufanya kazi bila intaneti.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Jaribu tena

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Matokeo

# Red words under the search box when no web browser opens a link.
store-no-browser = Hakuna kivinjari cha kufungua { $url }. Pata kimoja kwenye Programu kwanza.

# Said in the card or under the title while the lists are being read.
store-looking = Inatafuta…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Sasa iko kwenye Zilizosakinishwa, na kwenye pete

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Imeondolewa

# The word on an app's dial when installing or removing did not work.
store-done-not = Haikufanyika

# The word on an app's dial when it was installed.
store-done-installed = Imesakinishwa

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak ilikataa.

# Tab of the Installed page that shows everything.
store-tab-all = Zote

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Faili za Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Faili za Deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = Programu jalizi

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Hakuna kilichosakinishwa bado. Gundua ina vitu vingi vya kujaribu.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] programu jalizi 1 imesakinishwa
       *[other] programu jalizi { $n } zimesakinishwa
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] programu 1 imesakinishwa
       *[other] programu { $n } zimesakinishwa
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = vitu { $n } vimesakinishwa

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, kutoka Flathub
       *[other] { $start }, zote kutoka Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, kutoka faili ya Flatpak
       *[other] { $start }, zote kutoka faili za Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, AppImage
       *[other] { $start }, zote ni AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, kutoka faili ya .deb
       *[other] { $start }, zote kutoka faili za .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } kutoka Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 kutoka faili ya Flatpak
       *[other] { $n } kutoka faili za Flatpak
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] AppImage 1
       *[other] AppImage { $n }
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] programu 1
       *[other] programu { $n }
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 kutoka faili ya .deb
       *[other] { $n } kutoka faili za .deb
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] programu jalizi 1
       *[other] programu jalizi { $n }
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Imesasishwa.

# End of the Installed line when everything is up to date.
store-installed-state-all = Zote zimesasishwa.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 ina sasisho linalosubiri.
       *[other] { $n } zina masasisho yanayosubiri.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = Programu { $shown } kati ya { $total } zinalingana na “{ $find }”.

# The same when plugins are among the things.
store-installed-match-things = Vitu { $shown } kati ya { $total } vinalingana na “{ $find }”.

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Ongeza AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = Tafuta kilichosakinishwa

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage haikuweza kuongezwa.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Programu unazosakinisha kutoka Gundua, kutoka GitHub au kama AppImage, programu za faili za .deb na programu jalizi huonekana hapa.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Hakuna kilichosakinishwa kinacholingana na “{ $find }”.

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = Ondoa huuliza mara mbili: ibonyeze tena ndani ya sekunde tatu.

# On an installed thing's line: its version.
store-installed-version = toleo { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = imezuiliwa

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = imezuiliwa; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = Inasasisha { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Sasisha { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = Kuhusu

# Under an installed thing's row while it is being taken off.
store-installed-removing = Inaondoa...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Hakuna basi ya kipindi ya kuuliza faili ipi.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Chagua AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Kichaguzi cha faili hakikuweza kuulizwa.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Inaongeza...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } tayari iko hapo: tazama Zilizosakinishwa.

# Words once an AppImage is in the ring.
store-add-in-ring = Imeongezwa. Sasa iko kwenye orodha yako.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Hiyo si rejeleo la Flatpak ambalo kompyuta hii inaweza kutumia.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = Inasakinisha { $name } kutoka { $host }...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } imesakinishwa.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak haikukubali kuisakinisha.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } haikuweza kusakinishwa. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Inabadilisha...

# Heading of the question when an AppImage has the name of one already there.
# SAFETY: needs a human review before it ships
store-replace-head = { $name } tayari iko hapo

# Small heading over the AppImage being brought.
store-replace-new = Inayoletwa

# Small heading over the AppImage already there.
store-replace-old = Iliyopo tayari

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = Ruka: weka iliyopo tayari

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = Badilisha

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = Kuondoa { $name }?

# Under that heading: how much saved data the app has.
# SAFETY: needs a human review before it ships
store-flatpak-ask-size = Data yake iliyohifadhiwa inachukua { $size }.

# Under that heading when the app has saved no data.
# SAFETY: needs a human review before it ships
store-flatpak-ask-none = Haina data iliyohifadhiwa.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Inahesabu data yake iliyohifadhiwa...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Data iliyofutwa haiwezi kurudishwa.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Ondoa, weka data yake

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Ondoa na ufute data yake

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Futa data yake tu

# Why Delete its data only is greyed.
# SAFETY: needs a human review before it ships
store-flatpak-data-only-none = Haina data iliyohifadhiwa ya kufuta.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Ghairi: weka vyote viwili

# Said under the card or the row once only the data has been deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-gone = Data yake iliyohifadhiwa imefutwa.

# Words while only the data is being deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-going = Inafuta data yake iliyohifadhiwa...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Haikukamilika, na haikutoa sababu.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Onyesha tatizo lilipo

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Tafuta GitHub: andika, kisha bonyeza Enter

# Heading of the card of matches from AppImageHub.
store-gh-hub = Kwenye AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Bonyeza Enter ili kutafuta pia kwenye GitHub nzima.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub haikuulizwa: hakuna intaneti ya kuiuliza.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Inauliza GitHub...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = Hakuna zaidi kwenye GitHub.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] Inatafuta faili za AppImage na Flatpak katika mradi { $n }...
       *[other] Inatafuta faili za AppImage na Flatpak katika miradi { $n }...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Hakuna kati ya matokeo kwenye GitHub lenye faili ya AppImage au Flatpak kwa kompyuta hii.

# Heading of the card of matches from GitHub.
store-gh-on = Kwenye GitHub

# Heading of the card that holds the search box.
store-gh-search-card = Tafuta GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = Programu kwenye GitHub kama faili za AppImage au Flatpak. Hakuna anayezikagua: sakinisha kutoka kwa watu unaowaamini tu.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = GitHub haiwezi kutafutwa wakati kompyuta hii haijaunganishwa kwenye intaneti.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak na AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Tazama faili

# Button that installs a GitHub project or file.
store-gh-install = Sakinisha

# Link to a project's page on GitHub.
store-gh-open = Fungua kwenye GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, toleo { $tag }

# Name of the GitHub key's button and of its box.
# SAFETY: needs a human review before it ships
store-gh-key = Ufunguo wa GitHub

# Name of the GitHub key's button when a key is stored.
# SAFETY: needs a human review before it ships
store-gh-key-stored = Ufunguo wa GitHub: mmoja umehifadhiwa

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = Karibu hakika huuhitaji. Bila ufunguo, GitHub hujibu maswali 60 kwa saa kutoka kompyuta hii; ukiwa nao hujibu 5,000. Ufunguo huongeza kikomo hicho tu. Hutumika kutazama faili za umma na si kingine: haupi Trindle njia yoyote ya kuingia kwenye akaunti yako, na huweki alama kwenye ruhusa zozote unapoutengeneza. Unatunzwa kwenye kompyuta hii, kwenye kitunza funguo chako, na kutumwa kwa GitHub peke yake.

# In the GitHub key's box: how to make a key.
# SAFETY: needs a human review before it ships
store-gh-key-how = Tengeneza ufunguo kwenye GitHub, uupe jina lolote, acha kila ruhusa bila kuigusa, bonyeza Generate token, kisha bandika hapa kinachoonyeshwa.

# Red words under the GitHub key's box when the key could not be removed.
# SAFETY: needs a human review before it ships
store-gh-key-not-removed = Haukuweza kuondolewa: { $why }

# Red words under the GitHub key's box when the key could not be kept.
# SAFETY: needs a human review before it ships
store-gh-key-not-kept = Haukuweza kuhifadhiwa: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
# SAFETY: needs a human review before it ships
store-gh-key-keep-empty = Acha tupu ili kuuweka

# Placeholder of the GitHub key's box.
# SAFETY: needs a human review before it ships
store-gh-key-paste = Bandika ufunguo

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = Ondoa ufunguo

# Link in the GitHub key's box to the page where a key is made.
# SAFETY: needs a human review before it ships
store-gh-key-make = Tengeneza ufunguo kwenye GitHub

# Tip of the round button that keeps the GitHub key typed.
# SAFETY: needs a human review before it ships
store-gh-key-keep = Hifadhi ufunguo

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Hakuna kitu kwenye toleo lake jipya zaidi kinachosakinishwa hapa.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Imeongezwa

# Line under the title of the Move your apps page.
store-bk-line = Programu na mipangilio yako, katika faili moja ya kwenda nayo popote.

# Heading of the card that makes a backup.
store-bk-card = Hifadhi nakala

# Tip of the round button while the backup is being made.
store-bk-making-tip = Inatengeneza nakala rudufu

# Main button that makes the backup.
store-bk-make = Tengeneza nakala rudufu

# Tip of the round button that shows the backup in Folders.
store-bk-show = Ionyeshe kwenye Folda

# Name of the switch for the list of apps in a backup.
store-bk-apps = Programu

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] Orodha ya programu yako 1 ya Flatpak.
       *[other] Orodha ya programu zako { $n } za Flatpak.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } Kompyuta nyingine huzisakinisha upya kutoka Flathub, kwa hiyo faili inabaki ndogo.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Data ya programu zenyewe

# Under the Apps' own data switch.
store-bk-data-sub = Mipangilio ya kila programu na kazi iliyohifadhiwa, bila akiba zake.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = Zinawekwa kamili, kwa kuwa ni faili moja moja.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Mipangilio ya Trindle

# Under the Trindle settings switch.
store-bk-settings-sub = Pete, chaguo la mandhari, rangi, kibodi, nguvu, programu chaguomsingi na picha yako.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Sasa { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Inakokwenda

# Under that row: the folder and the file's name.
store-bk-where-sub = Folda yako ya nyumbani, kama { $name }

# Heading of the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-card = Rudisha nakala rudufu

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Chagua faili ya nakala rudufu

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Nakala rudufu ya Trindle, au iliyotengenezwa na { $flat }.

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = Programu zake husakinishwa na data na mipangilio yake kurudishwa. Hati na picha zako haziko ndani yake: zinakili wewe mwenyewe.

# Said while a backup is being made.
store-bk-making = Inahifadhi nakala... hili linaweza kuchukua dakika chache.

# Said when a backup is made: the file and its size.
store-bk-made = Ni { $path } ({ $size }). Inakili kwenye kijiti cha USB au hifadhi ya wingu ili kwenda nayo.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Hakuna nakala rudufu iliyochaguliwa, kwa sababu Trindle haina muunganisho na kichaguzi cha faili.

# Title of the window that asks which backup to use.
store-bk-choose-title = Chagua nakala rudufu

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Nakala rudufu za Trindle na Flat

# Red words when the file chooser gave no answer.
store-bk-no-answer = Hakuna nakala rudufu iliyochaguliwa, kwa sababu kichaguzi cha faili hakikujibu.

# Said while a backup file is read.
store-bk-reading = Inasoma nakala rudufu...

# Said when a backup was put back.
store-bk-logout = Toka kisha uingie tena ili kuona kila mpangilio.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Toka kisha uingie tena ili kuona kila mpangilio.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] Programu 1, { $with } zina mipangilio yake. Inaanza...
       *[other] Programu { $apps }, { $with } zina mipangilio yake. Inaanza...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Hakuna vyanzo vya watengenezaji vilivyoongezwa bado.
        [one] Chanzo 1 cha mtengenezaji kimeongezwa, kinasasishwa pamoja na masasisho ya Debian.
       *[other] Vyanzo { $n } vya watengenezaji vimeongezwa, vinasasishwa pamoja na masasisho ya Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Imeongezwa, na inasasishwa.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. Chanzo kimeongezwa, lakini programu haijasakinishwa.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Imesakinishwa kwa njia nyingine.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = Iondoe pamoja na chanzo chake

# Link to a maker's page, naming the program.
store-src-open = Fungua { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Ingia kwenye Tailscale

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = Ongeza { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = Tayari kuongezwa

# Name of the field for the address of a source.
store-src-address = Anwani

# Under the address field.
store-src-address-sub = Anwani ya hazina, kutoka kwenye maelekezo ya usakinishaji ya mtengenezaji wake.

# Name of the field for the suite of a source.
store-src-suite = Suite

# Under the suite field.
store-src-suite-sub = Kwa kawaida trixie au stable.

# Name of the field for the components of a source.
store-src-components = Components

# Under the components field.
store-src-components-sub = Kwa kawaida main.

# Name of the field for a source's signing key.
# SAFETY: needs a human review before it ships
store-src-key = Ufunguo wa kusaini

# Under the signing key field.
# SAFETY: needs a human review before it ships
store-src-key-sub = Anwani ya ufunguo wake wa kusaini.

# Name of the field for the program to install from a source.
store-src-program = Programu ya kusakinisha

# Under the program field.
store-src-program-sub = Jina la kifurushi chake, ikiwa unataka isakinishwe sasa.

# Placeholder of the program field.
store-src-program-hint = hiari

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = Ongeza chanzo hiki

# Heading of the card that adds another source.
# SAFETY: needs a human review before it ships
store-src-another = Ongeza chanzo kingine

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = Chanzo kinaweza kusakinisha masasisho ya kompyuta nzima, kwa hiyo ongeza watengenezaji unaowaamini tu. Kila kimoja huomba nenosiri la msimamizi.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = Ondoa chanzo hiki

# Heading of the card of sources already on the computer.
store-src-already = Tayari kwenye kompyuta hii

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Anwani zote mbili lazima zianze na https://

# Red words when the suite or components are not plain words.
store-src-bad-words = Suite na components lazima ziwe maneno ya kawaida.

# Red words when the program is not a package name.
store-src-bad-package = Hilo si jina la kifurushi.

# Word on a source's dial while its job starts.
store-src-starting = Inaanza

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Inafungua ukurasa wa kuingia wa Tailscale...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Ingia kwenye ukurasa uliofunguliwa; hili hukamilika lenyewe.

# Said under Tailscale's row when it is connected.
store-src-connected = Imeunganishwa

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = Kiungo cha programu hakijasimbwa kwa njia fiche.

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = Haijasimbwa kwa njia fiche

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = Anwani hii haijasimbwa kwa njia fiche. Mtu aliye njiani anaweza kuona kinachotumwa, na anaweza kubadilisha nakala rudufu na nyingine.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = Naelewa na nakubali hatari hii

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = Rudisha

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = Ghairi

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = Data yake iliyohifadhiwa haikuweza kuhesabiwa.

store-hub-warning = AppImageHub huorodhesha miradi tu: hakuna anayeikagua wala kuichanganua, na haihitaji kusainiwa. Trindle huendesha kila AppImage ndani ya boksi lenye folda ya nyumbani ya faragha: mwanzoni inaweza kutumia intaneti na sauti, na haioni faili zako zozote.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = AppImage inatoka wapi ni dokezo, si ukaguzi.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = Kwenye GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = Kwenye GitHub: { $repo }, nyota { $stars }

# Where an AppImage comes from: a website, by its name.
store-from-site = Si kwenye GitHub: kutoka tovuti, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Chanzo hakijulikani

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Fungua tovuti yake

# When an app was last updated.
store-age-updated = Imesasishwa mwisho: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Imesasishwa mwisho: { $date }. Haijasasishwa kwa zaidi ya miezi sita, kwa hiyo inaweza kuwa na kasoro za usalama ambazo hazijarekebishwa.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Imesasishwa mwisho: haijulikani

# When an app's project was made.
store-age-created = Ilitengenezwa { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Ilitengenezwa: haijulikani

# A date the source does not give, on a line of Facts.
store-age-not-known = Haijulikani

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Kabla ya kuongeza moja

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] AppImage { $count } imeorodheshwa kwenye AppImageHub.
       *[other] AppImage { $count } zimeorodheshwa kwenye AppImageHub.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Inaleta orodha ya AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = Orodha ya AppImageHub haikuweza kuletwa.

# The same with no internet and no copy kept.
store-browse-offline = Orodha ya AppImageHub haiwezi kuletwa wakati kompyuta hii haijaunganishwa kwenye intaneti.

# In the list when the kind chosen has none.
store-browse-none = AppImageHub haiorodheshi yoyote ya aina hii.

# Under the rows: how many are shown of how many.
store-browse-showing = Inaonyesha { $shown } kati ya { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Ongeza kwenye programu za Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Angalia maelezo na faili



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = ukaguzi wa masasisho umezimwa

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT itarudi OpenAI itakapotia saini upakuaji wake.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } za kuchagua kwenye Flathub na AppImageHub; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } za kuchagua kwenye AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Zote mbili

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage hazikaguliwi. Hakuna anayezihakiki wala kuzichanganua.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle huendesha kila AppImage ndani ya kisanduku chenye folda ya nyumbani ya faragha: mwanzoni inaweza kutumia intaneti na sauti, na haioni faili zako zozote.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Usionyeshe tena

# Button on that card that goes on with the install.
store-appimage-warn-ok = Sawa

# Button on that card that gives the install up.
store-appimage-warn-cancel = Ghairi

