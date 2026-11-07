# Apps: the Updates page (src/store/update_page/).

## The Updates page (src/store/update_page/)

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian's packages
# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Held back at version { $have }; { $waits }
# On a row held back: the one that waits.
store-upd-held-waits = Held back; { $waits }
# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Held back at version { $have }
# On a thing held back with no update waiting.
store-upd-held = Held back
# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Held back at version { $have }; nothing newer yet
# On the Held back card: nothing newer waits.
store-upd-held-nothing = Held back; nothing newer yet
# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 update
       *[other] { $n } updates
    }
# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] 1 update is ready
       *[other] { $n } updates are ready
    }
# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, { $size } in all
# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 is held back
       *[other] { $n } are held back
    }
# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }
# A line made into a sentence.
store-upd-sentence = { $text }.
# Two sentences one after the other.
store-upd-and-then = { $first } { $second }
# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = The firmware needs a restart to finish.
# Headline while the check runs.
store-upd-checking = Checking for updates.
# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Updating { $name }. You can keep working while it goes in.
# Headline while all the updates go in.
store-upd-installing-all = Installing the updates. You can keep working while they go in.
# Headline when the updates are in and the computer must start again.
store-upd-restart = Restart to finish: the updates are in, and go into use when the computer starts again.
# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = What has gone in needs a restart to finish.
# Added to the headline with no internet.
store-upd-no-more = This computer is not connected to the internet, so no more can be looked for.
# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = No updates are ready
# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = No updates are waiting that this computer knows of. It cannot look for more while it is not connected to the internet.
# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = the apps
# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle and Debian
# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = the firmware
# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Updates could not be checked for { $names }.
# Headline when everything is up to date.
store-upd-current = Everything is up to date.
# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Last checked at { $time }.
# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Checking for updates by itself is off. Press Check for updates to look.
# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Not checked
# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Cannot be checked without the internet
# Red words under a card when a check took too long.
store-upd-too-long = It took too long to answer.
# Tip of the round button that checks again.
store-upd-check-again = Check again
# Main button while the check runs.
store-upd-checking-main = Checking
# Main button while the updates go in.
store-upd-installing-main = Installing
# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] Install 1 update
       *[other] Install { $n } updates
    }
# Main button that restarts the computer.
store-upd-restart-now = Restart now
# Main button that checks for updates.
store-upd-check-for = Check for updates
# Tip of the round button that releases a thing held back.
store-upd-release = Release { $name }
# Tip of the round button that holds a thing's updates back.
store-upd-hold = Hold back { $name }
# Tip of the round button that ends an update that is running.
store-upd-stop = Stop
# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Stopping
# Under a row whose update was stopped.
store-upd-stopped = Stopped. Press Update to carry on.
# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Stopped. Press Install to carry on.
# A card's state while it is being checked.
store-upd-status-checking = Checking
# A card's state while its updates go in.
store-upd-status-installing = Installing
# A card's state when its check failed; the reason follows.
store-upd-status-failed = Could not check. { $why }
# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }
# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } held back
# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } held back
# A card's state when nothing waits.
store-upd-up-to-date = Up to date
# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = From { $from }, { $change }
# On a row of the Updates page: where an app comes from.
store-upd-from = From { $from }
# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = A shared part, from { $from }
# Note under the Apps card.
store-upd-steam-restarts = Steam restarts once no game is running.
store-upd-yours-alone = Yours alone, so they update without a password. An app brings the shared parts its new version needs.
# Note under the Apps card: what holding back does.
store-upd-hold-why = An update held back stays at the version it has, and out of Install all here and on the ring, until you release it.
# Heading of the Apps card.
store-upd-card-apps = Apps
# Plugins card: every plugin is up to date.
store-upd-plugins-current = Every plugin is up to date.
# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] 1 plugin's update is held back.
       *[other] { $n } plugins' updates are held back.
    }
# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 plugin has an update
       *[other] { $n } plugins have an update
    }
# Heading of the Plugins card.
store-upd-card-plugins = Plugins
# Held back card: nothing is held back.
store-upd-nothing-held = Nothing is held back.
# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Released
# Note under the Held back card: what releasing does.
store-upd-release-why = A thing released goes in with the rest again, from Install all here and on the ring.
# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = A device's firmware
# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = the firmware of device { $id }
# Heading of the Held back card.
store-upd-card-held = Held back
# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } to { $to }
# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 update, which goes in by itself
       *[other] { $n } updates, which go in together
    }
# On a row of Debian's updates: where it comes from.
store-upd-from-debian = From Debian
# Under Debian's rows: how many of them show.
store-upd-debian-showing = Showing { $shown } of { $total } of Debian's updates.
# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = This computer has Trindle { $version }.
# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle and Debian
# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Debian's packages are made to fit each other, and one put in alone can bring others with it or leave them out of step. So they go in together, from one button.
# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Trindle's updates and Debian's bring security fixes for the whole computer, so they cannot be held back.
# Heading of the card about the daily security updates.
store-upd-card-security = Security updates
# On the security card: its state.
store-upd-security-on = On, every day
# On the security card.
store-upd-security-note = Debian's fixes go in each day without asking.
# On the security card.
store-upd-security-part = Part of Trindle; it cannot be turned off.
# On the security card: that holding back never stops these.
store-upd-never-held = Holding an update back never stops these.
# Firmware card: the computer has no updater.
store-upd-fw-none = This computer has no firmware updater.
# Firmware card: restart to finish.
store-upd-fw-restart = Restart to finish
# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Version { $version }
# Note on the firmware card when the updater is there.
store-upd-fw-note = New firmware goes in when the computer restarts.
# Note on the firmware card when the updater is missing.
store-upd-fw-install = Install fwupd to keep the firmware up to date.
# Heading of the Firmware card.
store-upd-card-firmware = Firmware
# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd could not fetch the newest list.
# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub could not be reached.
# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Debian's lists could not be fetched.
# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Debian's lists could not be fetched. { $why }
# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Not held back. The list of what is held back could not be kept: { $why }.
# The same, for a release.
store-upd-not-released-list = Not released. The list of what is held back could not be kept: { $why }.
# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Held back in Apps, but Flatpak itself could not be told, so something else that updates apps could still put it in.
# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Released in Apps, but Flatpak itself could not be told and may still leave it out. Hold it back and release it again.
# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak said: { $why }
# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle
