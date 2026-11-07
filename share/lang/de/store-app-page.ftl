
# machine translated (German), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.

# Under the app's name when only the maker is known.
store-app-line-by = Von { $by }.

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, von { $by }.

# The state beside the dot while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-state-removing = Wird entfernt

# The state beside the dot while the app is being fetched.
store-app-state-downloading = Wird heruntergeladen

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = Wird eingespielt

# The state beside the dot while the app's own update goes in.
store-app-state-updating = Wird aktualisiert

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = Installiert, zurückgehalten

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = Installiert, zurückgehalten bei Version { $version }

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = Installiert, { $waits }

# The same, with the installed version.
store-app-state-waiting-version = Installiert, Version { $version }, { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = Installiert, aktuell

# The same, with the installed version.
store-app-state-current-version = Installiert, Version { $version }, aktuell

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = Nicht installiert

# The same, with the size of the download.
store-app-state-absent-size = Nicht installiert, { $size } Download

# Word beside the spinner while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-busy-removing = Wird entfernt

# Word beside the spinner while the app is being put in.
store-app-busy-installing = Wird installiert

# Tip and spoken name of the round button that gives up a download.
store-app-stop = Stopp

# Tip and spoken name of the round button that takes the app off.
# SAFETY: needs a human review before it ships
store-app-remove = Entfernen

# Round button beside Update or Release that opens the app.
store-app-open-round = Öffnen

# Main button that releases an app whose updates are held back.
store-app-release = Freigeben

# Main button that updates the app.
store-app-update = Aktualisieren

# Main button that opens the app.
store-app-open = Öffnen

# Main button that installs the app.
store-app-install = Installieren

# Button that opens the app's website.
store-app-website = Website

# In the picture frame when the app has no pictures.
store-app-no-pictures = Diese App hat keine Bilder.

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = Dieses Bild konnte nicht geladen werden.

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = Das Bild wird geladen …

# In the picture frame when the computer has no internet.
store-app-picture-offline = Bilder können nicht geladen werden, solange dieser Computer nicht mit dem Internet verbunden ist.

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = Bild { $at } von { $n }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = Bild { $at } von { $n }: { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = Früheres Bild

# Tip of the round button that shows the picture after.
store-app-picture-later = Späteres Bild

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = Proprietär

# Name of the Facts card.
store-app-facts = Fakten

# Facts line: the size of the download.
store-app-fact-download = Downloadgröße

# Facts line: where the app comes from.
store-app-fact-source = Quelle

# Facts line: the version.
store-app-fact-version = Version

# Facts line: the licence of an open source app.
store-app-fact-licence = Lizenz

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = Lizenz, nicht Open Source

# Facts line: who made the app.
store-app-fact-made-by = Entwickelt von

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  aus { $n ->
        [one] { $count } Bewertung
       *[other] { $count } Bewertungen
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = Noch keine Bewertungen.

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = Keine Bewertungen zum Anzeigen.

# Under the stars while the ratings are being fetched.
store-app-rating-looking = Bewertungen werden gesucht …

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = Vom offenen Bewertungsdienst von GNOME.

# Last line of the Rating card when there is no internet.
store-app-rating-offline = Dieser Computer ist nicht mit dem Internet verbunden.

# Name of the Rating card.
store-app-rating = Bewertung

# Text of the About card when the app says nothing of itself.
store-app-no-description = Diese App hat keine Beschreibung.

# Name of the About card.
store-app-about = Über die App

# Heading of a review that has no title.
store-app-review-no-title = Ohne Titel

# Under What people say when nobody has written.
store-app-people-none = Noch niemand hat über diese App geschrieben.

# Under What people say when there is no internet.
store-app-people-offline = Was andere sagen, kann nicht geladen werden, solange dieser Computer nicht mit dem Internet verbunden ist.

# Under What people say while reviews are being fetched.
store-app-people-looking = Es wird gesucht, was andere sagen …

# Name of the card of reviews.
store-app-people = Was andere sagen

# Facts line: when the app's project was made.
store-app-fact-created = Erstellt

