# Apps: lines of the lists of apps, where things came from and how long is left (src/store/updates.rs, listing.rs, look.rs, origin.rs, progress.rs, reviews.rs, key.rs).

## Where an installed thing came from (src/store/updates.rs)

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = A Flatpak file
# On an installed thing's line: it came from a .deb file.
store-from-deb = A .deb file
# On an installed thing's line: it is a plugin.
store-from-plugin = Plugin
# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }
# The kind of a pre-release version written rc.
store-version-rc = release candidate

## A tile of the list of apps (src/store/listing.rs)

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = Removing { $name }
# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Installing { $name }
# Tip of the round button on an app's tile that opens the app.
store-list-open = Open { $name }
# Tip of the round button on an app's tile that installs the app.
store-list-install = Install { $name }
# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Installed

## Tabs and times (src/store/look.rs, progress.rs)

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }
# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Show
# How long is left when it is a minute or less.
store-left-minute = about a minute left
# How long is left, in minutes.
store-left-minutes = about { $n } minutes left
# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] about 1 hour left
       *[other] about { $n } hours left
    }

## Where a download came from (src/store/origin.rs)

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Checked against HaGeZi's Threat Intelligence list
# Where a download came from: the site and the page that led to it.
store-origin-both = Downloaded from { $site }, reached from { $referrer }.
# Where a download came from: the site.
store-origin-site = Downloaded from { $site }.
# Where a download came from: the page that led to it.
store-origin-referrer = Reached from { $referrer }.
# Said when nothing tells where a download came from.
store-origin-unknown = Trindle cannot tell where this came from.
# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } { $name } is on HaGeZi's Threat Intelligence list of sites known for harmful downloads and scams. Only install this if you are sure of where it came from.
# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } The site is not on HaGeZi's Threat Intelligence list, but the list is not complete, so this does not prove it is safe.
# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } The site could not be checked, as HaGeZi's Threat Intelligence list could not be fetched.

## Reviews and the GitHub key's keyring (src/store/reviews.rs, key.rs)

# Name shown for a review whose writer gave none.
store-review-someone = Someone
# Red words when the keyring could not be reached.
store-key-unreached = The keyring could not be reached: { $why }.
# Red words when the key kept in the keyring is not text.
store-key-not-text = The key kept is not text.
# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } could not be removed: { $why }
# Red words when a file that held the key could not be read.
store-key-not-read = { $path } could not be read: { $why }
