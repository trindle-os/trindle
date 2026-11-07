# machine translated (Swahili), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Diski

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Kuhusu ukurasa huu

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Kila diski iliyo ndani au iliyochomekwa kwenye kompyuta hii, ikichorwa kama mstari wa sehemu zake. Kila sehemu ina upana wa ukubwa wake na imejazwa kadiri inavyotumika, kwa hiyo sehemu iliyojaa huonekana kwa urahisi.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = Chini ya picha, kila diski ina kadi. Fungua sehemu, iunganishe au itenganishe, au toa diski kwa usalama au uizime. Penseli kwenye sehemu hufungua mabadiliko yake: jina lake, folda yake yenyewe na kama iko tayari wakati wa kuwasha, ukubwa wake, kuikagua na kuirekebisha, kuifomati au kuifuta. Alama ya kujumlisha kwenye nafasi huru hutengeneza sehemu mpya.

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = Diski ambayo mfumo unaendeshwa kutoka kwake huonyeshwa lakini haitolewi kamwe kwa mabadiliko yoyote yanayofuta au kuhamisha vitu. Kila mabadiliko kwa diski yoyote huomba nenosiri la msimamizi, kila mara. Jicho huficha diski au sehemu kutoka Folda na pete: hubaki hapa, kijivu, na haijafungwa.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Kila sehemu ya kila diski, imejazwa kadiri inavyotumika.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Diski zako

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Diski haziwezi kusomwa kwa sasa.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Jaribu tena

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Tafuta tena

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Hakuna diski zilizopatikana. Chomeka moja na itaonekana hapa.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = Huduma ya diski

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = Basi la mfumo

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } haipatikani. Isakinishe kwa: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Inatazama diski…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } inaharibika. Nakili sasa chochote unachotaka kuhifadhi kutoka kwake.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } inahitaji kutazamwa: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Diski na nafasi iliyo juu yake.
        [one] Diski na nafasi iliyo juu yake, na diski moja iliyochomekwa.
       *[other] Diski na nafasi iliyo juu yake, na diski { $plugged } zilizochomekwa.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Karibu imejaa: { $free } tu ndizo huru kwenye kompyuta hii. Futa au hamisha faili fulani hivi karibuni.
        [one] Karibu imejaa: { $free } tu ndizo huru kwenye kompyuta hii, na diski moja imechomekwa. Futa au hamisha faili fulani hivi karibuni.
       *[other] Karibu imejaa: { $free } tu ndizo huru kwenye kompyuta hii, na diski { $plugged } zimechomekwa. Futa au hamisha faili fulani hivi karibuni.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Inajaa: { $free } huru kwenye kompyuta hii.
        [one] Inajaa: { $free } huru kwenye kompyuta hii, na diski moja imechomekwa.
       *[other] Inajaa: { $free } huru kwenye kompyuta hii, na diski { $plugged } zimechomekwa.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Nafasi ya kutosha: { $free } huru kwenye kompyuta hii.
        [one] Nafasi ya kutosha: { $free } huru kwenye kompyuta hii, na diski moja imechomekwa.
       *[other] Nafasi ya kutosha: { $free } huru kwenye kompyuta hii, na diski { $plugged } zimechomekwa.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Kompyuta hii

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Faili zako

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } bado huru, mahali mfumo ulipo.
       *[files] { $free } bado huru, kwa faili zako mwenyewe.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: imejaa { $full }, { $free } huru kati ya { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = kati ya { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Hakuna vipimo, diski hazisemi hali yake

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Nzuri, kama diski yenyewe inavyosema

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] Nzuri, kama diski zote { $n } zinavyosema
       *[other] Nzuri, kama diski zote { $n } zinavyosema
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } inahitaji kutazamwa, ona kadi yake
       *[other] { $n } zinahitaji kutazamwa, ona kadi zake
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] Diski 1, { $total } kwa jumla. { $health }.
       *[other] Diski { $n }, { $total } kwa jumla. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } huru

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Haijaunganishwa

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Nafasi huru

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = Hakuna diski wala kadi ndani yake

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Bado hakuna kitu juu yake

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB au inayoondolewa

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, ndani ya kompyuta hii

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB au inayoondolewa, imefichwa

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, ndani ya kompyuta hii, imefichwa

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, imejaa { $full }, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Sehemu

# Name given to a drive that gives no maker or model.
disks-drive-name = Diski

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Swap

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Imesimbwa, imefunguliwa

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Imesimbwa, imefungwa

# Kind of a part that holds other parts.
disks-kind-container = Ina sehemu nyingine

# Kind of a part that has no filesystem on it.
disks-kind-none = Hakuna mfumo wa faili

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, sehemu ambayo kompyuta huanzia
        [legacy] { $kind }, sehemu ambayo kompyuta ya zamani huanzia
       *[loader] { $kind }, sehemu ya kipakiaji cha kuwasha
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = kompyuta yoyote

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = Linux pekee

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = vifaa vya zamani

# Trouble a drive reports about its spare space.
disks-trouble-spare = nafasi ya akiba inaisha

# Trouble a drive reports about its heat.
disks-trouble-temperature = ina joto mno

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = inachakaa

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = imejifanya ya kusomwa tu

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = hifadhi ya kumbukumbu yake imeshindwa

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = sehemu yake imejifanya ya kusomwa tu

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = imechakaa kupita anavyotarajia mtengenezaji wake

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] doa { $n } lililoharibika
       *[other] madoa { $n } yaliyoharibika
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] imetumika kwa saa { $hours }
       *[other] imetumika kwa saa { $hours }
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Inaharibika: nakili sasa chochote unachotaka kuhifadhi kutoka kwake

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Inahitaji uangalizi: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Nzuri

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Hakuna diski inayojibu

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = Kasha linajibu, lakini diski iliyo ndani yake haijibu. Ichomoe, subiri sekunde kumi kisha uchomeke kwenye mlango wa buluu au USB-C kwa kebo yake yenyewe; ikibaki hivi, diski iliyo ndani inaweza kuwa imelegea au kuharibika.

# Tip of the round button on a drive's card that takes the drive out safely.
# SAFETY: needs a human review before it ships
disks-remove-safely = Ondoa kwa usalama

# Tip of the round button on a drive's card that powers the drive off.
# SAFETY: needs a human review before it ships
disks-power-off-tip = Izime

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Futa diski nzima

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Onyesha diski hii
       *[part] Onyesha sehemu hii
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Ficha diski hii. { $lock }
       *[part] Ficha sehemu hii. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Imefichwa kutoka Folda na pete. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = Mfumo unaendeshwa kutoka humo: hakuna kitu hapa kinachoweza kufutwa.

# Note on the card of a drive that has no partition table.
disks-no-table = Hakuna jedwali la sehemu: diski nzima ni huru.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = haijaunganishwa

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = huunganishwa wakati wa kuwasha katika { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = huonekana katika { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = katika { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Fungua

# Tip of the round button that mounts a part.
disks-mount = Unganisha

# Tip of the round button that unmounts a part.
disks-unmount = Tenganisha

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Ibadilishe

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Jina

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Jina (si lazima)

# Tip of the button that saves a part's new name.
disks-rename-tip = Ibadilishe jina

# Note under the Name row of a part.
disks-name-note = Inaitwa nini hapa na kwenye kompyuta nyingine.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Jina la folda

# Tip of the button that saves a part's folder.
disks-use-folder = Tumia folda hii

# Label of the row where a part's folder is chosen.
disks-folder = Folda

# Note under the Folder row of a part.
disks-folder-note = Mahali faili zake zinapoonekana. Iache tupu ili kompyuta ichague.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Unganisha wakati wa kuwasha

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Iko tayari kompyuta inapowashwa. Isipokuwepo, kompyuta huwaka vilevile.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Ukubwa kwa GB

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = Kuanzia { $least } hadi { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = Kuanzia { $least } hadi { $most }. Iunganishe ili kuona inaweza kuwa ndogo kiasi gani.

# Tip of the button that resizes a part.
disks-resize-tip = Ibadilishe ukubwa

# Label of the row where a part is resized.
disks-size = Ukubwa

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = Hii itafuta kila kitu kwenye { $words }. Haiwezi kutenduliwa.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Bofya tena: { $name } kwenye { $drive } inatoka { $from } hadi { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Tengeneza nafasi kwa sehemu mpya

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Nafasi kwa GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Tengeneza nafasi

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Bofya tena: { $name } kwenye { $drive } inapoteza { $size }, baada ya picha ya hali

# Note under the row that makes room.
disks-room-note = Inapunguzwa ikiwa inatumika, baada ya picha ya hali, ikiacha nafasi huru.

# Tip of the button that checks a part for faults.
disks-check-tip = Ikague kwa hitilafu

# Tip of the button that repairs a part.
disks-repair-tip = Irekebishe

# Label of the row where a part is checked and repaired.
disks-faults = Hitilafu

# Note under the Faults row.
disks-faults-note = Hutafuta hitilafu katika jinsi faili zake zinavyohifadhiwa, na kuzirekebisha. Hutenganishwa wakati huu unaendelea.

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = Bofya tena ili kurekebisha { $name }, kwenye { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Aina

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = Fomati

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = Ifomati

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = Huianza upya kama sehemu moja, ikijaza diski.

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = Hufuta kila kitu kwenye sehemu hii tu.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Bofya tena: kila kitu kwenye { $words } kitafutwa

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, kwenye { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = Futa { $name }, { $size }, kwenye { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Futa sehemu hii

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = Nafasi yake inakuwa huru, tayari kwa sehemu mpya au ile ya kabla yake kukua ndani yake.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Tengeneza sehemu mpya hapa

# Label of the row where a new part is made.
disks-new-part = Sehemu mpya

# Note under the row where a new part is made.
disks-new-part-note = Inatengenezwa katika nafasi hii huru, na kufomatiwa tayari kutumika.

# Tip of the button that makes a new part.
disks-make-tip = Itengeneze

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-make-warning = Bofya tena ili kutengeneza sehemu ya { $size } kwenye { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = Futa { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, kwenye { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Sehemu mpya

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } huru, kwenye { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Funga

# Words on a row while a part is being mounted.
disks-mounting = Inaunganisha

# Words on a row when a part has been mounted.
disks-mounted = Imeunganishwa

# Words on a row while a part is being unmounted.
disks-unmounting = Inatenganisha

# Words on a row when a part has been unmounted.
disks-unmounted = Imetenganishwa

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Inaifunga

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Salama kuondolewa

# Words on a drive's row while it is being powered off.
disks-powering-off = Inazima

# Words on a drive's row when it has been powered off.
disks-off-safe = Imezimwa: salama kuchomoa

# Words on a row while a part is being renamed.
disks-renaming = Inabadilisha jina

# Words on a row when a part has been renamed.
disks-renamed = Jina limebadilishwa

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = Kompyuta huchagua folda yake tena

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Inaonekana katika { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Huunganishwa wakati wa kuwasha katika { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Haiguswi wakati wa kuwasha

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Inahifadhi

# Words on a row while a part is being resized.
disks-resizing = Inabadilisha ukubwa

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Ukubwa umebadilishwa hadi { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Faili hazikuweza kupatikana, kwa hiyo nafasi haiwezi kutengenezwa.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Inachukua picha ya hali

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Picha ya hali haikuweza kupatikana baadaye.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Hakuna kilichobadilishwa ukubwa.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Mambo yalibadilika wakati picha ya hali ilipochukuliwa, kwa hiyo hakuna kilichobadilishwa ukubwa.

# Words on a row while room is being made.
disks-making-room = Inatengeneza nafasi

# Words on a row when room has been made. { $size } is how much.
disks-room-made = Nafasi ya { $size } imetengenezwa

# Words on a row while a part is being repaired.
disks-repairing = Inarekebisha

# Words on a row when a part has been repaired.
disks-repaired = Imerekebishwa

# Red words when some faults could not be put right.
disks-repair-failed = Hitilafu fulani hazikuweza kurekebishwa. Nakili chochote unachotaka kuhifadhi.

# Words on a row while a part is being checked.
disks-checking = Inakagua

# Words on a row when the check found no faults.
disks-no-faults = Hakuna hitilafu zilizopatikana

# Red words when the check found faults.
disks-faults-found = Hitilafu zimepatikana: bonyeza spana ili kuzirekebisha.

# Words on a row while a part or drive is being formatted.
# SAFETY: needs a human review before it ships
disks-formatting = Inafomati

# Words on a row when a part has been formatted.
# SAFETY: needs a human review before it ships
disks-formatted = Imefomatiwa

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
# SAFETY: needs a human review before it ships
disks-erasing-whole = kufuta diski nzima

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = Sehemu mpya haikuweza kupatikana.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Huduma ya diski haikuweza kufikiwa.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Kitambulisho cha sehemu mpya hakikuweza kusomwa.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = Inafuta

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = Imefutwa

# Words on a row while a new part is being made.
disks-making-it = Inaitengeneza

# Words on a row when a new part has been made.
disks-made = Imetengenezwa

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = Hilo halikuweza kuhifadhiwa: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
# SAFETY: needs a human review before it ships
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Lipe jina kwanza.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] Jina hilo ni refu mno: herufi { $max } kwa kiwango cha juu.
       *[other] Jina hilo ni refu mno: herufi { $max } kwa kiwango cha juu.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Tumia herufi, nambari, - na _ tu (bila nafasi).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Ianze kwa herufi au nambari.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } inatumika tayari.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] Jina hilo ni refu mno kwa aina hii: herufi { $max } kwa kiwango cha juu.
       *[other] Jina hilo ni refu mno kwa aina hii: herufi { $max } kwa kiwango cha juu.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Majina ya FAT hayawezi kuwa na * ? . , ; : / \ | + = < > [ ] au ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Iunganishe kwanza: aina hii inaweza kuongezwa ukubwa ikiwa imeunganishwa tu.
       *[smaller] Iunganishe kwanza: aina hii inaweza kupunguzwa ukubwa ikiwa imeunganishwa tu.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Aina hii haiwezi kuongezwa ukubwa.
       *[smaller] Aina hii haiwezi kupunguzwa ukubwa.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Hakuna nenosiri lililotolewa, kwa hiyo hakuna kilichobadilika.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Hakuna nenosiri lililotolewa, kwa hiyo kazi ilisimama katikati.

# Red words when another change is pressed while one is still being made.
disks-change-going = Mabadiliko yanafanywa.

disks-one-at-a-time = Mabadiliko mengine bado yanafanywa. Jaribu tena yatakapokwisha.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = Haikuweza kuhakikishwa kwamba nenosiri litaulizwa, kwa hiyo hakuna kilichobadilika.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = Nenosiri lililotolewa sasa halikuweza kuondolewa, kwa hiyo kwa dakika chache mabadiliko mengine kwa diski yanaweza kutouliza nenosiri. Inajaribiwa tena.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = Nenosiri lililotolewa sasa limeondolewa: mabadiliko yanayofuata yataomba nenosiri.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Huduma ya diski haikuweza kufikiwa, kwa hiyo hakuna kilichobadilika.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Diski ilifungwa wakati mabadiliko haya yalipokuwa yakifanywa.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Diski hubaki wazi hadi haya yamalizike, ili iweze kusema yaliishaje.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, kwa hiyo hakuna kilichobadilika.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, kwa hiyo kazi ilisimama katikati.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Huduma ya diski haikuweza kuulizwa

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } Kazi ilisimama katikati.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Zilizofanywa: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Kilichoshindwa: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Zisizofanywa: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Haijulikani: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } ilitenganishwa kwa ajili ya hili na haijaunganishwa sasa.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } ilitenganishwa kwa ajili ya hili na bado imetenganishwa: iunganishe kutoka safu yake.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } huenda ilitenganishwa: tazama safu yake.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = { $dev } ilitenganishwa kwa ajili ya hili, na kuiunganisha tena kunahitaji nenosiri lake lenyewe: iunganishe kutoka safu yake.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } ilitenganishwa kwa ajili ya hili na haikuweza kuunganishwa tena ({ $why }): iunganishe kutoka safu yake.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Imefomatiwa, lakini haikupewa tena folda yake { $dir }. { $why } Chagua folda tena kutoka safu yake.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Kuna kitu bado kinaitumia. Funga madirisha au programu zinazoonyesha faili zake, kisha jaribu tena.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Hakuna nafasi ya kutosha kwa kilichomo kwa ukubwa huo.

# Step of a change. { $dev } is the part.
disks-step-unmount = kutenganisha { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = kuunganisha { $dev } tena

# Step of a change.
disks-step-eject = kufunga diski ili iondolewe

# Step of a change.
disks-step-power-off = kuzima diski

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = kufomati { $dev }

# Step of a change.
disks-step-create = kutengeneza sehemu mpya

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = kufuta { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = kubadilisha ukubwa wa sehemu { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = kubadilisha ukubwa wa kilichomo kwenye { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = kurekebisha { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = kukagua { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = kubadilisha jina la { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = kuhifadhi folda na chaguo la kuwasha la { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = hatua { $method } kwenye { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Sehemu hii ya { $kind } inaweza kupunguzwa ukubwa isipotumika tu, kwa hiyo nafasi haiwezi kutengenezwa kutoka kwake hapa.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Sehemu hii ya xfs haiwezi kupunguzwa ukubwa.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Sehemu hii ya { $kind } haiwezi kupunguzwa ukubwa ikiwa inatumika.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Picha ya hali inaweza kuchukuliwa tu kwa diski ya mfumo yenyewe, kwa hiyo nafasi haiwezi kutengenezwa hapa.

# Red words under the Make room row.
disks-room-not-in-table = Si sehemu ya jedwali la diski, kwa hiyo nafasi haiwezi kutengenezwa kutoka kwake.

# Red words under the Make room row.
disks-room-no-online-shrink = Huduma ya diski haiwezi kupunguza Btrfs ikiwa inatumika hapa, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-holds-parts = Ina sehemu nyingine, kwa hiyo nafasi haiwezi kutengenezwa kutoka kwake.

# Red words under the Make room row.
disks-room-not-gpt = Diski yake haina jedwali la GPT, kwa hiyo nafasi haiwezi kutengenezwa kutoka kwake.

# Red words under the Make room row.
disks-room-mount-first = Iunganishe kwanza: Btrfs hupunguzwa ukubwa ikiwa imeunganishwa.

# Red words under the Make room row.
disks-room-mounts-not-one = Haijaunganishwa mahali pamoja hasa, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-read-only = Imeunganishwa kwa kusomwa tu, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-read-only-unknown = Haikuweza kujulikana kama ni ya kusomwa tu, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-many-drives = Faili zimeenea kwenye zaidi ya diski moja, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-drives-unknown = Haikuweza kujulikana faili zimeenea kwenye diski ngapi, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-swap-file = Faili ya swap iko juu yake, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-swap-unknown = Haikuweza kujulikana kama faili ya swap iko juu yake, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-used-unknown = Kiasi kinachotumika hakikuweza kusomwa, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-area-unknown = Huduma ya diski haisemi eneo la faili ni kubwa kiasi gani, kwa hiyo haikuweza kukaguliwa na nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-open-missing = Nakala yake iliyofunguliwa haikuweza kupatikana, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-encryption-sizes = Ukubwa uliotolewa kwa usimbaji haulingani, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row.
disks-room-sizes = Ukubwa uliotolewa kwake haulingani, kwa hiyo nafasi haiwezi kutengenezwa.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Ina { $used } na inahitaji zaidi ya hiyo kuhifadhiwa, kwa hiyo kidogo mno kinaweza kuachiliwa.

# Red words under the Make room row.
disks-room-sizes-unread = Ukubwa wake haukuweza kusomwa.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Nafasi kuanzia { $least } hadi { $most } inaweza kutengenezwa.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, kwa hiyo kazi ilisimamishwa.

# What was found wrong while making room.
disks-room-check-unmounted = Faili hazijaunganishwa tena

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = Sehemu ni { $is } na inapaswa kuwa { $should }

# What was found wrong while making room.
disks-room-check-open = Nakala iliyofunguliwa ya usimbaji haina ukubwa unaopaswa kuwa nao

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = Eneo la faili ni { $is } na linapaswa kuwa { $should }

# What was found wrong while making room.
disks-room-check-area-small = Eneo la faili ni dogo kuliko kinachotumika

# What was found wrong while making room.
disks-room-check-fit = Kinachoshikilia vitu hakitoshi tena

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] Sehemu haijaorodheshwa tena na huduma ya diski, kwa hiyo kazi ilisimamishwa.
        [copy] Nakala iliyofunguliwa haijaorodheshwa tena na huduma ya diski, kwa hiyo kazi ilisimamishwa.
       *[files] Mfumo wa faili haujaorodheshwa tena na huduma ya diski, kwa hiyo kazi ilisimamishwa.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = kupunguza eneo la faili

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = kupunguza nakala iliyofunguliwa ya usimbaji

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = kupunguza sehemu

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = kukagua ukubwa

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Kabla ya kutengeneza nafasi chini ya { $dev } ({ $at })


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Bofya tena: { $name } kwenye { $drive } inatoka { $from } hadi { $to }. Hifadhi nakala ya faili zake kwanza, kwa sababu umeme ukikatika inapopunguzwa unaweza kupoteza faili.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows kwenye { $name } ilikuwa imejihifadhi usingizini (hibernate) au kuachwa katika kuanza haraka, kwa hiyo kuipunguza ukubwa kunaweza kupoteza faili zake. Washa Windows, chagua Anzisha upya, kisha jaribu tena.

