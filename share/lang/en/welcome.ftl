## The welcome tour (src/welcome): the short tour at the first login

## Pages (src/welcome/mod.rs)

# Name of the welcome window, and the title of its first page.
welcome-title = Welcome to Trindle
# Title of the second page, about opening the ring.
welcome-page-ring = Open the ring
# Title of the third page, where the first apps are picked.
welcome-page-apps = Choose your apps
# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = More to choose from
# Title of the fifth page, about plugins.
welcome-page-plugins = Plugins add more
# Title of the sixth page, about crash reports.
welcome-page-reports = Help fix Trindle
# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = The dots
# Title of the last page, with a few things worth knowing.
welcome-page-know = Good to know
# Sentence under the title of the first page.
welcome-sentence-hello = Everything you need lives in one circle, the ring.
# Sentence under the title of the second page.
welcome-sentence-ring = Double-click at the very edge of the screen, or press the Super key.
# Sentence under the title of the third page.
welcome-sentence-apps = Almost nothing is installed yet, so pick the apps you want.
# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Pick more, then install everything you chose in one click.
# Sentence under the title of the fifth page.
welcome-sentence-plugins = Plugins are small extras from Apps, and each does only what you allow.
# Sentence under the title of the sixth page.
welcome-sentence-reports = Send a report when something goes wrong, if you choose.
# Sentence under the title of the dots page.
welcome-sentence-dots = A dot means a program is using something of yours.
# Sentence under the title of the last page.
welcome-sentence-know = A few things that make Trindle different.
# Top right of every page: where the person is in the tour.
welcome-step = Step { $n } of { $total }
# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Step { $n } of { $total }. { $sentence }
# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = The Welcome has been seen. It is in All apps as Welcome; delete this file to see it again at the next login.

## Foot of every page

# Name of the round button at the foot that closes the tour.
welcome-close = Close
# Name of the round button at the foot that goes to the page before.
welcome-back = Back
# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Next
# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Install the chosen apps
# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Start using Trindle
# Foot of the apps pages while the computer is offline.
welcome-offline = Connect to the internet to install apps.
# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Try again
# Foot while apps are going in: their names.
welcome-installing = Installing { $names }.
# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Installing { $names }, { $left }.
# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } is in the ring now.
       *[other] { $names } are in the ring now.
    }
# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = The apps could not be installed. { $why }
# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak refused.
# Foot of the apps pages: the apps picked so far.
welcome-chosen = Chosen: { $names }.
# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } and { $more } more

## Hello page

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = The ring
# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = What is in it
# Name of the first thing the ring holds.
welcome-held-programs = Your programs
# Line under it.
welcome-held-programs-line = Sit round the outside.
# Name of the second thing the ring holds.
welcome-held-time = The time and the date
# Line under it.
welcome-held-time-line = Sit in the middle.
# Name of the third thing the ring holds.
welcome-held-switches = The switches
# Line under it.
welcome-held-switches-line = Sound, screen, keep awake and power.

## Ring page

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = With the pointer
# Head of the card with the picture of the Super key.
welcome-ring-keyboard = With the keyboard
# Note under the picture of the Super key.
welcome-ring-super-note = The Super key often has a Windows logo.

## Apps pages

# Head of a card of apps: web browsers.
welcome-kind-browser = Web browser
# Head of a card of apps: email programs.
welcome-kind-email = Email
# Head of a card of apps: chat programs.
welcome-kind-chat = Chat
# Head of a card of apps: office programs.
welcome-kind-office = Office
# Head of a card of apps: music and video players.
welcome-kind-media = Music and video
# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Pictures and drawing
# Head of a card of apps: notes and writing.
welcome-kind-notes = Notes and writing
# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF and documents
# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = None of these are on Flathub just now.
# Beside an app that has been installed.
welcome-app-installed = Installed
# Beside an app that is going in now.
welcome-app-installing = Installing
# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, picked
# Name of the round button on an app's row that picks it.
welcome-app-pick = Pick { $name }

## Plugins page

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = In the ring
# Head of the card that lists the plugins on offer.
welcome-plugins-card = The plugins
# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = In Apps, under Plugins.
# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Open plugins in Apps
# Said in the plugins card while there are none to show.
welcome-plugins-none = The plugins on offer are listed in Apps.

## Crash reports page

# Head of the card on the crash reports page.
welcome-reports-card = Crash reports
# The one question about crash reports.
welcome-reports-question = Help fix Trindle by sending a report when something goes wrong? You always see it first.
# Note under the question about crash reports.
welcome-reports-note = Nothing is sent without your press. You see the whole report first, and it holds no files, passwords or Wi-Fi names.
# Second note under the question about crash reports.
welcome-reports-settings = You can change this any time in Settings, Crash reports.
# The answer that turns crash reports off.
welcome-reports-no = No
# The answer that turns crash reports on.
welcome-reports-yes = Yes, ask me
# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Send a report when something goes wrong

## Good to know page

# Head of a card on the last page.
welcome-know-boxes = Apps live in their own boxes
# Line in that card.
welcome-know-boxes-line = An app asks before it reaches your files, camera or microphone.
# Head of a card on the last page.
welcome-know-updates = Security updates install themselves
# Line in that card.
welcome-know-updates-line = Every day, without asking. Other updates wait for you.
# Head of a card on the last page.
welcome-know-ring = Settings and Apps are in the ring
# Line in that card.
welcome-know-ring-line = In its middle, above the time.
# Head of a card on the last page.
welcome-know-help = Help is in Settings
# Line in that card.
welcome-know-help-line = This tour is in All apps as Welcome, whenever you want it again.

## Pictures (src/welcome/pictures.rs)

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = A small screen with nothing on it but the ring in the middle: the time in its centre, favourite apps round it.
# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = The ring open on a small screen, with the icons of a few plugins in a row in its middle, under the time, where a plugin's icon sits once it is installed.
# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Two small drawings of where dots show: on a window's tab in the ring, and at the top middle of the screen.
welcome-dots-where-tab = On a window's tab
welcome-dots-where-corner = Top middle of the screen
welcome-picture-ring = A small screen. The pointer moves to the very top edge and double-clicks, and the ring opens in the middle.
# What the screen reader says for the picture of the Super key.
welcome-picture-super = The Super key being pressed, which opens the ring.

## The question on the first page, and the pages added for it

# Title of the first page, which asks how much the person knows.
welcome-page-level = How much do you know?
# Sentence under that title.
welcome-sentence-level = Pick the nearest, and the tour leaves out what you know.
# Name of the answer for someone who has not used Linux.
welcome-level-new = New to Linux
# Line in that answer's card.
welcome-level-new-line = Every page, including what any Linux desktop does.
# Name of the answer for someone who has used Linux.
welcome-level-linux = Used Linux before
# Line in that answer's card.
welcome-level-linux-line = Only what is Trindle's own. The Linux basics are skipped.
# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Used Trindle before
# Line in that answer's card.
welcome-level-trindle-line = Closes the tour now and does not show it again.
# Title of the page about files and folders.
welcome-page-files = Your files
# Sentence under that title.
welcome-sentence-files = Everything of yours lives in your home folder.
# Title of the page about getting software.
welcome-page-software = Getting software
# Sentence under that title.
welcome-sentence-software = Apps come from Apps, not from setup files you download.
# Title of the page about the administrator password.
welcome-page-admin = The administrator password
# Sentence under that title.
welcome-sentence-admin = Changes to the whole computer ask for a password first.
# Title of the page about snapshots.
welcome-page-snapshots = Snapshots
# Sentence under that title.
welcome-sentence-snapshots = Trindle keeps copies of the system, so an update can be undone.
# Title of the page about the person's own words.
welcome-page-words = Your own words
# Sentence under that title.
welcome-sentence-words = Change any word Trindle shows, or add a language of your own.
# Head of a card on the files page.
welcome-files-home = A home folder
# Line in that card.
welcome-files-home-line = Documents, Downloads and Pictures are inside it. Folders opens there.
# Head of a card on the files page.
welcome-files-drives = No drive letters
# Line in that card.
welcome-files-drives-line = A USB stick or another disk appears in Folders when you plug it in.
# Head of a card on the files page.
welcome-files-hidden = Hidden names
# Line in that card.
welcome-files-hidden-line = A name starting with a dot, such as .config, is hidden. It holds an app's settings.
# Head of a card on the files page.
welcome-files-bin = The bin
# Line in that card.
welcome-files-bin-line = A deleted file waits in the bin until you empty it.
# Head of a card on the software page.
welcome-software-apps = One place for apps
# Line in that card.
welcome-software-apps-line = Open Apps from the ring to find, install and remove programs.
# Head of a card on the software page.
welcome-software-exe = No setup files
# Line in that card.
welcome-software-exe-line = A Windows setup file, ending in .exe, does not run here. Look for the app in Apps.
# Head of a card on the software page.
welcome-software-updates = Updates in one place
# Line in that card.
welcome-software-updates-line = Apps and Settings keep everything current, so no app needs its own updater.
# Head of a card on the software page.
welcome-software-remove = Removing is clean
# Line in that card.
welcome-software-remove-line = Removing an app takes all of it away and leaves nothing behind.
# Head of a card on the administrator password page.
welcome-admin-why = Why it asks
# Line in that card.
welcome-admin-why-line = Installing for everyone or changing a disk affects every user, so the computer asks first.
# Head of a card on the administrator password page.
welcome-admin-who = Whose password
# Line in that card.
welcome-admin-who-line = That of an administrator, usually the one you log in with.
# Head of a card on the administrator password page.
welcome-admin-not = When it does not ask
# Line in that card.
welcome-admin-not-line = Apps you install for yourself and your own files never need it.
# Head of a card on the snapshots page.
welcome-snap-taken = Taken for you
# Line in that card.
welcome-snap-taken-line = On a Btrfs system, one is taken before and after every software change.
# Head of a card on the snapshots page.
welcome-snap-list = Listed in Settings
# Line in that card.
welcome-snap-list-line = Settings, Snapshots lists them. The camera takes one now.
# Head of a card on the snapshots page.
welcome-snap-boot = Used if it will not start
# Line in that card.
welcome-snap-boot-line = Choose Trindle snapshots in the boot menu, then a snapshot.
# Head of a card on the your own words page.
welcome-words-change = Change a word
# Line in that card.
welcome-words-change-line = Find any word Trindle shows and type your own. "Open it, brew" can replace "Click here to open the file".
# Head of a card on the your own words page.
welcome-words-share = Share a sheet
# Line in that card.
welcome-words-share-line = Export your words as a file, or import one a friend sent. It is checked before it is used.
# Head of a card on the your own words page.
welcome-words-all = Every program at once
# Line in that card.
welcome-words-all-line = All of Trindle reads its words from one place, so one change shows everywhere.
# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = It is in Settings, Region and language. Help explains it step by step.
# Name of the round button that opens that Settings page.
welcome-words-open = Open Region and language
