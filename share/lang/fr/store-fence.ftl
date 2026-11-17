
# machine translated (French), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Ce n'est pas un nom de programme valide.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Cela n'a pas été enregistré.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Choisissez un dossier qui existe.

# SAFETY: needs a human review before it ships
# Red words when the whole home folder is chosen to share.
store-fence-whole-home = Tout le dossier personnel reste privé.

# SAFETY: needs a human review before it ships
# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = Ce dossier contient votre dossier personnel, il reste donc privé.

# SAFETY: needs a human review before it ships
# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = Seuls les dossiers à l'intérieur de votre dossier personnel peuvent être partagés.

# SAFETY: needs a human review before it ships
# Red words when a hidden folder is chosen to share.
store-fence-hidden = Les dossiers cachés restent toujours privés.

# SAFETY: needs a human review before it ships
# Red words when a folder that keeps passwords or mail is chosen to share.
store-fence-passwords = Ce dossier contient des mots de passe ou du courrier, il reste donc privé.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Ce programme n'a pas de dossier de réglages à utiliser.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Tapez le nom d'un site, comme api.example.com, ou *.example.com pour tous les sites sous ce nom.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Tapez le nom d'un ordinateur, comme nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Écrivez chacun sous la forme NOM=valeur.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Écrivez chacun sous la forme NOM=valeur, avec des majuscules.

# SAFETY: needs a human review before it ships
# Words above the question when an AI program asks to use a folder.
store-fence-ask-share = Un programme d'IA demande à utiliser ce dossier. Il ne peut pas le voir pour l'instant.

# SAFETY: needs a human review before it ships
# Words above the question when an AI program asks to run something as the administrator.
store-fence-ask-admin = Un programme d'IA demande à exécuter ceci en tant qu'administrateur. Il s'exécuterait en dehors de la clôture et pourrait voir tout ce que vous voyez.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = Programme d'IA

# SAFETY: needs a human review before it ships
# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = Le laisser seulement lire ce dossier

# Note under the question about a folder.
store-fence-sees-next = Il verra le dossier au prochain démarrage du programme.

# SAFETY: needs a human review before it ships
# Tip of the round button that refuses a folder.
store-fence-no = Non

# SAFETY: needs a human review before it ships
# Tip of the round button that shares a folder until the session ends.
store-fence-share-session = Partager pour cette session seulement

# SAFETY: needs a human review before it ships
# Main button that shares a folder for good.
store-fence-share-always = Toujours partager

# SAFETY: needs a human review before it ships
# Tip of the round button that refuses to run something as the administrator.
store-fence-dont-allow = Ne pas autoriser

# SAFETY: needs a human review before it ships
# Main button that lets something run as the administrator.
store-fence-allow = Autoriser

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Cette commande est trop longue pour être affichée en entier, elle n'a donc pas été exécutée.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Une autre demande attend votre réponse.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = Vous ne l'avez pas autorisé.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } n'est pas disponible à l'intérieur de la clôture.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = Il n'a pas pu être démarré en dehors de la clôture.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Donnez le chemin complet du dossier.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = Vous ne l'avez pas partagé.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Partagé. Il sera là au prochain démarrage de ce programme.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Nommez l'ordinateur à joindre.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = Les programmes d'IA dans l'enclos ne peuvent pas joindre cet ordinateur avec ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = La clé pour les programmes d'IA n'a pas pu être créée : { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Nommez une commande à exécuter.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } n'est pas disponible ici.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } n'est pas disponible à l'intérieur de la clôture. Demandez qu'il soit exécuté à l'extérieur.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name } : la ligne de demande de la clôture est absente.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name } : la demande n'a pas été envoyée.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock a refusé : { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = La clôture n'a pas démarré : { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = La clôture n'a pas démarré : { $path } : { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = Les programmes d'IA gardent ici leurs propres connexions, à part des vôtres. Connectez-vous dans le programme quand il le demande.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Vos dossiers sont privés pour les programmes d'IA. Partagez-en un dans Réglages, Programmes d'IA, pour y travailler.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = La clôture n'a pas démarré : le filtre des appels système n'a pas pu être créé.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = La clôture n'a pas démarré : la limite réseau n'a pas pu être mise en place.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = La clôture n'a pas démarré : bubblewrap ne s'est pas lancé : { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name } : introuvable en dehors de la clôture
