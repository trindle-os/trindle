# machine translated (Swahili), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = Hakuna kilichorudishwa, kwa sababu { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Trindle ilirudisha sehemu tu ya nakala rudufu, kwa sababu { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = nakala rudufu ina kiungo au kitu kingine kinachofika nje ya folda inakoenda

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = kuna kitu kwenye kompyuta hii tayari mahali ambapo sehemu ya nakala rudufu inapaswa kwenda

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = hakuna mahali pa kufungua nakala rudufu

# The reason when the file is not a Trindle backup.
store-because-not-trindle = faili hiyo si nakala rudufu ya Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = nakala rudufu imeharibika na Trindle haikuweza kusoma orodha yake

# The reason when the file is not a Flat backup.
store-because-not-flat = faili hiyo haionekani kama nakala rudufu ya Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = faili hiyo haina orodha ya programu ya Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = ilitengenezwa na toleo jipya zaidi la Flat kuliko Trindle inavyoweza kusoma

# The reason when the file holds no apps.
store-because-no-apps = hakuna programu kwenye faili hiyo

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = nakala rudufu haikuweza kusomwa: { $why }

# The reason when the program gave none.
store-because-none = hakuna sababu iliyotolewa

# The reason when the system said permission was denied.
# SAFETY: needs a human review before it ships
store-because-permission = Trindle hairuhusiwi kuandika huko

# The reason when the disk is full.
store-because-space = hakuna nafasi iliyobaki kwenye kompyuta hii

# The reason when a part of the computer is read-only.
store-because-read-only = sehemu hiyo ya kompyuta hii ni ya kusoma tu

# The reason when the person's disk quota is used up.
store-because-quota = nafasi iliyoruhusiwa kwa akaunti hii imeisha

# The reason when a file or program is missing.
store-because-missing = faili au programu inayohitajika haipo

# The reason when no reason could be read from the program's last line.
store-because-failed = imeshindwa

# The reason when the backup has no name to be written under.
store-because-no-name = haina jina

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = mipangilio yake kwenye nakala rudufu inafika nje ya folda yake yenyewe

# The reason when something is too big to be read whole.
store-because-too-big = ni kubwa mno kusomwa

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = mipangilio yake kwenye nakala rudufu imeharibika au kubadilishwa

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Kila kitu kilirudishwa isipokuwa picha ya akaunti yako. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Trindle pia haikurudisha picha ya akaunti yako. { $why }

# Said when the pkexec program could not be started.
# SAFETY: needs a human review before it ships
store-refusal-pkexec = Haikuweza kuomba nenosiri: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Trindle haikuweza kuisoma kutoka kwenye nakala rudufu: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Trindle haikutengeneza nakala rudufu, kwa sababu { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = bado hakuna

# Beside the size of a part of the backup: about how big it is.
store-backup-about = takriban { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Inarudisha mipangilio ya Trindle na AppImage…

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Trindle ilirudisha mipangilio na data yako, lakini Flatpak haikusakinisha programu hizi: { $failed }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Trindle ilirudisha mipangilio na data yako, lakini haikusakinisha programu hizi: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] Trindle iliruka programu 1, kwa sababu jina lake halikuwa halali.
       *[other] Trindle iliruka programu { $n }, kwa sababu majina yake hayakuwa halali.
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = chanzo cha { $name } hakijasimbwa kwa njia fiche, kwa hiyo Trindle haikukitumia

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = chanzo cha { $name } hakipo kwenye kompyuta hii na nakala rudufu haisemi kiko wapi

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = mipangilio ya { $name } (kwa sababu nakala yake kwenye nakala rudufu haikupita ukaguzi)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (kwa sababu jina la chanzo chake si halali)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (kwa sababu { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (Flatpak haikuweza kuisakinisha: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = kila kitu (kwa sababu hakuna mahali pa kufungua nakala rudufu)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = ruhusa za kila programu

# Said among what could not be done: one app's settings.
store-flat-app-settings = mipangilio ya { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. Kilichokuwepo awali kimehifadhiwa kama { $path }: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Inasakinisha { $name } ({ $at } kati ya { $total })…

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Inarudisha mipangilio ya { $name } ({ $at } kati ya { $total })…

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] Imesakinisha programu { $n }.
       *[other] Imesakinisha programu { $n }.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
        [one] Mipangilio ya programu moja imerudishwa.
       *[other] Mipangilio ya programu { $n } imerudishwa.
    }

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] Programu { $n } ilikuwa imesakinishwa tayari.
       *[other] Programu { $n } zilikuwa zimesakinishwa tayari.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Zilizoachwa: { $list }

# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }: Trindle ilirudisha tu ruhusa zinazoondoa ufikiaji.
