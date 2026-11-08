# machine translated (Swahili), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Vifurushi vya Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Imezuiliwa kwenye toleo { $have }; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Imezuiliwa; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Imezuiliwa kwenye toleo { $have }

# On a thing held back with no update waiting.
store-upd-held = Imezuiliwa

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Imezuiliwa kwenye toleo { $have }; hakuna jipya zaidi bado

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Imezuiliwa; hakuna jipya zaidi bado

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] sasisho 1
       *[other] masasisho { $n }
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] sasisho 1 liko tayari
       *[other] masasisho { $n } yako tayari
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, { $size } kwa jumla

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 limezuiliwa
       *[other] { $n } yamezuiliwa
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = Programu dhibiti inahitaji kuanzishwa upya ili kukamilika.

# Headline while the check runs.
store-upd-checking = Inakagua masasisho.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Inasasisha { $name }. Unaweza kuendelea kufanya kazi inapoingia.

# Headline while all the updates go in.
store-upd-installing-all = Inasakinisha masasisho. Unaweza kuendelea kufanya kazi yanapoingia.

# Headline when the updates are in and the computer must start again.
# SAFETY: needs a human review before it ships
store-upd-restart = Anzisha upya ili kukamilisha: masasisho yameingia, na yataanza kutumika kompyuta itakapoanza tena.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Kilichoingia kinahitaji kuanzishwa upya ili kukamilika.

# Added to the headline with no internet.
store-upd-no-more = Kompyuta hii haijaunganishwa kwenye intaneti, kwa hiyo hakuna zaidi inayoweza kutafutwa.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Hakuna masasisho yaliyo tayari

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Hakuna masasisho yanayosubiri ambayo kompyuta hii inayajua. Haiwezi kutafuta zaidi isipokuwa imeunganishwa kwenye intaneti.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = programu

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle na Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = programu dhibiti

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Masasisho hayakuweza kukaguliwa kwa { $names }.

# Headline when everything is up to date.
store-upd-current = Kila kitu kimesasishwa.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Ilikaguliwa mara ya mwisho saa { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Haiwezi kukaguliwa bila intaneti

# Red words under a card when a check took too long.
store-upd-too-long = Ilichukua muda mrefu mno kujibu.

# Tip of the round button that checks again.
store-upd-check-again = Kagua tena

# Main button while the check runs.
store-upd-checking-main = Inakagua

# Main button while the updates go in.
store-upd-installing-main = Inasakinisha

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] Sakinisha sasisho 1
       *[other] Sakinisha masasisho { $n }
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = Anzisha upya sasa

# Main button that checks for updates.
store-upd-check-for = Kagua masasisho

# Tip of the round button that releases a thing held back.
store-upd-release = Achilia { $name }

# Tip of the round button that holds a thing's updates back.
store-upd-hold = Zuia { $name }

# A card's state while it is being checked.
store-upd-status-checking = Inakagua

# A card's state while its updates go in.
store-upd-status-installing = Inasakinisha

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Haikuweza kukagua. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } yamezuiliwa

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } yamezuiliwa

# A card's state when nothing waits.
store-upd-up-to-date = Imesasishwa

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = Kutoka { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = Kutoka { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Sehemu inayoshirikiwa, kutoka { $from }

# Note under the Apps card.
store-upd-yours-alone = Ni zako peke yako, kwa hiyo husasishwa bila nenosiri. Programu huleta sehemu zinazoshirikiwa ambazo toleo lake jipya linahitaji.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Sasisho lililozuiliwa hubaki kwenye toleo lililo nalo, na nje ya Sakinisha yote hapa na kwenye pete, hadi ulifungue.

# Heading of the Apps card.
store-upd-card-apps = Programu

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Kila programu jalizi imesasishwa.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] Sasisho la programu jalizi 1 limezuiliwa.
       *[other] Masasisho ya programu jalizi { $n } yamezuiliwa.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] programu jalizi 1 ina sasisho
       *[other] programu jalizi { $n } zina sasisho
    }

# Heading of the Plugins card.
store-upd-card-plugins = Programu jalizi

# Held back card: nothing is held back.
store-upd-nothing-held = Hakuna kilichozuiliwa.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Imeachiliwa

# Note under the Held back card: what releasing does.
store-upd-release-why = Kitu kilichoachiliwa huingia tena pamoja na vingine, kupitia Sakinisha yote hapa na kwenye pete.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = Programu dhibiti ya kifaa

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = programu dhibiti ya kifaa { $id }

# Heading of the Held back card.
store-upd-card-held = Vilivyozuiliwa

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } hadi { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] sasisho 1, ambalo huingia lenyewe
       *[other] masasisho { $n }, ambayo huingia pamoja
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Kutoka Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Inaonyesha { $shown } kati ya masasisho { $total } ya Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Kompyuta hii ina Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle na Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Vifurushi vya Debian vimetengenezwa kupatana, na kimoja kikisakinishwa peke yake kinaweza kuleta vingine au kuviacha visivyoendana. Kwa hiyo huingia pamoja, kutoka kitufe kimoja.

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Masasisho ya Trindle na ya Debian huleta marekebisho ya usalama kwa kompyuta nzima, kwa hiyo hayawezi kuzuiliwa.

# Heading of the card about the daily security updates.
store-upd-card-security = Masasisho ya usalama

# On the security card: its state.
store-upd-security-on = Yamewashwa, kila siku

# On the security card.
store-upd-security-note = Marekebisho ya Debian huingia kila siku bila kuuliza.

# On the security card.
store-upd-security-part = Sehemu ya Trindle; haiwezi kuzimwa.

# On the security card: that holding back never stops these.
store-upd-never-held = Kuzuia sasisho hakuyazuii haya kamwe.

# Firmware card: the computer has no updater.
store-upd-fw-none = Kompyuta hii haina kisasishaji cha programu dhibiti.

# Firmware card: restart to finish.
store-upd-fw-restart = Anzisha upya ili kukamilisha

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Toleo { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Programu dhibiti mpya huingia kompyuta inapoanzishwa upya.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Sakinisha fwupd ili kuweka programu dhibiti ikiwa imesasishwa.

# Heading of the Firmware card.
store-upd-card-firmware = Programu dhibiti

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd haikuweza kupakua orodha mpya zaidi.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub haikuweza kufikiwa.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Orodha za Debian hazikuweza kupakuliwa.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Orodha za Debian hazikuweza kupakuliwa. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Haikuzuiliwa. Orodha ya vilivyozuiliwa haikuweza kutunzwa: { $why }.

# The same, for a release.
store-upd-not-released-list = Haikuachiliwa. Orodha ya vilivyozuiliwa haikuweza kutunzwa: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Imezuiliwa katika Programu, lakini Flatpak yenyewe haikuweza kuambiwa, kwa hiyo kitu kingine kinachosasisha programu bado kinaweza kuiingiza.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Imeachiliwa katika Programu, lakini Flatpak yenyewe haikuweza kuambiwa na inaweza kuiacha nje. Izuie kisha uiachilie tena.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak ilisema: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Ukaguzi wa masasisho wenyewe umezimwa. Bonyeza Kagua masasisho ili kutafuta.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Haijakaguliwa

# Tip of the round button that ends an update that is running.
store-upd-stop = Simamisha

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Inasimamisha

# Under a row whose update was stopped.
store-upd-stopped = Imesimamishwa. Bonyeza Sasisha ili kuendelea.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Imesimamishwa. Bonyeza Sakinisha ili kuendelea.

# Note under the Apps card.
store-upd-steam-restarts = Steam huanza upya mara hakuna mchezo unaoendeshwa.

