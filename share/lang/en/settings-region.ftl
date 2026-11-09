# Settings: the Region and language page (src/settings/panels/region.rs).

## The Region and language page (src/settings/panels/region.rs)

# Name of the Region and language page, in the page list and as its heading.
settings-region-title = Region and language
# Words people may search for to find the Region and language page, separated by spaces.
settings-region-keywords = language locale country formats date currency
# Help line at the foot of the Region and language page.
settings-region-help = Language and formats take effect the next time you log in.
# Said beside the Language and Formats lists once a choice is saved.
settings-region-later = Log out and in to see it.
# Shown while the page reads the computer.
settings-region-looking = Looking…
# Under Money and paper: the paper and measures the chosen formats use (paper is a4, letter or other; measures is imperial or metric).
settings-region-paper-measures =
    { $paper ->
        [a4] A4 paper
        [letter] Letter paper
       *[other] paper
    }, { $measures ->
        [imperial] inches and pounds
       *[other] metric measures
    }
# Under Money and paper: { $money } is the money sign and { $paper } the paper and measures.
settings-region-money-and-paper = { $money }, { $paper }
# Under A date and a number: a date and a number written the way the formats chosen write them.
settings-region-date-and-number = { $date } and { $number }
# Said beside the Add a language line once a language is installed.
settings-region-installed-done = Installed
# Headline when the language and the formats are the same: { $lang } is the language's name.
settings-region-everything-in = Everything is in { $lang }.
# Headline when the formats differ from the language: the names are { $lang } and { $fmt }.
settings-region-menus-in = Menus are in { $lang }; dates and numbers are written as in { $fmt }.
# Headline when a change is saved but waits for the next login: { $words } is the sentence so far.
settings-region-with-wait = { $words } Log out and in to see the change.
# Said beside the Install another line while a language is made.
settings-region-installing = Installing
# Name of the list of languages for menus and messages, heading of its line and of the card.
settings-region-language = Language
# Name of the list of formats and heading of its line.
settings-region-formats = Formats
# Help text of the Language card (the information button).
settings-region-language-about = The language is what menus and messages are in. The formats are how dates, times, numbers, money, measurements and paper size are written, and may follow another country's ways. Both take effect the next time you log in.
# Under the Language heading.
settings-region-language-sub = Menus and messages, from the next login.
# Under the Formats heading.
settings-region-formats-sub = Dates, numbers, money and paper size.
# Heading of the line that shows a date and a number in the chosen formats.
settings-region-date-number = A date and a number
# Heading of the line that shows the money and paper of the chosen formats.
settings-region-money-paper = Money and paper
# Heading of the line that shows the language now in use.
settings-region-in-use = In use now
# Heading of the card for installing another language.
settings-region-add = Add a language
# Help text of the Add a language card (the information button).
settings-region-add-about = Installs another language, so it can be chosen for the menus or the formats. Making one takes a little while and needs an administrator's password.
# Heading of the line shown when every language is already installed.
settings-region-all-installed = Every language is already installed
# Heading of the line saying how many languages are installed.
settings-region-installed = Installed
# Under the Installed heading: how many languages are installed.
settings-region-count =
    { $n ->
        [one] { $n } language installed.
       *[other] { $n } languages installed.
    }
# Name of the list of languages that can be installed, and heading of its line.
settings-region-install-another = Install another
# Tip of the button that installs the chosen language.
settings-region-install-this = Install this language (asks for your password)
# Under the Install another heading.
settings-region-needs-password = Needs an administrator's password.
# Heading of the line saying how many more languages can be made.
settings-region-ready = Ready to add
# Under the Ready to add heading: how many more languages can be made.
settings-region-more =
    { $n ->
        [one] { $n } more language this computer can make.
       *[other] { $n } more languages this computer can make.
    }
settings-region-removing = Removing
settings-region-removed-done = Removed
settings-region-in-use-stays = In use, so it stays
settings-region-remove-language = Remove this language
settings-region-kept = Languages on this computer
settings-region-kept-about = Every language installed here. Take away any you do not use; the one you are using stays. Asks for the administrator password.
