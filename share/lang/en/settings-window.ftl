# The Settings window itself: its title, the side bar with its search box and group headings, and the words printed by its command line options. Each page's own words are in the settings-<page>.ftl files.

## Window (src/settings/mod.rs)

# Title of the Settings window, spoken by a screen reader and shown in the window list.
settings-window-title = Settings
# Typing box at the top of the Settings side bar; it filters the pages.
settings-window-search = Search settings
# Small grey heading over the side bar's first group of pages: how it looks and sounds.
settings-window-group-personal = Personal
# Small grey heading over the side bar's group of pages for the computer's parts.
settings-window-group-devices = Devices
# Small grey heading over the side bar's group of pages for networks.
settings-window-group-network = Network
# Small grey heading over the side bar's group of pages for the system.
settings-window-group-system = System
# Small grey heading over the side bar's last group of pages: about Trindle, thanks and help.
settings-window-group-about = About
# Small grey heading over pages a search finds that are in the Apps program.
settings-window-in-apps = In Apps
# Small grey heading over the row a search finds that opens the Folders program's settings.
settings-window-in-folders = In Folders
# The side bar row that opens the Folders program at its own settings.
settings-window-in-folders-name = Folders' own settings
# Words the Settings search matches the Folders settings row by, in lower case, separated by spaces.
settings-window-in-folders-keywords =
    folders files file manager start pane home recent drives usb stick sd card plugged in open eject list endings extensions names tick boxes checkboxes picking selecting rows compact normal roomy
# Quiet line in the side bar when a search finds no page.
settings-window-no-page = No page matches. Try a shorter word, or look in Help.

## Command line (src/settings/mod.rs)

# Printed in a terminal by trindle-settings --themes for a theme that cannot be used; id is its short name, name its full name, why the reason.
settings-window-theme-refused = {"  "}{ $id }  { $name } (cannot be used: { $why })
# Printed in a terminal when trindle-settings --theme is given no theme name.
settings-window-theme-name = Name a theme, such as: trindle-settings --theme default
# Printed in a terminal after trindle-settings --theme worked; theme is the theme's name.
settings-window-theme-now = The theme is now { $theme }.
# The name printed for the built-in theme in the line above.
settings-window-theme-own = Trindle's own
# Printed in a terminal when the theme could not be chosen; error says why and ends with a full stop.
settings-window-themes-list = { $error } trindle-settings --themes lists them.
# Printed in a terminal when trindle-settings --keys is not given the word default.
settings-window-keys-how = To put every key and button back: trindle-settings --keys default
# Printed in a terminal after every key and button was put back.
settings-window-keys-back = Every key and button is back as it is printed.
# Printed in a terminal when the keys could not be put back; error says why.
settings-window-keys-failed = The keys could not be put back: { $error }
