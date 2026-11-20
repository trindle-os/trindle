
# machine translated (French), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Paquets Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Retenue à la version { $have } ; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Retenue ; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Retenue à la version { $have }

# On a thing held back with no update waiting.
store-upd-held = Retenue

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Retenue à la version { $have }. Il n'y a rien de plus récent pour l'instant.

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Retenue ; rien de plus récent pour l'instant

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 mise à jour
       *[other] { $n } mises à jour
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] 1 mise à jour est prête
       *[other] { $n } mises à jour sont prêtes
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, { $size } au total

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 est retenue
       *[other] { $n } sont retenues
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text } ; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = Redémarrez pour terminer la mise à jour du micrologiciel.

# Headline while the check runs.
store-upd-checking = Recherche de mises à jour.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Mise à jour de { $name }. Vous pouvez continuer à travailler pendant l'installation.

# Headline while all the updates go in.
store-upd-installing-all = Installation des mises à jour. Vous pouvez continuer à travailler pendant l'installation.

# Headline when the updates are in and the computer must start again.
store-upd-restart = Redémarrez l'ordinateur pour terminer l'installation des mises à jour.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Redémarrez pour terminer l'installation.

# Added to the headline with no internet.
store-upd-no-more = Cet ordinateur n'est pas connecté à Internet : Trindle ne peut donc pas chercher d'autres mises à jour.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Aucune mise à jour n'est prête

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Trindle ne connaît aucune mise à jour en attente. Il ne peut pas en chercher d'autres tant que cet ordinateur n'est pas connecté à Internet.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = les applications

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle et Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = le micrologiciel

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Trindle n'a pas pu rechercher les mises à jour de { $names }.

# Headline when everything is up to date.
store-upd-current = Tout est à jour.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Dernière vérification à { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Impossible à vérifier sans Internet

# Red words under a card when a check took too long.
store-upd-too-long = La vérification a pris trop de temps.

# Tip of the round button that checks again.
store-upd-check-again = Vérifier de nouveau

# Main button while the check runs.
store-upd-checking-main = Vérification

# Main button while the updates go in.
store-upd-installing-main = Installation

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] Installer 1 mise à jour
       *[other] Installer { $n } mises à jour
    }

# Main button that restarts the computer.
store-upd-restart-now = Redémarrer maintenant
# Beside Restart now: the box that puts the restart off, then the ring's Updates button reminds. Its answers are the choices below.
store-upd-remind-later = Me le rappeler plus tard
# Red words under the headline when the Remind me later time could not be saved; the reason follows.
store-upd-remind-not-saved = Impossible d’enregistrer le rappel : { $why }.
# One answer in the Remind me later box.
store-upd-remind-hours =
    { $n ->
        [one] Dans 1 heure
       *[other] Dans { $n } heures
    }

# Main button that checks for updates.
store-upd-check-for = Rechercher des mises à jour

# Tip of the round button that releases a thing held back.
store-upd-release = Autoriser les mises à jour de { $name }

# Tip of the round button that holds a thing's updates back.
store-upd-hold = Retenir { $name }

# A card's state while it is being checked.
store-upd-status-checking = Vérification

# A card's state while its updates go in.
store-upd-status-installing = Installation

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Vérification impossible. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text } ; { $held } retenues

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } retenues

# A card's state when nothing waits.
store-upd-up-to-date = À jour

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = Depuis { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = Depuis { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Une partie partagée, depuis { $from }

# Note under the Apps card.
store-upd-yours-alone = Ces applications sont réservées à vous seul, elles se mettent donc à jour sans mot de passe. La mise à jour d'une application installe aussi les parties partagées dont elle a besoin.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Une mise à jour retenue reste à sa version actuelle. Elle est ignorée quand vous installez toutes les mises à jour, ici et sur l'anneau, jusqu'à ce que vous autorisiez de nouveau les mises à jour.

# Heading of the Apps card.
store-upd-card-apps = Applications

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Chaque module est à jour.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] La mise à jour de 1 module est retenue.
       *[other] Les mises à jour de { $n } modules sont retenues.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 module a une mise à jour
       *[other] { $n } modules ont une mise à jour
    }

# Heading of the Plugins card.
store-upd-card-plugins = Modules

# Held back card: nothing is held back.
store-upd-nothing-held = Rien n'est retenu.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Non retenue

# Note under the Held back card: what releasing does.
store-upd-release-why = Une fois les mises à jour de nouveau autorisées, elle est installée avec le reste, ici et sur l'anneau.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = Le micrologiciel d'un appareil

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = le micrologiciel de l'appareil { $id }

# Heading of the Held back card.
store-upd-card-held = Retenues

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } vers { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 mise à jour, installée toute seule
       *[other] { $n } mises à jour, installées ensemble
    }

# Under Debian's rows: how many of them show.
store-upd-debian-showing = { $shown } affichées sur { $total } mises à jour Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Cet ordinateur a Trindle { $version }.

# SAFETY: needs a human review before it ships
# Heading of the card about the daily security updates.
store-upd-card-security = Mises à jour de sécurité

# On the security card: its state.
store-upd-security-on = Activées, chaque jour

# SAFETY: needs a human review before it ships
# On the security card: that holding back never stops these.
store-upd-never-held = Retenir une mise à jour ne bloque jamais les mises à jour de sécurité.

# Firmware card: the computer has no updater.
store-upd-fw-none = Cet ordinateur n'a pas d'outil de mise à jour du micrologiciel.

# Firmware card: restart to finish.
store-upd-fw-restart = Redémarrez pour terminer

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Version { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Le nouveau micrologiciel s'installe au redémarrage de l'ordinateur.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Installez fwupd pour garder le micrologiciel à jour.

# Heading of the Firmware card.
store-upd-card-firmware = Micrologiciel

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd n'a pas pu récupérer la dernière liste.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub n'a pas pu être joint.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Trindle n'a pas pu récupérer les listes de Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Trindle n'a pas pu récupérer les listes de Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Trindle n'a pas pu la retenir, car il n'a pas pu enregistrer la liste des mises à jour retenues : { $why }.

# The same, for a release.
store-upd-not-released-list = Trindle n'a pas pu autoriser de nouveau les mises à jour, car il n'a pas pu enregistrer la liste des mises à jour retenues : { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Elle est retenue dans Apps, mais Trindle n'a pas pu le signaler à Flatpak. Un autre programme qui met à jour les applications pourrait encore l'installer.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Les mises à jour sont de nouveau autorisées dans Apps, mais Trindle n'a pas pu le signaler à Flatpak, qui pourrait donc encore la laisser de côté. Retenez-la, puis autorisez de nouveau les mises à jour.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak a dit : { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle

# Said when a security fix for a boxed app can't be downloaded. { $package } is the package, { $version } the fixed version.
store-patch-not-fetched = Trindle n'a pas pu télécharger { $package } { $version }.

# Said when a security fix for a boxed app was downloaded but isn't there to use.
store-patch-not-found = Trindle n'a pas retrouvé le paquet qu'il avait téléchargé.

# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = La recherche automatique de mises à jour est désactivée.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Non vérifié

# Tip of the round button that ends an update that is running.
store-upd-stop = Arrêter

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Arrêt en cours

# Under a row whose update was stopped.
store-upd-stopped = Arrêté. Appuyez sur Mettre à jour pour continuer.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Arrêté. Relancez l'installation pour continuer.

# Note under the Apps card.
store-upd-steam-restarts = Steam redémarre quand plus aucun jeu ne tourne.

