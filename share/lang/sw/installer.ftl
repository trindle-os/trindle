# machine translated (Swahili), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Sakinisha { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Karibu

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Diski

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Faragha

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Nenosiri

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Wewe

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Tayari

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Inasakinisha

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Imekamilika

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Inajiandaa

# Red line at the foot when the installer's helper could not be started.
installer-no-start = Imeshindwa kuanzisha kisakinishi: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Imeshindwa kusoma mpango.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Imeshindwa kusoma mpango: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = Usakinishaji umesimama. Hakuna kingine kilichoandikwa.

# Name of the round button at the foot that goes back a page.
installer-back = Rudi nyuma

# Name of the main button at the foot that goes to the next page.
installer-next = Endelea

# Name of the main button at the foot of the Ready page that starts the install.
installer-install = Sakinisha

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Anzisha upya sasa

# Beside the keyboard test box on the first page.
installer-try-keyboard = Jaribu kibodi

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Andika hapa kujaribu kibodi

# Note beside Install after one press, when the second press writes beside another system.
installer-arm = Bofya tena ili kusakinisha

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = Bofya tena ili kufuta na kusakinisha

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Chagua lugha

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Chagua kibodi

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Chagua ukanda wa saa

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Chagua mahali pa Trindle

# Note beside Next on the disk page while the disks are read again, after one turned out not to be the disk shown.
installer-disks-rereading = Inasoma diski tena

# Note beside Next on the disk password page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-disk-password = Andika nenosiri la diski mara mbili. Lazima liwe sawa mara zote mbili

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Andika jina la kuingia

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Ipe kompyuta hii jina

# Note beside Next on the account page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-password = Andika nenosiri lako mara mbili. Lazima liwe sawa mara zote mbili

# Title of the first page.
installer-welcome-title = Karibu kwenye { $name }

# Sentence under the title of the first page.
installer-welcome-help = Chagua lugha yako, na Trindle itaweka kibodi na ukanda wa saa vinavyolingana nayo, ambavyo unaweza kuvibadilisha.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Lugha

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Kibodi

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Saa

# Under a card's head while nothing has been chosen in its list.
installer-select = Chagua

# Title of the disk page.
installer-disks-title = Mahali pa { $name }

# Sentence under the title of the disk page.
# SAFETY: needs a human review before it ships
installer-disks-help = Kila kitu kilicho kwenye diski unayochagua kitabadilishwa, isipokuwa uchague kusakinisha kando ya kilichopo tayari.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Trindle haikupata diski kubwa ya kutosha. Inahitaji GB 20.

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = Tumia diski nzima, ukibadilisha kila kitu kilicho juu yake

# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = Weka { $what } na ushiriki diski

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = Kwa kilichopo sasa: { $kept }. Kwa Trindle: { $mine }

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Nafasi tupu

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } na { $last }

# Under a disk's name: what is on it.
installer-disk-has = Ina { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Ina { $list }, imefungwa kwa usimbaji fiche

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Ina mfumo uliosimbwa

# Under a disk's name: there is nothing on it.
installer-disk-empty = Tupu

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Ina faili, lakini hakuna mfumo ambao Trindle inautambua

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = Faili ya swap ya GB { $gb }

# Name of the switch that makes a swap file.
installer-swap-switch = Tengeneza faili ya swap

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Boot imewashwa, kwa hiyo kompyuta hii haiwezi kujihifadhi usingizini (hibernate), lakini faili ya swap bado husaidia kumbukumbu inapopungua. Trindle huitengeneza tu ikiwa diski ina nafasi nyingi ya ziada.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = Faili ya swap huiwezesha kompyuta kujihifadhi usingizini (hibernate), ambapo kila kitu kilicho wazi huhifadhiwa hata umeme ukiwa umezimwa. Pia huipa kompyuta nafasi ya ziada kumbukumbu inapopungua. Bila hiyo, kompyuta haiwezi kujihifadhi usingizini. Trindle huitengeneza tu ikiwa diski ina nafasi nyingi ya ziada.

# Title of the page about encrypting the disk.
installer-privacy-title = Kusimba diski kwa usimbaji fiche

# Sentence under the title of that page.
installer-privacy-help = Hakuna anayeweza kusoma diski iliyosimbwa akiitoa kwenye kompyuta hii.

# Head of the card with the three choices.
installer-privacy-locked = Jinsi diski inavyofungwa

# Name and words of the choice not to encrypt the disk.
# SAFETY: needs a human review before it ships
installer-privacy-none = Usisimbe diski

# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = Isimbe, na uombe nenosiri kompyuta inapoanza

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = Isimbe, na uruhusu chipu ya TPM ya kompyuta hii ifungue

# Head of the card about the TPM chip.
installer-privacy-about = Kuhusu chipu

# In the card about the chip, on a computer that has one.
# SAFETY: needs a human review before it ships
installer-privacy-chip = Kompyuta hii ina chipu ya TPM. Itakapoanza mara ya kwanza baada ya kusakinisha, itaomba nenosiri mara moja wakati chipu inaandaliwa. Baada ya hapo chipu hufungua diski na huandiki chochote. Hifadhi nenosiri, kwa sababu utalihitaji ikiwa chipu itabadilishwa au diski ikihamishiwa kompyuta nyingine.

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = Kompyuta hii haina chipu ya TPM ambayo Trindle inaweza kutumia, kwa hiyo nenosiri litaombwa kila inapoanza. Kwenye kompyuta yenye chipu, chipu inaweza kufungua diski yenyewe.

# Title of the disk password page.
installer-disk-title = Nenosiri la diski

# Sentence under the title of the disk password page.
installer-disk-help = Unaliandika kabla ya kitu kingine chochote kompyuta inapoanza.

# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = Nenosiri la diski

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Pendekeza kifungu cha siri

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Andika tena

# Red line under the password boxes when the two do not match.
# SAFETY: needs a human review before it ships
installer-mismatch = Manenosiri hayo mawili hayalingani.

# Head of the card that tells the person to write the password down.
installer-disk-write-down = Liandike

# Words in that card.
# SAFETY: needs a human review before it ships
installer-disk-write-down-text = Andika nenosiri hili kwenye karatasi na uliweke mahali salama, mbali na kompyuta. Bila hilo, hakuna anayeweza kufungua diski kamwe, hata wewe. Hakuna njia ya kurudi ndani. Kifungu cha siri chenye maneno manne ya kawaida ni rahisi kuandika na kigumu kubashiri.

# Title of the account page.
installer-you-title = Kukuhusu

# Sentence under the title of the account page.
installer-you-help = Hii ni akaunti utakayoingia nayo.

# Head of the card with the name boxes.
installer-account = Akaunti yako

# Label and name of the box for the person's name.
installer-your-name = Jina lako

# Label and name of the box for the sign-in name.
installer-user-name = Jina la kuingia

# Label and name of the box for the computer's name.
installer-host-name = Jina la kompyuta hii

# Head of the card with the password boxes.
installer-your-password = Nenosiri lako

# Label and name of the first password box.
# SAFETY: needs a human review before it ships
installer-password = Nenosiri

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Ugumu wa kubashiri

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Andika jina la kuingia.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Jina hilo ni refu mno.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Anza kwa herufi a-z.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Tumia a-z, 0-9, - na _ tu.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Mfumo tayari unatumia jina hilo.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Ipe kompyuta hii jina.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Herufi, tarakimu na - tu.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Kompyuta ingebashiri hili papo hapo.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] Kompyuta ingebashiri hili kwa takriban dakika { $n }.
       *[other] Kompyuta ingebashiri hili kwa takriban dakika { $n }.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] Kompyuta ingebashiri hili kwa takriban saa { $n }.
       *[other] Kompyuta ingebashiri hili kwa takriban saa { $n }.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] Kompyuta ingebashiri hili kwa takriban siku { $n }.
       *[other] Kompyuta ingebashiri hili kwa takriban siku { $n }.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] Kompyuta ingehitaji takriban mwaka { $n } kubashiri hili.
       *[other] Kompyuta ingehitaji takriban miaka { $n } kubashiri hili.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Kompyuta ingehitaji muda mrefu kuliko historia yote iliyorekodiwa kubashiri hili.

# Title of the Ready page.
installer-ready-title = Tayari kusakinisha

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Hakuna kilichobadilika bado

# Head of the card about security updates.
installer-ready-security = Masasisho ya usalama

# In the card about security updates.
installer-ready-security-text = { $name } husakinisha marekebisho ya usalama ya Debian yenyewe kila siku. Unaweza kuzima hili kwenye Mipangilio, Kuhusu kompyuta hii.

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = Kila kitu kilicho kwenye { $disk } kitabadilishwa.

# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Kila kitu kilicho kwenye { $part } kinabaki, lakini kihifadhi nakala kwanza. Umeme ukikatika wakati inapunguzwa ukubwa, faili zinaweza kupotea.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Bado hakuna diski iliyochaguliwa.

# Name of a row in the list: the whole disk will be used.
installer-ready-whole = { $disk } yote

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = Kila kitu kilicho juu yake kinabadilishwa na { $name }.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Kando ya { $part }

# Words of that row.
installer-ready-beside-text = { $part } inapungua hadi { $size }, na { $name } inasakinishwa kwenye nafasi inayopatikana.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Hakuna diski

# Words of that row.
installer-ready-no-disk-text = Chagua diski.

# Name of a row in the list: the disk will not be encrypted.
# SAFETY: needs a human review before it ships
installer-ready-plain = Haijasimbwa

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-plain-text = Yeyote atakayeitoa diski anaweza kuisoma.

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Imesimbwa

# Words of that row when a password is typed at each start.
installer-ready-locked-password = Unaandika nenosiri lake kila kompyuta inapoanza.

# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = Unaandika nenosiri mara moja, kisha chipu ya TPM ya kompyuta hii inafungua diski.

# Added to the end of the disk's row on the last page when a swap file is to be made, with its size in GB.
installer-ready-swap = Pamoja na faili ya swap ya GB { $gb }.

# Under the choice to let the TPM chip unlock the disk, on a computer with no usable chip.
installer-privacy-chip-off = Inahitaji chipu ya TPM ambayo Trindle inaweza kutumia.

# In the card about the chip when the disk is not being encrypted, on a computer that has a chip.
installer-privacy-chip-none = Kompyuta hii ina chipu ya TPM, inayoweza kufungua diski iliyosimbwa.

# In the card about the chip when the disk is not being encrypted, on a computer with no chip Trindle can use.
installer-privacy-no-chip-none = Kompyuta hii haina chipu ya TPM ambayo Trindle inaweza kutumia.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Kibodi { $keyboard }, saa { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = Inasakinisha { $name }

# Head of the card that holds the short tour.
installer-tour = Ziara fupi

# Title of the last page.
installer-done-title = { $name } imesakinishwa

# Sentence under the title of the last page.
installer-done-help = Toa USB stick, kisha anzisha upya.

# Head of the card that repeats the warning about the disk password.
installer-done-keep = Hifadhi nenosiri la diski salama

# In that card when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-keep-chip = Diski imesimbwa. Kompyuta itakapoanza mara ya kwanza, itaomba nenosiri la diski mara moja wakati chipu inaandaliwa. Baada ya hapo diski hujifungua yenyewe. Andika nenosiri hilo kwenye karatasi na ulihifadhi mahali salama, mbali na kompyuta hii. Bila hilo, hakuna anayeweza kufungua diski kamwe, hata wewe.

# In that card when the password is typed at each start.
# SAFETY: needs a human review before it ships
installer-keep-password = Diski imesimbwa. Kama hujafanya hivyo, andika nenosiri la diski kwenye karatasi sasa na ulihifadhi mahali salama, mbali na kompyuta hii. Bila hilo, hakuna anayeweza kufungua diski kamwe, hata wewe.

# Title of the first slide.
installer-slide-ring = Kila kitu katika duara moja

# Words of the first slide.
installer-slide-ring-text = Sogeza kielekezi hadi ukingo wa skrini na ubofye mara mbili ili kufungua duara. Programu zako ziko kwenye ukingo wa nje, na saa na swichi ziko katikati.

# Title of the second slide.
installer-slide-snapshots = Tengua sasisho baya

# Words of the second slide.
installer-slide-snapshots-text = Trindle huchukua picha ya mfumo (snapshot) kabla na baada ya kila badiliko la programu. Kwenye Mipangilio unaweza kurudisha mfumo mzima kwenye yoyote kati yake. Kompyuta isipoanza, unaweza kufanya hivyo kutoka kwenye menyu ya kuwasha.

# Title of the third slide.
installer-slide-apps = Programu zisizokusumbua

# Words of the third slide.
installer-slide-apps-text = Programu za Flathub, programu za Debian yenyewe, AppImage na vyanzo vya programu vyote viko mahali pamoja, pamoja na nakala rudufu unazoweza kubeba hadi kompyuta nyingine.

# Title of the fourth slide.
installer-slide-yours = Ifanye yako

# Words of the fourth slide.
installer-slide-yours-text = Chagua nuru au giza, chagua sauti zako mwenyewe, na utumie ubao wa kunakili unaokumbuka ulichonakili.

# Title of the fifth slide.
installer-slide-small = Ndogo na tulivu

# Words of the fifth slide.
installer-slide-small-text = Eneo-kazi hutumia kumbukumbu kidogo na huanza kwa sekunde chache, hivyo kompyuta ya zamani huhisi mpya tena.

# Title of the tour slide about the ring.
installer-tour-ring = Duara ndilo eneo-kazi lote

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Weka kielekezi kwenye kona hii na duara litafunguka

# Note on the ring picture, at the time.
installer-tour-ring-time = Saa na tarehe ziko katikati

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Programu zako ziko kwenye ukingo wa nje

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Swichi za sauti, skrini, programu zako zote, kuzuia kompyuta kulala na kuzima

# Title of the tour slide about Settings.
installer-tour-settings = Mipangilio, kwa maneno rahisi

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Andika hapa kupata mpangilio

# Note on the Settings picture, at Network.
installer-tour-settings-network = Mtandao ni mahali pa kujiunga na mtandao wa Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Nuru au giza, wakati wowote upendao

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Rangi ishirini, au andika yako mwenyewe

# Title of the tour slide about Apps.
installer-tour-apps = Programu, kwa kila kitu kingine

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Tafuta programu kwa jina au kwa kile inachofanya

# Note on the Apps picture, at the list.
installer-tour-apps-many = Maelfu ya programu za bure, kutoka Flathub na Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Masasisho ya kila kitu, ikiwemo Trindle yenyewe

# Title of the tour slide about voice control.
installer-tour-voice = Ongea na kompyuta yako

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = Udhibiti wa sauti uko kwenye Mipangilio, kisha Pete

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Bonyeza Ongeza udhibiti kwa sauti mara moja. Upakuaji ni takriban MB 150 na huchukua dakika chache

# Note on the voice picture, about what to say.
installer-tour-voice-say = Kisha gusa katikati ya duara na useme jina la programu, “sauti juu”, au “mipangilio ya Wi-Fi”

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Trindle inaelewa unachosema kwenye kompyuta hii, na hakitumwi popote pengine kamwe

# Title of the tour slide about snapshots.
installer-tour-snapshots = Rudi kabla ya badiliko

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Trindle huchukua picha ya mfumo kabla ya kila sasisho

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Kitu kikiharibika, unaweza kurudi jinsi ilivyokuwa kutoka kwenye menyu ya kuwasha

# Name of the picture of the ring, and the head of its card.
installer-live-caption = Duara, lenye kila kitu ndani yake

# Sentence under the title of the welcome from a stick.
installer-live-help = Unatumia kutoka kwenye USB stick, kwa hiyo hakuna kilichobadilishwa kwenye kompyuta hii.

# Head of the card with the three first things to do.
installer-live-begin = Kuanza

# Name of the first thing to do.
installer-live-open = Fungua duara

# Line under it.
installer-live-open-line = Bofya mara mbili ukingoni mwa skrini, au bonyeza Super.

# Name of the second thing to do.
installer-live-speak = Tumia sauti yako

# Line under it when voice control is there.
installer-live-speak-line = Gusa katikati ya duara na useme unachotaka.

# Line under it when voice control has to be added.
installer-live-speak-add = Bonyeza Ongeza udhibiti kwa sauti kwenye Mipangilio, chini ya Pete. Kisha gusa katikati na uongee.

# Name of the third thing to do.
installer-live-tab = Vyote kwenye duara

# Line under it.
installer-live-tab-line = Utapata kilicho wazi, arifa zako na ubao wa kunakili hapo.

# The choice of the dark look.
installer-live-dark = Giza

# The choice of the light look.
installer-live-light = Nuru

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Nuru au giza, kwa eneo-kazi zima

# Name of the round button that closes this window and opens the ring.
installer-live-around = Tazama kwanza

# Name of the main button that opens the installer.
installer-live-install = Sakinisha { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = Inatengeneza nafasi

# Foot of the install page while the disk is prepared.
installer-step-prepare = Inaandaa diski

# Foot of the install page while a place is made beside another system.
installer-step-place = Inatengeneza nafasi ya Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = Inakagua kilichopo tayari kwenye diski

# Foot of the install page while another system is made smaller.
installer-step-smaller = Inapunguza ukubwa wa mfumo mwingine

# Foot of the install page while the disk is encrypted.
installer-step-lock = Inasimba diski

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Inaweka chipu ya TPM

# Foot of the install page while the system is copied.
installer-step-copy = Inanakili Trindle kwenye diski

# Foot of the install page while settings are written.
installer-step-settings = Inaandika mipangilio

# Foot of the install page while the person's account is made.
installer-step-account = Inatengeneza akaunti yako

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Inaondoa vya ziada vya USB

# Foot of the install page while the start-up files are made.
installer-step-startup = Inatengeneza faili za kuanzia

# Foot of the install page while the boot menu is set up.
installer-step-boot = Inaweka menyu ya kuwasha

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Inaweka picha za mfumo (snapshots)

# Foot of the install page at the very end.
installer-step-finishing = Inamalizia

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } haikuweza kuanza: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } haikumaliza. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = Imeshindwa kuandika { $path }: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } si diski.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } si diski kwenye kompyuta hii.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Hakuna diski iliyochaguliwa.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Jina hilo la kuingia haliwezi kutumiwa na Trindle.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Jina hilo la kompyuta haliwezi kutumiwa na Trindle.

# Red line at the foot: the language cannot be used.
installer-bad-language = Lugha hiyo haiwezi kutumiwa na Trindle.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Kibodi hiyo haiwezi kutumiwa na Trindle.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Ukanda huo wa saa hauwezi kutumiwa na Trindle.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Jina hilo lina herufi ambayo Trindle haiwezi kutumia.

# Red line at the foot: a password has a line break in it.
# SAFETY: needs a human review before it ships
installer-bad-password = Nenosiri haliwezi kuwa na mstari mpya.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = Sehemu ya pili ya kisakinishi inahitaji kuendeshwa kama root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = Diski hii haina kizigeu cha kuwasha cha kushiriki, kwa hiyo Trindle haiwezi kusakinishwa kando ya kilichopo juu yake.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Trindle haikupata kizigeu kipya.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = Imeshindwa kusoma ukubwa wa { $part }.

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle haiwezi kupunguza ukubwa wa vizigeu vya { $fs }.

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle haiwezi kupunguza ukubwa wa kizigeu cha aina isiyojulikana.

# Red line at the foot: encrypting needs a password and there is none.
# SAFETY: needs a human review before it ships
installer-disk-password-needed = Unahitaji nenosiri ili kusimba diski.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Imeshindwa kunakili mfumo: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Imeshindwa kunakili mfumo. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Hakuna nafasi ya faili ya swap.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Trindle haikuweza kusoma mahali faili ya swap ilipo kwenye diski.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Trindle haikuweza kutambua vizigeu vipya.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Imeshindwa kuweka ukanda wa saa: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Trindle haikuweza kusakinisha menyu ya kuwasha kwenye kompyuta hii.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Una uhakika?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Ona kilicho wazi, tumia ubao wa kunakili, na usafishe skrini kwa kitufe kimoja

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } ni kijiti ambacho Trindle inaendeshwa kutoka kwacho.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } inatumika. Funga chochote kinachoitumia, kisha jaribu tena.

# Said beside how far the install has got, once it has started.
installer-started = Inasakinisha

# The button that ends an install under way.
installer-halt = Simamisha

# Beside Stop after its first press.
installer-halt-sure = Kusimamisha usakinishaji? Yaliyokuwa kwenye diski tayari yamepotea.

# While an install is being stopped.
installer-halting = Inasimamisha

# When Stop could not let go of the disk.
installer-halt-stuck = Trindle haikuweza kuachilia diski. Washa kompyuta upya ujaribu tena.

# The copy is done and the person is still answering.
installer-step-waiting = Imenakiliwa. Inasubiri majibu yako.



installer-already-running = Kisakinishi kingine tayari kinafanya kazi kwenye kompyuta hii.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle inahitaji angalau GB 1 ya kumbukumbu, na kompyuta hii ina kidogo kuliko hiyo.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Unaweza kusimba diski tu kwenye kompyuta inayoanza kwa UEFI, na hii haifanyi hivyo.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows kwenye { $part } ilikuwa imejihifadhi usingizini (hibernate) au kuzimwa kwa kuanza haraka. Kuipunguza ukubwa sasa kunaweza kupoteza faili zake. Washa Windows, chagua Anzisha upya, kisha jaribu tena.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle imesakinishwa, lakini haikuweza kusanidi picha za hali: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle imesakinishwa, lakini haikuweza kuweka picha za hali kwenye menyu ya kuwasha: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Si kila kitu kilisanidiwa

installer-suggest-failed = Trindle haikuweza kutengeneza nenosiri. Andika lako mwenyewe.

installer-disk-changed = { $dev } si diski iliyoonyeshwa. Rudi nyuma na uchague diski tena.

installer-no-gpt = { $dev } haitumii jedwali la vizigeu la GPT, kwa hiyo Trindle haiwezi kusakinishwa kando ya kilichopo juu yake.

installer-boot-partition-small = Kizigeu cha kuwasha kwenye diski hii ni kidogo mno kushirikiwa, kwa hiyo Trindle haiwezi kusakinishwa kando ya kilichopo juu yake.

installer-swap-failed = Trindle imesakinishwa, lakini haikuweza kutengeneza faili ya swap: { $why }

installer-uuid-missing = Trindle haikuweza kutambua { $dev } baada ya kuiandaa.

installer-ready-replaced-known = Kila kitu kilicho kwenye { $disk } kitabadilishwa ({ $holds }).

installer-disk-named = { $what }, { $dev }

installer-run-stopped = { $program } ilisimama kabla ya kupata kila ilichohitaji: { $why }
