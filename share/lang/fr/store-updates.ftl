
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
store-upd-held-version-nothing = Retenue à la version { $have } ; rien de plus récent pour l'instant

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
store-upd-firmware-restart = Le micrologiciel a besoin d'un redémarrage pour se terminer.

# Headline while the check runs.
store-upd-checking = Recherche de mises à jour.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Mise à jour de { $name }. Vous pouvez continuer à travailler pendant l'installation.

# Headline while all the updates go in.
store-upd-installing-all = Installation des mises à jour. Vous pouvez continuer à travailler pendant l'installation.

# Headline when the updates are in and the computer must start again.
store-upd-restart = Redémarrez pour terminer : les mises à jour sont installées, et entrent en service au prochain démarrage de l'ordinateur.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Ce qui vient d'être installé a besoin d'un redémarrage pour se terminer.

# Added to the headline with no internet.
store-upd-no-more = Cet ordinateur n'est pas connecté à Internet, aucune autre recherche n'est donc possible.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Aucune mise à jour n'est prête

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Aucune mise à jour n'attend, à la connaissance de cet ordinateur. Il ne peut pas en chercher d'autres tant qu'il n'est pas connecté à Internet.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = les applications

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle et Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = le micrologiciel

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Les mises à jour n'ont pas pu être recherchées pour { $names }.

# Headline when everything is up to date.
store-upd-current = Tout est à jour.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Dernière vérification à { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Impossible à vérifier sans Internet

# Red words under a card when a check took too long.
store-upd-too-long = La réponse a pris trop de temps.

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

# Main button that checks for updates.
store-upd-check-for = Rechercher des mises à jour

# Tip of the round button that releases a thing held back.
store-upd-release = Libérer { $name }

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
store-upd-yours-alone = Pour vous seul, elles se mettent donc à jour sans mot de passe. Une application apporte les parties partagées dont sa nouvelle version a besoin.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Une mise à jour retenue reste à la version actuelle, et en dehors de Tout installer ici et sur l'anneau, jusqu'à ce que vous la libériez.

# Heading of the Apps card.
store-upd-card-apps = Applications

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Chaque extension est à jour.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] La mise à jour de 1 extension est retenue.
       *[other] Les mises à jour de { $n } extensions sont retenues.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 extension a une mise à jour
       *[other] { $n } extensions ont une mise à jour
    }

# Heading of the Plugins card.
store-upd-card-plugins = Extensions

# Held back card: nothing is held back.
store-upd-nothing-held = Rien n'est retenu.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Libérée

# Note under the Held back card: what releasing does.
store-upd-release-why = Ce qui est libéré repart avec le reste, depuis Tout installer ici et sur l'anneau.

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
        [one] 1 mise à jour, qui s'installe toute seule
       *[other] { $n } mises à jour, qui s'installent ensemble
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Depuis Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = { $shown } affichées sur { $total } mises à jour de Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Cet ordinateur a Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle et Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Les paquets Debian sont faits pour aller ensemble, et un seul installé à part peut en entraîner d'autres ou les laisser désynchronisés. Ils s'installent donc ensemble, avec un seul bouton.

# SAFETY: needs a human review before it ships
# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Les mises à jour de Trindle et de Debian apportent des correctifs de sécurité pour tout l'ordinateur, elles ne peuvent donc pas être retenues.

# SAFETY: needs a human review before it ships
# Heading of the card about the daily security updates.
store-upd-card-security = Mises à jour de sécurité

# On the security card: its state.
store-upd-security-on = Activées, chaque jour

# SAFETY: needs a human review before it ships
# On the security card.
store-upd-security-note = Les correctifs de Debian s'installent chaque jour sans rien demander.

# SAFETY: needs a human review before it ships
# On the security card.
store-upd-security-part = Fait partie de Trindle ; ne peut pas être désactivé.

# SAFETY: needs a human review before it ships
# On the security card: that holding back never stops these.
store-upd-never-held = Retenir une mise à jour ne bloque jamais celles-ci.

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
store-upd-no-lists = Les listes de Debian n'ont pas pu être récupérées.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Les listes de Debian n'ont pas pu être récupérées. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Non retenue. La liste de ce qui est retenu n'a pas pu être conservée : { $why }.

# The same, for a release.
store-upd-not-released-list = Non libérée. La liste de ce qui est retenu n'a pas pu être conservée : { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Retenue dans Apps, mais Flatpak lui-même n'a pas pu en être informé, donc autre chose qui met à jour des applications pourrait encore l'installer.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Libérée dans Apps, mais Flatpak lui-même n'a pas pu en être informé et pourrait encore la laisser de côté. Retenez-la puis libérez-la de nouveau.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak a dit : { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle



# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = La recherche automatique de mises à jour est désactivée. Appuyez sur Rechercher les mises à jour pour chercher.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Non vérifié

# Tip of the round button that ends an update that is running.
store-upd-stop = Arrêter

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Arrêt en cours

# Under a row whose update was stopped.
store-upd-stopped = Arrêté. Appuyez sur Mettre à jour pour continuer.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Arrêté. Appuyez sur Installer pour continuer.

# Note under the Apps card.
store-upd-steam-restarts = Steam redémarre quand plus aucun jeu ne tourne.

