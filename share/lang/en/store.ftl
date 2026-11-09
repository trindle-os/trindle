# Apps: the window, its side bar and the front pages (src/store/mod.rs, catalogue.rs, dropped.rs).

## The window and its side bar (src/store/mod.rs)

# Title of the Apps window, and the program's name.
store-title = Apps
# Typing box at the top of Apps, shown quietly while empty.
store-search-placeholder = Search apps
# Side bar place: the front page of apps to try.
store-nav-discover = Discover
# Side bar place: the apps that are installed.
store-nav-installed = Installed
# Side bar place: updates, when none are waiting.
store-nav-updates = Updates
# Side bar place: updates, with how many are waiting.
store-nav-updates-waiting = Updates ({ $n })
# Side bar place: plugins.
store-nav-plugins = Plugins
# Side bar place and title: the AppImages AppImageHub lists.
store-nav-browse = Browse AppImages
# Side bar place: apps from GitHub.
store-nav-github = From GitHub
# Side bar place: where software comes from.
store-nav-sources = Software sources
# Side bar place, title of the page and its way back: moving apps and settings to another computer.
store-move = Move your apps
# Small heading over the side bar's first group.
store-group-apps = Apps
# Small heading over the pages from Settings that are about apps.
store-group-settings = Settings for apps
# Small heading over the categories of apps.
store-group-categories = Categories
# Small heading over the other ways to get an app.
store-group-more = More ways to get apps
# Title of a page of search results; the words searched for are in the quotes.
store-grid-results = Results for “{ $text }”
# Title of the front page of apps, and of a page before the catalogue has come.
store-grid-discover = Discover
# Heading of the apps most people want, on the front page.
store-grid-popular = Popular
# Title of the page of apps from a source, before the catalogue has come.
store-grid-apps = Apps
# Red words under the search box when an app asked for is not in the catalogue.
store-not-in-catalogue = { $id } is not in the Flathub catalogue.

## Categories of apps (src/store/catalogue.rs)

# Side bar category: music, film and sound.
store-cat-audio-video = Sound and video
# Side bar category: drawing and photographs.
store-cat-graphics = Graphics and photos
# Side bar category: documents and spreadsheets.
store-cat-office = Office
# Side bar category: browsers and chat.
store-cat-network = Internet
# Side bar category: programming tools.
store-cat-development = Development
# Side bar category: teaching and learning.
store-cat-education = Learning
# Side bar category: games.
store-cat-game = Games
# Side bar category: science.
store-cat-science = Science
# Side bar category: small everyday tools.
store-cat-utility = Utilities
# Side bar category: tools for the computer itself.
store-cat-system = System
# Red words when the list of apps could not be read.
store-catalogue-unreadable = The catalogue could not be read: { $why }
# Red words when Flathub's list of apps could not be fetched. { $why } is what Flatpak said.
store-catalogue-not-fetched = The list of apps could not be fetched from Flathub. Check the internet connection. { $why }

## Dropping a file on the window (src/store/dropped.rs)

# Red words under the search box when nothing that was dropped on the window is a kind Apps takes.
store-dropped-only = Only .deb, .AppImage and .flatpakref files can be dropped here.

## Why an app was not updated (src/store/mod.rs)

# Red words under an app that Flatpak did not update; what Flatpak said follows.
store-not-updated-said = Not updated. Flatpak said: { $why }
# Red words under an app that Flatpak did not update and gave no reason for.
store-not-updated-none = Not updated, and Flatpak gave no reason.
