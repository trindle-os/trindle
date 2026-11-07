# Apps: the pages of apps, the installed list, sources and moving apps (src/store/pages/).

## Pages of apps (src/store/pages/)

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } app
       *[other] { $count } apps
    }
# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } update is waiting
       *[other] { $count } updates are waiting
    }
# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } to choose from on Flathub; { $more }.
# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } to choose from on Flathub and AppImageHub; { $more }.
# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } to choose from on AppImageHub.
# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = checking for updates is off
# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = everything of yours is up to date
# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.
# Line under the title of a search that found nothing.
store-grid-none-found = No apps match “{ $text }”.
# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT returns once OpenAI signs its download.
# Line under the title of an empty page of apps.
store-grid-none-here = There are no apps here yet.
# Advice in the empty page of a search.
store-grid-try-shorter = Try a shorter word, or look through a kind on the left.
# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Search by name above, or look through another kind on the left.
# Under a page of apps: how many are shown of how many there are.
store-grid-showing = Showing { $shown } of { $apps }.
# Link under a page of apps that shows more of them.
store-grid-show-more = Show { $next } more
# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Fetching the list of apps from Flathub. The first time takes a minute.
# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Apps could not reach Flathub.
# Under the title of the waiting page when the computer is not connected.
store-wait-offline = This computer is not connected to the internet. Apps tries again by itself as soon as it is.
# In the waiting page when the computer is not connected.
store-wait-comes-from = The apps to choose from come from Flathub, over the internet.
# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } and the settings for apps, on the left, work without it.
# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } and the settings for apps, on the left, work without Flathub.
# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } and the settings for apps, on the left, work without the internet.
# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Try again
# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Results
# Red words under the search box when no web browser opens a link.
store-no-browser = There is no web browser to open { $url }. Get one in Apps first.

## The Installed page (src/store/pages/)

# Said in the card or under the title while the lists are being read.
store-looking = Looking…
# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = It is in Installed now, and in the ring
# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Removed
# The word on an app's dial when installing or removing did not work.
store-done-not = Not done
# The word on an app's dial when it was installed.
store-done-installed = Installed
# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak refused.
# Tab of the Installed page that shows everything.
store-tab-all = All
# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak files
# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImages
# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb files
# Tab of the Installed page that shows plugins.
store-tab-plugins = Plugins
# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Nothing is installed yet. Discover is full of things to try.
# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 plugin is installed
       *[other] { $n } plugins are installed
    }
# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 program is installed
       *[other] { $n } programs are installed
    }
# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } things are installed
# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, from Flathub
       *[other] { $start }, all from Flathub
    }
# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, from a Flatpak file
       *[other] { $start }, all from Flatpak files
    }
# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, an AppImage
       *[other] { $start }, all AppImages
    }
# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, from a .deb file
       *[other] { $start }, all from .deb files
    }
# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }
# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }
# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } from Flathub
# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 from a Flatpak file
       *[other] { $n } from Flatpak files
    }
# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
       *[other] { $n } AppImages
    }
# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 app
       *[other] { $n } apps
    }
# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 from a .deb file
       *[other] { $n } from .deb files
    }
# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 plugin
       *[other] { $n } plugins
    }
# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }
# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = It is up to date.
# End of the Installed line when everything is up to date.
store-installed-state-all = All are up to date.
# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 has an update waiting.
       *[other] { $n } have updates waiting.
    }
# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } of { $total } programs match “{ $find }”.
# The same when plugins are among the things.
store-installed-match-things = { $shown } of { $total } things match “{ $find }”.
# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }
# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Add an AppImage
# Placeholder of the typing box on the Installed page.
store-installed-search = Search what is installed
# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = The AppImage could not be added.
# In the card of the Installed page when nothing is installed.
store-installed-empty = Apps you install from Discover, from GitHub or as AppImages, programs from .deb files and plugins appear here.
# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Nothing installed matches “{ $find }”.
# Note at the foot of the Installed list.
store-installed-remove-twice = Remove asks twice: press it again within three seconds.
# On an installed thing's line: its version.
store-installed-version = version { $version }
# On an installed thing's line: its updates are held back.
store-installed-held = held back
# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = held back; { $waits }
# Tip and spoken name of the button while an app is updating.
store-installed-updating = Updating { $name }
# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Update { $name }
# Tip of the round button that shows an installed app's page.
store-installed-about = About
# Under an installed thing's row while it is being taken off.
store-installed-removing = Removing...

## Adding an AppImage or a Flatpak file (src/store/pages/)

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = There is no session bus to ask which file.
# Title of the window that asks which AppImage to add.
store-add-choose-title = Choose an AppImage
# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImages
# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = The file chooser could not be asked.
# Words in the Installed card while an AppImage is being added.
store-add-adding = Adding...
# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } is already there: see Installed.
# Words once an AppImage is in the ring.
store-add-in-ring = Added. It is in your list now.
# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = That is not a Flatpak reference this computer can use.
# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = Installing { $name } from { $host }...
# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } is installed.
# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak would not install it.
# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } could not be installed. { $why }
# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Replacing...
# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } is already there
# Small heading over the AppImage being brought.
store-replace-new = The one being brought
# Small heading over the AppImage already there.
store-replace-old = The one already there
# Tip of the round button that keeps the AppImage already there.
store-replace-skip = Skip: keep the one already there
# Button that replaces the AppImage already there with the new one.
store-replace-do = Replace
# Heading of the card that asks what to do with a Flatpak being removed.
store-flatpak-ask-head = Remove { $name }?
# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Its saved data takes { $size }.
# Under that heading when the app has saved no data.
store-flatpak-ask-none = It has no saved data.
# Under that heading when the folder of the saved data cannot be read, so its size is not known.
store-flatpak-ask-uncounted = Its saved data could not be counted.
# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Counting its saved data...
# Under the choices: the two that delete data cannot be undone.
store-flatpak-ask-undone = Deleted data cannot be brought back.
# Button: remove the app and leave the data it saved.
store-flatpak-keep = Remove, keep its data
# Button (red): remove the app and delete its saved data.
store-flatpak-wipe = Remove and delete its data
# Button (red): delete only the saved data and keep the app.
store-flatpak-data-only = Delete its data only
# Why Delete its data only is greyed.
store-flatpak-data-only-none = It has no saved data to delete.
# Tip of the round button that shuts the card with nothing done.
store-flatpak-cancel = Cancel: keep both
# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = Its saved data is deleted.
# Words while only the data is being deleted.
store-flatpak-data-going = Deleting its saved data...
# Red words under a button when a job ended and gave no reason.
store-job-no-reason = It did not finish, and gave no reason.
# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Show where the problem is

## The GitHub page (src/store/pages/)

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Search GitHub: type, then press Enter
# Heading of the card of matches from AppImageHub.
store-gh-hub = On AppImageHub ({ $n })
# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Press Enter to search the whole of GitHub as well.
# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub was not asked: there is no internet to ask it over.
# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Asking GitHub...
# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = Nothing more on GitHub.
# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] Looking for AppImage and Flatpak files in { $n } project...
       *[other] Looking for AppImage and Flatpak files in { $n } projects...
    }
# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = None of the matches on GitHub has an AppImage or Flatpak file for this computer.
# Heading of the card of matches from GitHub.
store-gh-on = On GitHub
# Heading of the card that holds the search box.
store-gh-search-card = Search GitHub
# Line under the title of the GitHub page.
store-gh-line = Apps on GitHub as AppImage or Flatpak files. Nobody checks these: install only from people you trust.
# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = GitHub cannot be searched while this computer is not connected to the internet.

## A project on GitHub and the GitHub key (src/store/pages/)

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak and AppImage
# Tip of the round button that shows or hides a project's files.
store-gh-see-files = See the files
# Button that installs a GitHub project or file.
store-gh-install = Install
# Link to a project's page on GitHub.
store-gh-open = Open on GitHub
# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, release { $tag }
# Name of the GitHub key's button and of its box.
store-gh-key = GitHub key
# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = GitHub key: one is stored
# In the GitHub key's box: what the key is for.
store-gh-key-words = You almost certainly do not need one. Without a key GitHub answers 60 questions an hour from this computer; with one it answers 5,000. The key only raises that limit. It is used to look at public files and nothing else: it gives Trindle no way into your account, and you tick no permissions when you make it. It is kept on this computer, in your keyring, and sent to GitHub alone.
# In the GitHub key's box: how to make a key.
store-gh-key-how = Make a key on GitHub, give it any name, leave every permission alone, press Generate token, then paste what it shows here.
# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = Could not remove it: { $why }
# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = Could not keep it: { $why }
# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = Leave empty to keep it
# Placeholder of the GitHub key's box.
store-gh-key-paste = Paste the key
# Tip of the round button that removes the stored GitHub key.
store-gh-key-remove = Remove the key
# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = Make a key on GitHub
# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = Keep the key
# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Nothing in its newest release installs here.

## The Move your apps page (src/store/pages/)

# Word on the file's dial and under a GitHub file when it was added.
store-added = Added
# Line under the title of the Move your apps page.
store-bk-line = Your apps and settings, in one file to take anywhere.
# Heading of the card that makes a backup.
store-bk-card = Back up
# Tip of the round button while the backup is being made.
store-bk-making-tip = Making the backup
# Main button that makes the backup.
store-bk-make = Make the backup
# Tip of the round button that shows the backup in Folders.
store-bk-show = Show it in Folders
# Name of the switch for the list of apps in a backup.
store-bk-apps = Apps
# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] The list of your 1 Flatpak app.
       *[other] The list of your { $n } Flatpak apps.
    }
# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } The other computer installs them fresh from Flathub, so the file stays small.
# Name of the switch for the apps' own data in a backup.
store-bk-data = Apps' own data
# Under the Apps' own data switch.
store-bk-data-sub = Each app's settings and saved work, without its caches.
# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImages
# Under the AppImages switch.
store-bk-appimages-sub = Kept whole, as they are single files.
# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle settings
# Under the Trindle settings switch.
store-bk-settings-sub = The ring, wallpaper choice, colours, keyboard, power, default apps and your picture.
# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Now { $size }.
# Name of the row that says where the backup goes.
store-bk-where = Where it goes
# Under that row: the folder and the file's name.
store-bk-where-sub = Your home folder, as { $name }
# Heading of the card that puts a backup back.
store-bk-back-card = Put a backup back
# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Choose a backup file
# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = A Trindle backup, or one made by { $flat }.
# Note in the card that puts a backup back.
store-bk-back-note = Its apps are installed and its data and settings put back. Your documents and pictures are not in it: copy those over yourself.
# Said while a backup is being made.
store-bk-making = Backing up... this can take a few minutes.
# Said when a backup is made: the file and its size.
store-bk-made = It is { $path } ({ $size }). Copy it to a USB stick or cloud storage to take it with you.
# Red words when no connection to the file chooser exists.
store-bk-no-bus = No backup was chosen, because Trindle has no connection to the file chooser.
# Title of the window that asks which backup to use.
store-bk-choose-title = Choose a backup
# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle and Flat backups
# Red words when the file chooser gave no answer.
store-bk-no-answer = No backup was chosen, because the file chooser did not answer.
# Said while a backup file is read.
store-bk-reading = Reading the backup...
# Said when a backup was put back.
store-bk-logout = Log out and back in to see every setting.
# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Log out and back in to see every setting.
# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 app, { $with } with their settings. Starting...
       *[other] { $apps } apps, { $with } with their settings. Starting...
    }

## The Software sources page (src/store/pages/)

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] No makers' own sources added yet.
        [one] 1 maker's source added, kept up to date with Debian's updates.
       *[other] { $n } makers' sources added, kept up to date with Debian's updates.
    }
# Under a maker's source that is added and installed.
store-src-added = { $what }. Added, and kept up to date.
# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. The source is added, but the program is not installed.
# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Installed another way.
# Tip of the round button that removes a maker's source and its program.
store-src-remove-both = Remove it and its source
# Link to a maker's page, naming the program.
store-src-open = Open { $name }
# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Sign in to Tailscale
# Tip of the round button that adds a maker's source, naming the program.
store-src-add = Add { $name }
# Heading of the card of makers' sources that can be added.
store-src-ready = Ready to add
# Name of the field for the address of a source.
store-src-address = Address
# Under the address field.
store-src-address-sub = The repository's address, from its maker's install instructions.
# Name of the field for the suite of a source.
store-src-suite = Suite
# Under the suite field.
store-src-suite-sub = Usually trixie or stable.
# Name of the field for the components of a source.
store-src-components = Components
# Under the components field.
store-src-components-sub = Usually main.
# Name of the field for a source's signing key.
store-src-key = Signing key
# Under the signing key field.
store-src-key-sub = The address of its signing key.
# Name of the field for the program to install from a source.
store-src-program = Program to install
# Under the program field.
store-src-program-sub = Its package name, if you want it installed now.
# Placeholder of the program field.
store-src-program-hint = optional
# Tip of the round button that adds the source typed.
store-src-add-this = Add this source
# Heading of the card that adds another source.
store-src-another = Add another source
# Note in that card.
store-src-another-note = A source can install updates for the whole computer, so add only makers you trust. Each asks for an administrator's password.
# Tip of the round button that removes a source that is already on the computer.
store-src-remove-this = Remove this source
# Heading of the card of sources already on the computer.
store-src-already = Already on this computer
# Red words when the addresses of a new source are not secure.
store-src-bad-address = Both addresses must start with https://
# Red words when the suite or components are not plain words.
store-src-bad-words = The suite and components must be plain words.
# Red words when the program is not a package name.
store-src-bad-package = That is not a package name.
# Word on a source's dial while its job starts.
store-src-starting = Starting
# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Opening Tailscale's sign-in page...
# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Sign in on the page that has opened; this finishes by itself.
# Said under Tailscale's row when it is connected.
store-src-connected = Connected

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
store-bk-http-line = The link to the apps is not encrypted.
# Title of the card shown before a restore from an http address over the internet.
store-bk-http-title = Not encrypted
# Words of that card.
store-bk-http-card = This address is not encrypted. Someone on the way could see what is sent, and could swap the backup for a different one.
# Label of the tick box on that card; the restore button works only once it is on.
store-bk-http-tick = I understand and accept this risk
# Button on that card that restores the backup.
store-bk-http-go = Restore
# Button on that card that gives the restore up.
store-bk-http-cancel = Cancel
# Said before every Add of an AppImage from AppImageHub, in plain ink: what is and is not checked, and how Trindle boxes it.
store-hub-warning = AppImageHub only lists projects: nobody reviews or scans them, and they need not be signed. Trindle runs each AppImage in a box with a private home folder: at first it may use the internet and sound, and sees none of your files.
# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = Where an AppImage comes from is a hint, not a check.
# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = On GitHub: { $repo }
# The same, with how many stars the project has.
store-from-github-stars = On GitHub: { $repo }, { $stars } stars
# Where an AppImage comes from: a website, by its name.
store-from-site = Not on GitHub: from a website, { $site }
# Where an AppImage comes from, for one added by hand.
store-from-unknown = Source not known
# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Open its website
# When an app was last updated.
store-age-updated = Last updated { $date }
# The same, said in red, when that is more than six months ago.
store-age-updated-old = Last updated { $date }. Not updated for more than six months, so it may have unfixed security faults.
# When the source does not say when an app was last updated.
store-age-updated-unknown = Last updated: not known
# When an app's project was made.
store-age-created = Created { $date }
# When the source does not say when an app's project was made.
store-age-created-unknown = Created: not known
# A date the source does not give, on a line of Facts.
store-age-not-known = Not known
# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Before you add one
# Title of the card that lists the AppImages.
store-browse-list = AppImages
# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] { $count } AppImage is listed on AppImageHub.
       *[other] { $count } AppImages are listed on AppImageHub.
    }
# The same while AppImageHub's list is being fetched.
store-browse-fetching = Fetching AppImageHub's list…
# The same when the list could not be fetched.
store-browse-failed = AppImageHub's list could not be fetched.
# The same with no internet and no copy kept.
store-browse-offline = AppImageHub's list cannot be fetched while this computer is not connected to the internet.
# In the list when the kind chosen has none.
store-browse-none = AppImageHub lists none of this kind.
# Under the rows: how many are shown of how many.
store-browse-showing = Showing { $shown } of { $n }.
# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Add to Trindle's apps
# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = See details and files
# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Both
# The same: only Flatpaks.
store-show-flatpaks = Flatpaks
# The same: only AppImages.
store-show-appimages = AppImages
# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }
# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage
# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImages are not checked. Nobody reviews or scans them.
# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle runs each AppImage in a box with a private home folder: at first it may use the internet and sound, and sees none of your files.
# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Don't show again
# Button on that card that goes on with the install.
store-appimage-warn-ok = OK
# Button on that card that gives the install up.
store-appimage-warn-cancel = Cancel
