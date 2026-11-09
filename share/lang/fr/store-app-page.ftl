
# machine translated (French), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.

# Under the app's name when only the maker is known.
store-app-line-by = Par { $by }.

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, par { $by }.

# The state beside the dot while the app is being taken off.
store-app-state-removing = Suppression

# The state beside the dot while the app is being fetched.
store-app-state-downloading = Téléchargement

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = Mise en place

# The state beside the dot while the app's own update goes in.
store-app-state-updating = Mise à jour

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = Installée, mises à jour retenues

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = Installée, mises à jour retenues à la version { $version }

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = Installée, { $waits }

# The same, with the installed version.
store-app-state-waiting-version = Installée, version { $version }, { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = Installée, à jour

# The same, with the installed version.
store-app-state-current-version = Installée, version { $version }, à jour

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = Non installée

# The same, with the size of the download.
store-app-state-absent-size = Non installée, { $size } à télécharger

# Word beside the spinner while the app is being taken off.
store-app-busy-removing = Suppression

# Word beside the spinner while the app is being put in.
store-app-busy-installing = Installation

# Tip and spoken name of the round button that gives up a download.
store-app-stop = Arrêter

# SAFETY: needs a human review before it ships
# Tip and spoken name of the round button that takes the app off.
store-app-remove = Supprimer

# Round button beside Update or Release that opens the app.
store-app-open-round = Ouvrir

# Main button that releases an app whose updates are held back.
store-app-release = Libérer

# Main button that updates the app.
store-app-update = Mettre à jour

# Main button that opens the app.
store-app-open = Ouvrir

# Main button that installs the app.
store-app-install = Installer

# Button that opens the app's website.
store-app-website = Site web

# In the picture frame when the app has no pictures.
store-app-no-pictures = Cette application n'a pas d'images.

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = Cette image n'a pas pu être récupérée.

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = Récupération de l'image…

# In the picture frame when the computer has no internet.
store-app-picture-offline = Les images ne peuvent pas être récupérées tant que cet ordinateur n'est pas connecté à Internet.

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = Image { $at } sur { $n }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = Image { $at } sur { $n } : { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = Image précédente

# Tip of the round button that shows the picture after.
store-app-picture-later = Image suivante

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = Propriétaire

# Name of the Facts card.
store-app-facts = Informations

# Facts line: the size of the download.
store-app-fact-download = Taille du téléchargement

# Facts line: where the app comes from.
store-app-fact-source = Source

# Facts line: the version.
store-app-fact-version = Version

# Facts line: the licence of an open source app.
store-app-fact-licence = Licence

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = Licence, pas open source

# Facts line: who made the app.
store-app-fact-made-by = Créée par

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  sur { $n ->
        [one] { $count } avis
       *[other] { $count } avis
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = Pas encore d'évaluations.

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = Aucune évaluation à afficher.

# Under the stars while the ratings are being fetched.
store-app-rating-looking = Recherche des évaluations…

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = Du service d'avis ouvert de GNOME.

# Last line of the Rating card when there is no internet.
store-app-rating-offline = Cet ordinateur n'est pas connecté à Internet.

# Name of the Rating card.
store-app-rating = Évaluation

# Text of the About card when the app says nothing of itself.
store-app-no-description = Cette application n'a pas de description.

# Name of the About card.
store-app-about = À propos

# Heading of a review that has no title.
store-app-review-no-title = Sans titre

# Under What people say when nobody has written.
store-app-people-none = Personne n'a encore écrit sur cette application.

# Under What people say when there is no internet.
store-app-people-offline = Ce que disent les gens ne peut pas être récupéré tant que cet ordinateur n'est pas connecté à Internet.

# Under What people say while reviews are being fetched.
store-app-people-looking = Recherche de ce que disent les gens…

# Name of the card of reviews.
store-app-people = Ce que disent les gens

# Facts line: when the app's project was made.
store-app-fact-created = Créé

