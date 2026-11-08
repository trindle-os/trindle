# machine translated (Swahili), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Imehifadhiwa

# Said beside a button that worked, when nothing more is known.
settings-common-done = Imekamilika

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Haikufanyika

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Kuhusu ukurasa huu

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = Kuhusu { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } ipo tayari

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] Fanya vivyo hivyo kwa zilizobaki (nyingine { $n })
       *[other] Fanya vivyo hivyo kwa zilizobaki (nyingine { $n })
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Fanya vivyo hivyo kwa zilizobaki

# Tip of the button that keeps the one already there.
settings-common-skip = Ruka: weka iliyopo tayari

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = Badilisha

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = Badilisha iliyopo tayari na inayoletwa

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = Inayoletwa

# Heading of the line that shows the one already there.
settings-common-already-line = Iliyopo tayari

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = Ibadilishe iliyopo tayari?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] Itarudi nyuma baada ya sekunde { $left }.
       *[other] Itarudi nyuma baada ya sekunde { $left }.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] Itarudi nyuma baada ya sekunde { $left }. { $more }
       *[other] Itarudi nyuma baada ya sekunde { $left }. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Rudi nyuma sasa

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Weka

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } haipatikani

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Iweke kwa: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = Ukurasa wa wavuti haukuweza kufunguliwa: hakuna kivinjari kilichoweza kuanzishwa.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } haikuweza kufunguliwa.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Haikuweza kuhifadhiwa.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Haikufanyika: basi la ujumbe la mfumo halikufikika. Washa kompyuta upya, kisha jaribu tena.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Haikufanyika: basi la ujumbe la kuingia huku halikufikika. Toka kisha ingia tena, halafu jaribu tena.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Dirisha la kuchagua halikuweza kufunguliwa. Toka kisha ingia tena, halafu jaribu tena.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = Diski imejaa.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Diski haiwezi kuandikwa kwa sasa.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = Sehemu ya mfumo inayofanya hili haifanyi kazi.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = Ilichukua muda mrefu kujibu. Jaribu tena.

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = Nenosiri halikutolewa.

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = Akaunti hii hairuhusiwi kufanya hivyo.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Kuna kitu kinachohitajika kinakosekana kwenye kompyuta hii.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } na { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = Pete hufunguka kwa kubofya ukingoni mwa skrini.

# Word drawn in the middle of that picture.
settings-common-ring-name = Pete

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Firmware ya kompyuta mpakato haikufikika: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = Firmware ya kompyuta mpakato haikukubali kikomo cha kuchaji.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Kompyuta hii hairuhusu kuchaji kusimamishwa mapema.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Haikuweza kuweka { $path }: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Kujaribu Trindle kwenye mashine pepe

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle inafanya kazi kwenye { $machine }. Inafanya kazi vizuri huko, lakini mashine pepe haiwi haraka wala laini kama kompyuta halisi, kwa hiyo tafadhali pima kasi yake kwenye kompyuta halisi.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle inafanya kazi kwenye { $machine } bila michoro ya 3D, kwa hiyo kichakataji lazima kichore kila kitu ambacho chipu ya michoro kwa kawaida huchora. Madirisha, video na kurasa za wavuti zitahisi polepole kuliko kwenye kompyuta halisi, na Trindle huacha uhuishaji wake ili kusaidia. Kama kompyuta inayoiendesha inaweza, kuwasha kasi ya 3D kwenye mipangilio ya skrini ya mashine pepe hubadilisha sana. Ikiwekwa kwenye kompyuta halisi, hakuna kati ya haya linalohusika.
