# Settings page "Default apps": which program opens each kind of file or link.

## Page (src/settings/panels/default_apps.rs)

# Name of the Settings page about default apps, in the list and as its heading.
settings-default-apps-title = Default apps
# Words a person might type into the Settings search to find the default apps page; separate them with spaces.
settings-default-apps-keywords = browser mail email files pictures music video pdf calendar open with terminal
# Introduction under the heading of the default apps page, and the words behind the page i.
settings-default-apps-help = Choose which program opens web links, emails, pictures, the calendar and so on. You choose the terminal in ~/.config/xdg-terminals.list. If you haven't chosen one, Trindle uses the first one that is installed.

## Kinds of file

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-web = Web
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-mail = Mail
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-folders = Folders
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-text = Text
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pictures = Pictures
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-music = Music
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-video = Video
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pdf = PDF documents
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-calendar = Calendar
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-web-in-sentence = web
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-mail-in-sentence = mail
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-folders-in-sentence = folders
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-text-in-sentence = text
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pictures-in-sentence = pictures
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-music-in-sentence = music
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-video-in-sentence = video
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pdf-in-sentence = PDF documents
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-calendar-in-sentence = calendar

## Cards

# Title of a card on the default apps page.
settings-default-apps-card-internet = Internet
# Words behind the i of a card on the default apps page.
settings-default-apps-card-internet-about = Choose which programs open web links, email addresses and calendar invitations.
# Title of a card on the default apps page.
settings-default-apps-card-files = Files and documents
# Words behind the i of a card on the default apps page.
settings-default-apps-card-files-about = Choose which programs open folders, plain text files and PDF documents.
# Title of a card on the default apps page.
settings-default-apps-card-media = Pictures and sound
# Words behind the i of a card on the default apps page.
settings-default-apps-card-media-about = Choose which programs open pictures, music and videos.

## Page

# Sentence at the top of the page when every kind has a program; { $web } is the program that opens web links.
settings-default-apps-all-set-web = Everything has a program to open it, and { $web } opens web links.
# Sentence at the top of the page when every kind has a program and the one for web links is not known.
settings-default-apps-all-set = Everything has a program to open it.
# Sentence at the top of the page when one kind has no program chosen; { $kind } is its name in lower case.
settings-default-apps-one-missing = Choose a program to open { $kind }.
# Sentence at the top of the page when several kinds have no program chosen; { $kinds } is their names as a list joined with "or".
settings-default-apps-many-missing = You haven't chosen a program to open { $kinds } yet.
# Small words under a kind that no installed program can open.
settings-default-apps-none-installed = No installed program opens these yet.
# First choice in a list when no program is chosen for that kind yet.
settings-default-apps-not-chosen = Not chosen yet
# Said in green under a kind's list after the choice was kept.
settings-default-apps-saved = Saved
