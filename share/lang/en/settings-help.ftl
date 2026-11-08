# The Help page of Settings: its search, the cards that group the topics and the sentences at the top. The topics themselves are in settings-help-topics.ftl.

## Help page (src/settings/panels/help.rs)

# Name of the Help page in the Settings side bar and its title.
settings-help-title = Help
# Words the Settings search matches the Help page by, in lower case.
settings-help-keywords = help how guide question manual support
# Heading of the card of Help topics for starting out.
settings-help-area-start = Getting started
# Heading of the card of Help topics about apps.
settings-help-area-apps = Apps
# Heading of the card of Help topics about the screen and how Trindle looks.
settings-help-area-screen = Screen and looks
# Heading of the card of Help topics about magnifying, screen reading and voice.
settings-help-area-access = Seeing and hearing
# Heading of the card of Help topics about networks, sound and connected devices.
settings-help-area-devices = Networks, sound and devices
# Heading of the card of Help topics about updates, snapshots and security.
settings-help-area-safe = Keeping it safe
# Heading of the card of Help topics about this computer.
settings-help-area-computer = This computer
# The sentence at the top of Help before anything is typed.
settings-help-start = New to Trindle? Start with Welcome, or type what you are looking for.
# The sentence at the top of Help when a search finds nothing; typed is what was typed.
settings-help-found-none = Nothing matches “{ $typed }”. Try a shorter word.
# The sentence at the top of Help when a search finds topics; n is how many and typed is what was typed.
settings-help-found =
    { $n ->
        [one] One topic about “{ $typed }”.
       *[other] { $n } topics about “{ $typed }”.
    }
# Tip of the arrow button that closes an open Help topic; title is the topic's title.
settings-help-close = Close { $title }
# Tip of the arrow button that opens a Help topic to read; title is the topic's title.
settings-help-read = Read { $title }
# Bold line in the Reading card of Help when no topic is open.
settings-help-nothing-open = Nothing open
# Quiet line under "Nothing open" in Help.
settings-help-nothing-open-sub = Press the arrow beside a topic to read it here.
# Heading of the Help card that shows the open topics.
settings-help-reading = Reading
# Placeholder and spoken name of the typing box that searches Help.
settings-help-search-placeholder = Search help, such as “wifi”, “printer” or “shut down”
# Heading of the Help card with the search box.
settings-help-search = Search help
# Line and tip of the button that goes back to the Settings page Help was opened from; title is that page's title.
settings-help-back = Back to { $title }

# The row in the search card that opens the Help program, which holds every page and a search of them.
settings-help-program = Help in its own window
# The hint on that row's button.
settings-help-program-open = Open Help
# Red words under the line "Help in its own window" when the Help program could not be started.
settings-help-program-not-opened = Help could not be opened.
