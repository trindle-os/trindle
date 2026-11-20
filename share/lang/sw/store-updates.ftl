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
store-upd-held-version-nothing = Imezuiliwa kwenye toleo { $have }. Hakuna jipya zaidi bado.

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
store-upd-firmware-restart = Anzisha upya ili kukamilisha sasisho la programu dhibiti.

# Headline while the check runs.
store-upd-checking = Inakagua masasisho.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Inasasisha { $name }. Unaweza kuendelea kufanya kazi inaposakinishwa.

# Headline while all the updates go in.
store-upd-installing-all = Inasakinisha masasisho. Unaweza kuendelea kufanya kazi yanaposakinishwa.

# Headline when the updates are in and the computer must start again.
# SAFETY: needs a human review before it ships
store-upd-restart = Anzisha kompyuta upya ili kumaliza kusakinisha masasisho.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Anzisha upya ili kukamilisha usakinishaji.

# Added to the headline with no internet.
store-upd-no-more = Kompyuta hii haijaunganishwa kwenye intaneti, kwa hiyo Trindle haiwezi kutafuta masasisho zaidi.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Hakuna masasisho yaliyo tayari

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Trindle haijui masasisho yoyote yanayosubiri. Haiwezi kutafuta zaidi wakati kompyuta hii haijaunganishwa kwenye intaneti.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = programu

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle na Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = programu dhibiti

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Trindle haikuweza kukagua masasisho ya { $names }.

# Headline when everything is up to date.
store-upd-current = Kila kitu kimesasishwa.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Ilikaguliwa mara ya mwisho saa { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Haiwezi kukaguliwa bila intaneti

# Red words under a card when a check took too long.
store-upd-too-long = Ukaguzi ulichukua muda mrefu mno.

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
# Beside Restart now: the box that puts the restart off, then the ring's Updates button reminds. Its answers are the choices below.
store-upd-remind-later = Nikumbushe baadaye
# Red words under the headline when the Remind me later time could not be saved; the reason follows.
store-upd-remind-not-saved = Imeshindwa kuhifadhi kikumbusho: { $why }.
# One answer in the Remind me later box.
store-upd-remind-hours =
    { $n ->
        [one] Baada ya saa 1
       *[other] Baada ya saa { $n }
    }

# Main button that checks for updates.
store-upd-check-for = Kagua masasisho

# Tip of the round button that releases a thing held back.
store-upd-release = Ruhusu masasisho ya { $name }

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
store-upd-yours-alone = Programu hizi ni zako peke yako, kwa hiyo husasishwa bila nenosiri. Sasisho la programu pia husakinisha sehemu zinazoshirikiwa inazohitaji.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Sasisho lililozuiliwa hubaki kwenye toleo la sasa. Huachwa nje unaposakinisha masasisho yote, hapa na kwenye pete, hadi uruhusu masasisho tena.

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
store-upd-released = Haijazuiliwa

# Note under the Held back card: what releasing does.
store-upd-release-why = Ukiruhusu masasisho tena, litasakinishwa pamoja na mengine, hapa na kwenye pete.

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
        [one] sasisho 1, linalosakinishwa lenyewe
       *[other] masasisho { $n }, yanayosakinishwa pamoja
    }

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Inaonyesha { $shown } kati ya masasisho { $total } ya Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Kompyuta hii ina Trindle { $version }.

# Heading of the card about the daily security updates.
store-upd-card-security = Masasisho ya usalama

# On the security card: its state.
store-upd-security-on = Yamewashwa, kila siku

# On the security card: that holding back never stops these.
store-upd-never-held = Kuzuia sasisho hakuzuii kamwe masasisho ya usalama.

# Firmware card: the computer has no updater.
store-upd-fw-none = Kompyuta hii haina kisasishaji cha programu dhibiti.

# Firmware card: restart to finish.
store-upd-fw-restart = Anzisha upya ili kukamilisha

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Toleo { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Programu dhibiti mpya husakinishwa kompyuta inapoanzishwa upya.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Sakinisha fwupd ili kuweka programu dhibiti ikiwa imesasishwa.

# Heading of the Firmware card.
store-upd-card-firmware = Programu dhibiti

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd haikuweza kupakua orodha mpya zaidi.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub haikuweza kufikiwa.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Trindle haikuweza kupata orodha za Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Trindle haikuweza kupata orodha za Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Trindle haikuweza kuzuia, kwa sababu haikuweza kuhifadhi orodha ya masasisho yaliyozuiliwa: { $why }.

# The same, for a release.
store-upd-not-released-list = Trindle haikuweza kuruhusu masasisho tena, kwa sababu haikuweza kuhifadhi orodha ya masasisho yaliyozuiliwa: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Imezuiliwa katika Programu, lakini Trindle haikuweza kubadilisha hili katika Flatpak. Programu nyingine inayosasisha programu bado inaweza kuisakinisha.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Masasisho yameruhusiwa tena katika Programu, lakini Trindle haikuweza kubadilisha hili katika Flatpak, kwa hiyo Flatpak inaweza bado kuiacha nje. Izuie, kisha uruhusu masasisho tena.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak ilisema: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle

# Said when a security fix for a boxed app can't be downloaded. { $package } is the package, { $version } the fixed version.
store-patch-not-fetched = Trindle haikuweza kupakua { $package } { $version }.

# Said when a security fix for a boxed app was downloaded but isn't there to use.
store-patch-not-found = Trindle haikuweza kupata kifurushi ilichopakua.

# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Ukaguzi wa masasisho wa kiotomatiki umezimwa.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Haijakaguliwa

# Tip of the round button that ends an update that is running.
store-upd-stop = Simamisha

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Inasimamisha

# Under a row whose update was stopped.
store-upd-stopped = Imesimamishwa. Bonyeza Sasisha ili kuendelea.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Imesimamishwa. Sakinisha tena ili kuendelea.

# Note under the Apps card.
store-upd-steam-restarts = Steam huanza upya mara hakuna mchezo unaoendeshwa.

