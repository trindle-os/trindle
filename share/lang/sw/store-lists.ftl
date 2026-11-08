# machine translated (Swahili), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Faili ya Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = Faili ya .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = Programu jalizi

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = toleo la majaribio ya mwisho

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = Inaondoa { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Inasakinisha { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = Fungua { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = Sakinisha { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Imesakinishwa

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Onyesha

# How long is left when it is a minute or less.
store-left-minute = imebaki takriban dakika moja

# How long is left, in minutes.
store-left-minutes = imebaki takriban dakika { $n }

# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] imebaki takriban saa 1
       *[other] imebaki takriban saa { $n }
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Imekaguliwa dhidi ya orodha ya Threat Intelligence ya HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Imepakuliwa kutoka { $site }, kwa kupitia { $referrer }.

# Where a download came from: the site.
store-origin-site = Imepakuliwa kutoka { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Imefikiwa kupitia { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle haiwezi kujua hii ilitoka wapi.

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } iko kwenye orodha ya Threat Intelligence ya HaGeZi ya tovuti zinazojulikana kwa upakuaji hatari na ulaghai. Isakinishe tu ikiwa una uhakika ilitoka wapi.

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } Tovuti hii haiko kwenye orodha ya Threat Intelligence ya HaGeZi, lakini orodha hiyo haijakamilika, kwa hiyo hili halithibitishi kuwa ni salama.

# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } Tovuti haikuweza kukaguliwa, kwa sababu orodha ya Threat Intelligence ya HaGeZi haikuweza kupatikana.

# Name shown for a review whose writer gave none.
store-review-someone = Mtu fulani

# Red words when the keyring could not be reached.
store-key-unreached = Hifadhi ya funguo haikuweza kufikiwa: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = Ufunguo uliohifadhiwa si maandishi.

# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } haikuweza kuondolewa: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = { $path } haikuweza kusomwa: { $why }
