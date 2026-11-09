# machine translated (Swahili), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Hakuna { $file } inayoweza kusomwa juu ya folda ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } haina sehemu ya [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } si ile Trindle hii inayoijua: inajua Interface={ $known }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] inahitaji Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] inahitaji Id ya herufi, tarakimu, nukta na vistari, kama org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] inahitaji Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] inahitaji Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] Summary ni mstari mmoja.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } ni maneno ya kawaida kwenye mstari mmoja, herufi { $max } kwa kiwango cha juu.
       *[other] [{ $group }] { $key } ni maneno ya kawaida kwenye mstari mmoja, herufi { $max } kwa kiwango cha juu.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
# SAFETY: needs a human review before it ships
plugins-check-permission-unknown = Ruhusa { $name } haipo: zilizopo ni { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] inahitaji Icon: faili ya SVG au PNG kwenye folda ya programu jalizi.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon inataja { $icon }, ambayo haipo kwenye folda ya programu jalizi.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] inahitaji Label, inayosemwa kielekezi kikiwa juu yake.

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] inahitaji Run, kinachoanzishwa kwa kubofya.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] inahitaji Fields, majina ya sehemu zake.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Jina la sehemu { $name } linaweza kuwa na herufi, tarakimu, vistari na mistari chini tu.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Sehemu { $name } haina sehemu ya [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] inahitaji Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices ni { $most } kwa kiwango cha juu, kila moja maneno ya kawaida ya herufi { $chars } kwa kiwango cha juu.
       *[other] [{ $group }] Choices ni { $most } kwa kiwango cha juu, kila moja maneno ya kawaida ya herufi { $chars } kwa kiwango cha juu.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom ni jina la faili kwenye nyumbani mwa programu jalizi: herufi, tarakimu, nukta, vistari na mistari chini, lisianze na nukta, bila folda.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] ni chaguo, kwa hiyo inahitaji Choices zenye mbili au zaidi, au ChoicesFrom faili.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default si mojawapo ya Choices zake.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] ni dokezo, kwa hiyo inahitaji TextFrom, jina la faili kwenye nyumbani mwa programu jalizi: herufi, tarakimu, nukta, vistari na mistari chini, lisianze na nukta, bila folda.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] ni dokezo, ambalo halina mpangilio, kwa hiyo haina Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind ni { $kind }: lazima iwe switch, text, choice au note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] ni swichi, kwa hiyo Default yake ni true au false.

# Red words when the [Background] part is given without the background permission.
# SAFETY: needs a human review before it ships
plugins-background-needs-permission = [Background] inahitaji ruhusa ya background katika [Plugin] Permissions.

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = Programu jalizi haijishikizi popote: ipe [Ring], [Settings], [Background], [Shortcut] au ruhusa ya arifa.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
# SAFETY: needs a human review before it ships
plugins-links-outside = { $path } inaunganisha nje ya folda ya programu jalizi.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run haiwezi kusomwa kama amri.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
# SAFETY: needs a human review before it ships
plugins-run-outside = [{ $group }] Run lazima itaje programu ndani ya folda ya programu jalizi, si { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run inataja { $program }, ambayo si programu iliyomo kwenye programu jalizi (haipo, au haiwezi kuendeshwa).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu ina kipengee { $n }: { $max } kwa kiwango cha juu zinaruhusiwa.
       *[other] [Ring] Menu ina vipengee { $n }: { $max } kwa kiwango cha juu vinaruhusiwa.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Id ya menyu { $id } inaweza kuwa na herufi, tarakimu, vistari na mistari chini tu.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu inataja { $id } mara mbili.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu inataja { $id }, ambayo haina sehemu ya [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label ni mstari mmoja mfupi wa herufi { $max } kwa kiwango cha juu.
       *[other] [{ $group }] Label ni mstari mmoja mfupi wa herufi { $max } kwa kiwango cha juu.
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] inahitaji Run, kinachoanzishwa kwa kuichagua.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-only-flow = [{ $group }] ni ya Flow tu kutoka orodha ya Trindle, ambayo inaomba ruhusa ya control.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-needs-run = [{ $group }] inahitaji Run, amri inayopewa PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen ni herufi { $max } kwa kiwango cha juu.
       *[other] [{ $group }] ShowWhen ni herufi { $max } kwa kiwango cha juu.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen huandikwa field=value, kama ShowWhen=mode=home: sehemu moja nyingine na thamani moja inayopaswa kuwa nayo.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen huandikwa field=value, na jina la sehemu linaweza kuwa na herufi, tarakimu, vistari na mistari chini tu.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen inahitaji thamani ya maneno ya kawaida kwenye mstari mmoja, herufi { $max } kwa kiwango cha juu.
       *[other] [{ $group }] ShowWhen inahitaji thamani ya maneno ya kawaida kwenye mstari mmoja, herufi { $max } kwa kiwango cha juu.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen inataja sehemu yake yenyewe: lazima itaje nyingine, ya awali.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen inataja sehemu { $other }, ambayo haimo kwenye Fields za programu jalizi hii.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen inataja sehemu { $other }, inayokuja baada yake kwenye Fields: lazima iwe sehemu ya awali.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen inataja sehemu { $other }, ambayo ina ShowWhen yake yenyewe: sehemu inaweza kutegemea tu ile isiyo na ShowWhen.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen inataja swichi { $other }, kwa hiyo thamani yake ni true au false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen ina thamani { $value }, ambayo si mojawapo ya Choices za { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen inataja dokezo { $other }, ambalo halina mpangilio: sehemu inaweza kutegemea swichi, chaguo au maandishi tu.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Mtandao] Kusikiliza: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = [Network] Listen inahitaji ruhusa ya network katika [Plugin] Permissions.

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = [Network] Listen inahitaji ruhusa ya listen katika [Plugin] Permissions.

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = Ruhusa ya listen inahitaji [Network] Listen, milango ya kuruhusu ndani.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = plugin.ini yake si ya interface inayojulikana na Trindle hii.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Haitaji milango ya kusikiliza.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = plugin.ini yake haikuweza kusomwa.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = Haijaruhusiwa kusikiliza.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = Milango inayotaja si ile iliyoruhusiwa. Lazima iruhusiwe tena.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none = { $name } inahitaji ruhusa yako ili ifanye kazi: 0 kati ya { $n } zimeruhusiwa.

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } ina sehemu tu ya ruhusa inazoomba: { $a } kati ya { $n } zimeruhusiwa.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] Haiombi chochote.
        [one] Kitu kimoja inachoomba kimeruhusiwa.
       *[other] Vitu vyote { $n } inavyoomba vimeruhusiwa.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = Ruhusu

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = kufikiwa kwenye milango mingine kuliko awali

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Haijawekwa.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = programu jalizi nyingine

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Programu jalizi hiyo haijawekwa.

# Red words when a permission could not be kept. { $why } is the reason.
# SAFETY: needs a human review before it ships
plugins-not-saved = Haikuweza kuhifadhiwa: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Faili za programu jalizi hazikuweza kunakiliwa.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Sehemu yake ya nyuma haifanyi kazi: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = trindle-plugin haikupatikana ili kuianzisha.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] haikuanza ndani ya sekunde { $n }.
       *[other] haikuanza ndani ya sekunde { $n }.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = hakuna kipindi cha eneo-kazi kinachoendelea cha kujiunga nacho, kwa hiyo haikuanzishwa. Ingia kwanza.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = haiwezi kujua ijiunge na kipindi kipi cha eneo-kazi ({ $sessions }), kwa hiyo haikuanzishwa. Iendeshe tena huku WAYLAND_DISPLAY ikiwa imewekwa kwa sahihi.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = folda iko mahali faili inapopaswa kuwa

# Error text when a plugin's id is not an id.
plugins-not-an-id = si id ya programu jalizi

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts ina njia ya mkato { $n }: { $max } kwa kiwango cha juu zinaruhusiwa.
       *[other] [Plugin] Shortcuts ina njia za mkato { $n }: { $max } kwa kiwango cha juu zinaruhusiwa.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = Id ya njia ya mkato { $id } inaweza kuwa na herufi, tarakimu, vistari na mistari chini tu.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts inataja { $id } mara mbili.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts inataja { $id }, ambayo haina sehemu ya [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } tayari ni njia ya mkato ya Trindle; chagua vitufe vingine.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } inatumiwa na njia nyingine ya mkato ya programu jalizi hiyo.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] inahitaji Keys, kama Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] inahitaji Run, kinachoanzishwa na vitufe.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Vitufe vilivyoandikwa kwa ajili yake si njia ya mkato. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } tayari ni njia ya mkato ya Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } tayari ni mojawapo ya njia zako za mkato.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } tayari inatumiwa na { $holder }.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Hiyo si njia ya mkato ya programu jalizi iliyowekwa.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Haikuweza kuanza kutumika ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Haikuweza kuhifadhiwa: hakuna kilichobadilishwa, kwa sababu { $why }.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Haikuweza kuhifadhiwa ({ $why }).


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Ruhusa ya ufikivu ni kwa kisoma skrini cha Trindle kutoka katalogi pekee.

