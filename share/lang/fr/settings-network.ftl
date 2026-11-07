# machine translated (French), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } est coupé par un interrupteur ou une touche de l'ordinateur lui-même. Activez-le, puis réessayez.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } est bloqué par le système, et le blocage n'a pas pu être levé.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
settings-network-join-password-typed = Ce mot de passe n'a pas été accepté. Essayez encore.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
settings-network-join-password-kept = Le mot de passe conservé pour ce réseau n'a pas été accepté. Oubliez le réseau avec la corbeille, puis rejoignez-le à nouveau.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Ce réseau est introuvable. Vérifiez son nom, et qu'il est à portée.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Le réseau a laissé entrer cet ordinateur mais ne lui a pas donné d'adresse. Réessayez, ou redémarrez le routeur.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Le réseau n'a pas laissé cet ordinateur le rejoindre. Rapprochez-vous, puis réessayez.

# Said in red when the password typed is one a Wi-Fi network cannot have.
settings-network-bad-password = Ce mot de passe n'est pas possible pour un réseau Wi-Fi. Vérifiez-le, puis réessayez.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Le réseau n'a pas pu être rejoint.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = ouvrir avec votre mot de passe

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = ouvrir et modifier avec votre mot de passe

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = tout le monde à la maison peut ouvrir

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = tout le monde à la maison peut modifier

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place } : { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = Arrêter de partager { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Non géré

# State of a network device that cannot be used.
settings-network-state-unavailable = Indisponible

# State of a network device that is not connected.
settings-network-state-disconnected = Déconnecté

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Préparation

# State of a network device that is being set up.
settings-network-state-configuring = Configuration

# State of a network device that needs a password.
settings-network-state-needs-password = Demande un mot de passe

# State of a network device that is getting an address.
settings-network-state-address = Obtention d'une adresse

# State of a network device whose connection is being checked.
settings-network-state-checking = Vérification

# State of a network device that is waiting.
settings-network-state-waiting = En attente

# State of a network device that is connected.
settings-network-state-connected = Connecté

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Déconnexion

# State of a network device that failed.
settings-network-state-failed = Échec

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Connecté par câble, et à { $name } par Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Connecté à { $name }, signal fort.
        [good] Connecté à { $name }, bon signal.
        [fair] Connecté à { $name }, signal moyen.
       *[other] Connecté à { $name }, signal faible.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Connecté par câble.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Le Wi-Fi est désactivé, cet ordinateur n'est donc pas connecté.

# Headline of the page when not connected.
settings-network-top-none = Non connecté. Choisissez un réseau ci-dessous.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Recherche de réseaux…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Les réseaux ne peuvent pas être consultés : NetworkManager ne tourne pas.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Le bus du système

# Name of the page in the list of settings pages and its heading.
settings-network-title = Réseau

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi sans fil internet ethernet câble mot de passe connecter pare-feu partage partager imprimante dossier

# Help text at the top of the page.
settings-network-help = Activez ou désactivez le Wi-Fi, rejoignez un réseau, oubliez-en un dont vous ne voulez plus, ou laissez d'autres ordinateurs se connecter.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Signal fort de { $name }.
        [good] Bon signal de { $name }.
        [fair] Signal moyen de { $name }.
       *[other] Signal faible de { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Sa puce demande un Linux plus récent.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Cet ordinateur n'a pas de Wi-Fi fonctionnel.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Désactivé. Activez-le pour rejoindre un réseau.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Activé, et non connecté à un réseau.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Utiliser le Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi et câble

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Le Wi-Fi relie cet ordinateur à un réseau sans câble. Désactivé, il cesse de chercher des réseaux et économise un peu de batterie. Un réseau qui ne diffuse pas son nom n'apparaît jamais dans la liste des réseaux à portée : rejoignez-le ici en écrivant son nom exactement, majuscules comprises. Quand la puce Wi-Fi de cet ordinateur est de celles qu'aucun pilote de son Linux ne gère encore, il faut un Linux plus récent que celui sur lequel tourne Trindle. Un câble réseau a sa propre ligne, avec un bouton pour le connecter ou le déconnecter.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Laisser d'autres ordinateurs se connecter

# Small line under that switch.
settings-network-share-switch-sub = Pour partager des dossiers et des imprimantes à la maison.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Recherche des dossiers partagés…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Les dossiers partagés n'ont pas pu être lus

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Pas encore accessibles

# Small line under it.
settings-network-not-reachable-sub = Les autres ne peuvent pas les ouvrir tant que c'est désactivé.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Aucun dossier n'est partagé

# Small line under it.
settings-network-no-shared-sub = Faites un clic droit sur l'un d'eux dans Dossiers pour le partager.

# Heading of the Sharing card.
settings-network-sharing = Partage

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Tant que « Laisser d'autres ordinateurs se connecter » est désactivé, rien sur le réseau ne peut atteindre cet ordinateur s'il n'a pas demandé d'abord, si bien que personne sur le même Wi-Fi ne peut tenter sa chance. Activez-le pour partager un dossier ou une imprimante avec d'autres ordinateurs à la maison. Pour partager un dossier, faites un clic droit dessus dans Dossiers et choisissez Partager sur le réseau. Chacun indique qui peut faire quoi. Avec votre mot de passe : les autres peuvent ouvrir les fichiers, ou les ouvrir et les modifier, avec votre mot de passe de partage. Tout le monde à la maison : toute personne sur le réseau peut ouvrir les fichiers, ou les ouvrir et les modifier, sans mot de passe. Les autres ordinateurs de la maison ne peuvent en ouvrir aucun tant que l'interrupteur est désactivé.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Arrêt…

# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = En attente du mot de passe...

# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = Partage autorisé

# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = Tenus à l'écart

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Le Wi-Fi n'a pas pu être activé.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Le Wi-Fi n'a pas pu être désactivé.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Déconnecter

# Tip of the button that connects a network or a cable.
settings-network-connect = Connecter

# Shown under a network or cable while it is being connected.
settings-network-connecting = Connexion...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Aucun réseau à portée

# Small line under it.
settings-network-none-in-range-sub = Recherché à nouveau à chaque ouverture de cette page.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Réseaux à portée

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = Tous les réseaux Wi-Fi que cet ordinateur capte. À l'ouverture de la page, celui utilisé est en premier, puis ceux enregistrés ici, puis les autres selon la force de leur signal ; une ligne reste en place tant que la page est ouverte. Appuyez sur les flèches pour en rejoindre un ; un réseau sécurisé demande son mot de passe la première fois. La corbeille oublie un réseau enregistré et son mot de passe. Un réseau ouvert n'est pas chiffré : des personnes proches pourraient voir ce qui y est envoyé.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Activez le Wi-Fi pour voir les réseaux à portée.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Enregistré

# Part of a network's small line: it needs a password.
settings-network-secured = Sécurisé

# Part of a network's small line: it has no password and is not encrypted.
# SAFETY: needs a human review before it ships
settings-network-open = Ouvert, non chiffré

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Oublier ce réseau

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = Le réseau n'a pas pu être oublié.

# Placeholder of the box where a network's password is typed.
settings-network-password = Mot de passe

# Tip of the button that joins the network.
settings-network-join = Rejoindre

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Annuler

# Shown under a network while it is being joined.
settings-network-joining = Connexion...

# Said in red when a Wi-Fi password is too short.
settings-network-password-short = Les mots de passe Wi-Fi font au moins 8 caractères.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Rejoindre un réseau masqué

# Small line under it.
settings-network-hidden-sub = Écrivez son nom exactement, majuscules comprises.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Nom du réseau

# Placeholder of the box where a hidden network's password is typed.
settings-network-hidden-password = Mot de passe, s'il y en a un

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Le nom du réseau est nécessaire.

# Shown under a network after it was joined.
settings-network-joined = Connecté

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Il a quand même été enregistré : oubliez-le avec la corbeille avant de le rejoindre à nouveau.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Il ne s'est pas déconnecté.
