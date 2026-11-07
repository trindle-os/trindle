
# machine translated (French), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } application
       *[other] { $count } applications
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } mise à jour est en attente
       *[other] { $count } mises à jour sont en attente
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } à choisir sur Flathub ; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = tout ce qui est à vous est à jour

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Aucune application ne correspond à « { $text } ».

# Line under the title of an empty page of apps.
store-grid-none-here = Il n'y a pas encore d'applications ici.

# Advice in the empty page of a search.
store-grid-try-shorter = Essayez un mot plus court, ou parcourez une catégorie à gauche.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Cherchez par nom ci-dessus, ou parcourez une autre catégorie à gauche.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $shown } affichées sur { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = En afficher { $next } de plus

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Récupération de la liste des applications depuis Flathub. La première fois, cela prend une minute.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Apps n'a pas pu joindre Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Cet ordinateur n'est pas connecté à Internet. Apps réessaie tout seul dès qu'il l'est.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Les applications à choisir viennent de Flathub, par Internet.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } et les réglages des applications, à gauche, fonctionnent sans cela.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } et les réglages des applications, à gauche, fonctionnent sans Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } et les réglages des applications, à gauche, fonctionnent sans Internet.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Réessayer

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Résultats

# Red words under the search box when no web browser opens a link.
store-no-browser = Aucun navigateur web pour ouvrir { $url }. Obtenez-en un d'abord dans Apps.

# Said in the card or under the title while the lists are being read.
store-looking = Recherche…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Elle est maintenant dans Installées, et dans l'anneau

# SAFETY: needs a human review before it ships
# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Supprimée

# The word on an app's dial when installing or removing did not work.
store-done-not = Non fait

# The word on an app's dial when it was installed.
store-done-installed = Installée

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak a refusé.

# Tab of the Installed page that shows everything.
store-tab-all = Toutes

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Fichiers Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Fichiers deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = Extensions

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Rien n'est encore installé. Découvrir est plein de choses à essayer.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 extension est installée
       *[other] { $n } extensions sont installées
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 programme est installé
       *[other] { $n } programmes sont installés
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } éléments sont installés

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, depuis Flathub
       *[other] { $start }, tous depuis Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, depuis un fichier Flatpak
       *[other] { $start }, tous depuis des fichiers Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, une AppImage
       *[other] { $start }, toutes des AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, depuis un fichier .deb
       *[other] { $start }, tous depuis des fichiers .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start } : { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } depuis Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 depuis un fichier Flatpak
       *[other] { $n } depuis des fichiers Flatpak
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
       *[other] { $n } AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 application
       *[other] { $n } applications
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 depuis un fichier .deb
       *[other] { $n } depuis des fichiers .deb
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 extension
       *[other] { $n } extensions
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Il est à jour.

# End of the Installed line when everything is up to date.
store-installed-state-all = Tous sont à jour.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 a une mise à jour en attente.
       *[other] { $n } ont une mise à jour en attente.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } programmes sur { $total } correspondent à « { $find } ».

# The same when plugins are among the things.
store-installed-match-things = { $shown } éléments sur { $total } correspondent à « { $find } ».

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Ajouter une AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = Rechercher parmi ce qui est installé

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = L'AppImage n'a pas pu être ajoutée.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Les applications que vous installez depuis Découvrir, depuis GitHub ou en AppImage, les programmes des fichiers .deb et les extensions apparaissent ici.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Rien d'installé ne correspond à « { $find } ».

# SAFETY: needs a human review before it ships
# Note at the foot of the Installed list.
store-installed-remove-twice = Supprimer demande confirmation : appuyez de nouveau dans les trois secondes.

# On an installed thing's line: its version.
store-installed-version = version { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = retenue

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = retenue ; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = Mise à jour de { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Mettre à jour { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = À propos

# Under an installed thing's row while it is being taken off.
store-installed-removing = Suppression...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Il n'y a pas de bus de session pour demander quel fichier.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Choisir une AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Le sélecteur de fichiers n'a pas pu être appelé.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Ajout...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } est déjà là : voir Installées.

# Words once an AppImage is in the ring.
store-add-in-ring = Ajouté. C'est maintenant dans votre liste.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Ce n'est pas une référence Flatpak que cet ordinateur peut utiliser.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = Installation de { $name } depuis { $host }...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } est installé.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak n'a pas voulu l'installer.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } n'a pas pu être installé. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Remplacement...

# SAFETY: needs a human review before it ships
# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } est déjà là

# Small heading over the AppImage being brought.
store-replace-new = Celle qu'on apporte

# Small heading over the AppImage already there.
store-replace-old = Celle qui est déjà là

# SAFETY: needs a human review before it ships
# Tip of the round button that keeps the AppImage already there.
store-replace-skip = Ignorer : garder celle qui est déjà là

# SAFETY: needs a human review before it ships
# Button that replaces the AppImage already there with the new one.
store-replace-do = Remplacer

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Cela ne s'est pas terminé, et aucune raison n'a été donnée.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Montrer où est le problème

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Rechercher sur GitHub : tapez, puis appuyez sur Entrée

# Heading of the card of matches from AppImageHub.
store-gh-hub = Sur AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Appuyez sur Entrée pour aussi chercher dans tout GitHub.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub n'a pas été interrogé : il n'y a pas d'Internet pour le faire.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Interrogation de GitHub...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = Rien de plus sur GitHub.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] Recherche de fichiers AppImage et Flatpak dans { $n } projet...
       *[other] Recherche de fichiers AppImage et Flatpak dans { $n } projets...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Aucun résultat sur GitHub n'a de fichier AppImage ou Flatpak pour cet ordinateur.

# Heading of the card of matches from GitHub.
store-gh-on = Sur GitHub

# Heading of the card that holds the search box.
store-gh-search-card = Rechercher sur GitHub

# SAFETY: needs a human review before it ships
# Line under the title of the GitHub page.
store-gh-line = Des applications sur GitHub sous forme de fichiers AppImage ou Flatpak. Personne ne les vérifie : n'installez que celles de personnes en qui vous avez confiance.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = GitHub ne peut pas être consulté tant que cet ordinateur n'est pas connecté à Internet.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak et AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Voir les fichiers

# Button that installs a GitHub project or file.
store-gh-install = Installer

# Link to a project's page on GitHub.
store-gh-open = Ouvrir sur GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, version { $tag }

# Name of the GitHub key's button and of its box.
store-gh-key = Clé GitHub

# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = Clé GitHub : une est enregistrée

# In the GitHub key's box: what the key is for.
store-gh-key-words = Vous n'en avez presque certainement pas besoin. Sans clé, GitHub répond à 60 questions par heure depuis cet ordinateur ; avec une clé, il répond à 5 000. La clé ne fait que relever cette limite. Elle sert à consulter des fichiers publics et rien d'autre : elle ne donne à Trindle aucun accès à votre compte, et vous ne cochez aucune permission en la créant. Elle est gardée sur cet ordinateur, dans votre trousseau, et envoyée à GitHub seulement.

# In the GitHub key's box: how to make a key.
store-gh-key-how = Créez une clé sur GitHub, donnez-lui le nom de votre choix, ne touchez à aucune permission, appuyez sur Generate token, puis collez ici ce qui s'affiche.

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = Impossible de la supprimer : { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = Impossible de la garder : { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = Laissez vide pour la garder

# Placeholder of the GitHub key's box.
store-gh-key-paste = Collez la clé

# SAFETY: needs a human review before it ships
# Tip of the round button that removes the stored GitHub key.
store-gh-key-remove = Supprimer la clé

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = Créer une clé sur GitHub

# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = Garder la clé

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Rien dans sa dernière version ne s'installe ici.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Ajoutée

# Line under the title of the Move your apps page.
store-bk-line = Vos applications et réglages, dans un seul fichier à emporter n'importe où.

# Heading of the card that makes a backup.
store-bk-card = Sauvegarder

# Tip of the round button while the backup is being made.
store-bk-making-tip = Création de la sauvegarde

# Main button that makes the backup.
store-bk-make = Faire la sauvegarde

# Tip of the round button that shows the backup in Folders.
store-bk-show = L'afficher dans Folders

# Name of the switch for the list of apps in a backup.
store-bk-apps = Applications

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] La liste de votre 1 application Flatpak.
       *[other] La liste de vos { $n } applications Flatpak.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } L'autre ordinateur les installe de nouveau depuis Flathub, le fichier reste donc petit.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Données propres des applications

# Under the Apps' own data switch.
store-bk-data-sub = Les réglages et le travail enregistré de chaque application, sans ses caches.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = Gardées entières, car ce sont des fichiers uniques.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Réglages de Trindle

# Under the Trindle settings switch.
store-bk-settings-sub = L'anneau, le choix du fond d'écran, les couleurs, le clavier, l'alimentation, les applications par défaut et votre image.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Actuellement { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Où il va

# Under that row: the folder and the file's name.
store-bk-where-sub = Votre dossier personnel, sous le nom { $name }

# Heading of the card that puts a backup back.
store-bk-back-card = Remettre une sauvegarde

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Choisir un fichier de sauvegarde

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Une sauvegarde de Trindle, ou une faite par { $flat }.

# SAFETY: needs a human review before it ships
# Note in the card that puts a backup back.
store-bk-back-note = Ses applications sont installées et ses données et réglages remis en place. Vos documents et images n'y sont pas : copiez-les vous-même.

# Said while a backup is being made.
store-bk-making = Sauvegarde en cours... cela peut prendre quelques minutes.

# Said when a backup is made: the file and its size.
store-bk-made = C'est { $path } ({ $size }). Copiez-le sur une clé USB ou un stockage en ligne pour l'emporter.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Aucune sauvegarde n'a été choisie, car Trindle n'a pas de connexion au sélecteur de fichiers.

# Title of the window that asks which backup to use.
store-bk-choose-title = Choisir une sauvegarde

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Sauvegardes de Trindle et de Flat

# Red words when the file chooser gave no answer.
store-bk-no-answer = Aucune sauvegarde n'a été choisie, car le sélecteur de fichiers n'a pas répondu.

# Said while a backup file is read.
store-bk-reading = Lecture de la sauvegarde...

# Said when a backup was put back.
store-bk-logout = Déconnectez-vous puis reconnectez-vous pour voir tous les réglages.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Déconnectez-vous puis reconnectez-vous pour voir tous les réglages.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 application, { $with } avec leurs réglages. Démarrage...
       *[other] { $apps } applications, { $with } avec leurs réglages. Démarrage...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Aucune source propre à un créateur n'est encore ajoutée.
        [one] 1 source de créateur ajoutée, tenue à jour avec les mises à jour de Debian.
       *[other] { $n } sources de créateurs ajoutées, tenues à jour avec les mises à jour de Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Ajoutée et tenue à jour.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. La source est ajoutée, mais le programme n'est pas installé.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Installé autrement.

# SAFETY: needs a human review before it ships
# Tip of the round button that removes a maker's source and its program.
store-src-remove-both = Le supprimer, avec sa source

# Link to a maker's page, naming the program.
store-src-open = Ouvrir { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Se connecter à Tailscale

# Tip of the round button that adds a maker's source, naming the program.
store-src-add = Ajouter { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = Prêtes à ajouter

# Name of the field for the address of a source.
store-src-address = Adresse

# Under the address field.
store-src-address-sub = L'adresse du dépôt, tirée des instructions d'installation de son créateur.

# Name of the field for the suite of a source.
store-src-suite = Suite

# Under the suite field.
store-src-suite-sub = En général trixie ou stable.

# Name of the field for the components of a source.
store-src-components = Composants

# Under the components field.
store-src-components-sub = En général main.

# Name of the field for a source's signing key.
store-src-key = Clé de signature

# Under the signing key field.
store-src-key-sub = L'adresse de sa clé de signature.

# Name of the field for the program to install from a source.
store-src-program = Programme à installer

# Under the program field.
store-src-program-sub = Le nom de son paquet, si vous voulez l'installer maintenant.

# Placeholder of the program field.
store-src-program-hint = facultatif

# Tip of the round button that adds the source typed.
store-src-add-this = Ajouter cette source

# Heading of the card that adds another source.
store-src-another = Ajouter une autre source

# SAFETY: needs a human review before it ships
# Note in that card.
store-src-another-note = Une source peut installer des mises à jour pour tout l'ordinateur, n'ajoutez donc que des créateurs en qui vous avez confiance. Chacune demande le mot de passe d'un administrateur.

# SAFETY: needs a human review before it ships
# Tip of the round button that removes a source that is already on the computer.
store-src-remove-this = Supprimer cette source

# Heading of the card of sources already on the computer.
store-src-already = Déjà sur cet ordinateur

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Les deux adresses doivent commencer par https://

# Red words when the suite or components are not plain words.
store-src-bad-words = La suite et les composants doivent être des mots simples.

# Red words when the program is not a package name.
store-src-bad-package = Ce n'est pas un nom de paquet.

# Word on a source's dial while its job starts.
store-src-starting = Démarrage

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Ouverture de la page de connexion de Tailscale...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Connectez-vous sur la page qui s'est ouverte ; cela se termine tout seul.

# Said under Tailscale's row when it is connected.
store-src-connected = Connecté

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
store-bk-http-line = Le lien vers les applications n'est pas chiffré.

# SAFETY: needs a human review before it ships
# Title of the card shown before a restore from an http address over the internet.
store-bk-http-title = Non chiffré

# SAFETY: needs a human review before it ships
# Words of that card.
store-bk-http-card = Cette adresse n'est pas chiffrée. Quelqu'un sur le chemin pourrait voir ce qui est envoyé, et pourrait remplacer la sauvegarde par une autre.

# SAFETY: needs a human review before it ships
# Label of the tick box on that card; the restore button works only once it is on.
store-bk-http-tick = Je comprends et j'accepte ce risque

# SAFETY: needs a human review before it ships
# Button on that card that restores the backup.
store-bk-http-go = Restaurer

# SAFETY: needs a human review before it ships
# Button on that card that gives the restore up.
store-bk-http-cancel = Annuler

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = Retirer { $name } ?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Ses données enregistrées occupent { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = Elle n'a aucune donnée enregistrée.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Calcul de ses données enregistrées...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Les données supprimées ne peuvent pas être récupérées.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Retirer et garder ses données

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Retirer et supprimer ses données

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Supprimer seulement ses données

# Why Delete its data only is greyed.
store-flatpak-data-only-none = Elle n'a aucune donnée enregistrée à supprimer.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Annuler : tout garder

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = Ses données enregistrées sont supprimées.

# Words while only the data is being deleted.
store-flatpak-data-going = Suppression de ses données enregistrées...

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = Ses données enregistrées n'ont pas pu être comptées.

store-hub-warning = AppImageHub ne fait que répertorier des projets : personne ne les examine ni ne les analyse, et ils ne sont pas forcément signés. Trindle exécute chaque AppImage dans une boîte avec un dossier personnel privé : au début, il peut utiliser internet et le son, et ne voit aucun de vos fichiers.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = L'origine d'un AppImage est une indication, pas une vérification.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = Sur GitHub : { $repo }

# The same, with how many stars the project has.
store-from-github-stars = Sur GitHub : { $repo }, { $stars } étoiles

# Where an AppImage comes from: a website, by its name.
store-from-site = Pas sur GitHub : d'un site web, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Origine inconnue

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Ouvrir son site web

# When an app was last updated.
store-age-updated = Dernière mise à jour : { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Dernière mise à jour : { $date }. Pas mis à jour depuis plus de six mois, il peut donc avoir des failles de sécurité non corrigées.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Dernière mise à jour : inconnue

# When an app's project was made.
store-age-created = Créé le { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Création : inconnue

# A date the source does not give, on a line of Facts.
store-age-not-known = Inconnu

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Avant d'en ajouter un

# Title of the card that lists the AppImages.
store-browse-list = AppImages

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] { $count } AppImage est listé sur AppImageHub.
       *[other] { $count } AppImages sont listés sur AppImageHub.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Récupération de la liste d'AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = La liste d'AppImageHub n'a pas pu être récupérée.

# The same with no internet and no copy kept.
store-browse-offline = La liste d'AppImageHub ne peut pas être récupérée tant que cet ordinateur n'est pas connecté à internet.

# In the list when the kind chosen has none.
store-browse-none = AppImageHub n'en liste aucun de ce genre.

# Under the rows: how many are shown of how many.
store-browse-showing = { $shown } affichés sur { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Ajouter aux applications de Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Voir les détails et les fichiers



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = la recherche de mises à jour est désactivée

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT reviendra quand OpenAI aura signé son téléchargement.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } au choix sur Flathub et AppImageHub ; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } au choix sur AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Les deux

# The same: only Flatpaks.
store-show-flatpaks = Flatpaks

# The same: only AppImages.
store-show-appimages = AppImages

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage : { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = Les AppImages ne sont pas vérifiés. Personne ne les examine ni ne les analyse.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle exécute chaque AppImage dans une boîte avec un dossier personnel privé : au début, il peut utiliser internet et le son, et ne voit aucun de vos fichiers.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Ne plus afficher

# Button on that card that goes on with the install.
store-appimage-warn-ok = OK

# Button on that card that gives the install up.
store-appimage-warn-cancel = Annuler

