# Apps: the page for one app (src/store/app_page.rs).

## The page of one app (src/store/app_page.rs)

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.
# Under the app's name when only the maker is known.
store-app-line-by = By { $by }.
# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, by { $by }.
# The state beside the dot while the app is being taken off.
store-app-state-removing = Removing
# The state beside the dot while the app is being fetched.
store-app-state-downloading = Downloading
# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = Putting it in
# The state beside the dot while the app's own update goes in.
store-app-state-updating = Updating
# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = Installed, held back
# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = Installed, held back at version { $version }
# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = Installed, { $waits }
# The same, with the installed version.
store-app-state-waiting-version = Installed, version { $version }, { $waits }
# The state beside the dot for an installed app that is up to date.
store-app-state-current = Installed, up to date
# The same, with the installed version.
store-app-state-current-version = Installed, version { $version }, up to date
# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = Not installed
# The same, with the size of the download.
store-app-state-absent-size = Not installed, { $size } to download
# Word beside the spinner while the app is being taken off.
store-app-busy-removing = Removing
# Word beside the spinner while the app is being put in.
store-app-busy-installing = Installing
# Tip and spoken name of the round button that gives up a download.
store-app-stop = Stop
# Tip and spoken name of the round button that takes the app off.
store-app-remove = Remove
# Round button beside Update or Release that opens the app.
store-app-open-round = Open
# Main button that releases an app whose updates are held back.
store-app-release = Release
# Main button that updates the app.
store-app-update = Update
# Main button that opens the app.
store-app-open = Open
# Main button that installs the app.
store-app-install = Install
# Button that opens the app's website.
store-app-website = Website
# In the picture frame when the app has no pictures.
store-app-no-pictures = This app has no pictures.
# In the picture frame when the picture could not be fetched.
store-app-picture-missed = This picture could not be fetched.
# In the picture frame while the picture is being fetched.
store-app-picture-fetching = Fetching the picture…
# In the picture frame when the computer has no internet.
store-app-picture-offline = Pictures cannot be fetched while this computer is not connected to the internet.
# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = Picture { $at } of { $n }
# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = Picture { $at } of { $n }: { $caption }
# Tip of the round button that shows the picture before.
store-app-picture-earlier = Earlier picture
# Tip of the round button that shows the picture after.
store-app-picture-later = Later picture
# Licence shown in Facts for an app that is not open source.
store-app-proprietary = Proprietary
# Name of the Facts card.
store-app-facts = Facts
# Facts line: the size of the download.
store-app-fact-download = Download size
# Facts line: where the app comes from.
store-app-fact-source = Source
# Facts line: the version.
store-app-fact-version = Version
# Facts line: the licence of an open source app.
store-app-fact-licence = Licence
# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = Licence, not open source
# Facts line: who made the app.
store-app-fact-made-by = Made by
# Facts line: when the app's project was made.
store-app-fact-created = Created
# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  from { $n ->
        [one] { $count } rating
       *[other] { $count } ratings
    }
# Under the stars when nobody has rated the app.
store-app-rating-none = No ratings yet.
# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = No ratings to show.
# Under the stars while the ratings are being fetched.
store-app-rating-looking = Looking for ratings…
# Last line of the Rating card: where the ratings come from.
store-app-rating-source = From GNOME's open review service.
# Last line of the Rating card when there is no internet.
store-app-rating-offline = This computer is not connected to the internet.
# Name of the Rating card.
store-app-rating = Rating
# Text of the About card when the app says nothing of itself.
store-app-no-description = This app has no description.
# Name of the About card.
store-app-about = About
# Heading of a review that has no title.
store-app-review-no-title = No title
# Under What people say when nobody has written.
store-app-people-none = Nobody has written about this app yet.
# Under What people say when there is no internet.
store-app-people-offline = What people say cannot be fetched while this computer is not connected to the internet.
# Under What people say while reviews are being fetched.
store-app-people-looking = Looking for what people say…
# Name of the card of reviews.
store-app-people = What people say
