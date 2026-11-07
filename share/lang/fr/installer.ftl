
# machine translated (French), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Installer { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Bienvenue

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Disque

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Confidentialité

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Mot de passe

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Vous

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Prêt

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Installation

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Terminé

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Préparation

# Red line at the foot when the installer's helper could not be started.
installer-no-start = L'installateur n'a pas pu démarrer : { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Le plan n'a pas pu être lu.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Le plan n'a pas pu être lu : { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = L'installation s'est arrêtée. Rien de plus n'a été écrit.

# Name of the round button at the foot that goes back a page.
installer-back = Retour

# Name of the main button at the foot that goes to the next page.
installer-next = Suivant

# SAFETY: needs a human review before it ships
# Name of the main button at the foot of the Ready page that starts the install.
installer-install = Installer

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Redémarrer maintenant

# Beside the keyboard test box on the first page.
installer-try-keyboard = Essayer le clavier

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Saisissez du texte ici pour essayer le clavier

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press writes beside another system.
installer-arm = Cliquez à nouveau pour installer

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press erases a whole disk.
installer-arm-erase = Cliquez à nouveau pour effacer et installer

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Choisissez une langue

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Choisissez un clavier

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Choisissez un fuseau horaire

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Choisissez où installer Trindle

# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = Saisissez deux fois le mot de passe du disque, identique les deux fois

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Un identifiant de connexion est nécessaire

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Donnez un nom à cet ordinateur

# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = Saisissez deux fois votre mot de passe, identique les deux fois

# Title of the first page.
installer-welcome-title = Bienvenue dans { $name }

# Sentence under the title of the first page.
installer-welcome-help = Choisissez votre langue ; le clavier et le fuseau horaire la suivent, et vous pouvez changer l'un comme l'autre.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Langue

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Clavier

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Fuseau horaire

# Under a card's head while nothing has been chosen in its list.
installer-select = Sélectionner

# Title of the disk page.
installer-disks-title = Où installer { $name }

# SAFETY: needs a human review before it ships
# Sentence under the title of the disk page.
installer-disks-help = Tout ce qui se trouve sur le disque que vous choisissez est remplacé, sauf si vous faites de la place à côté de l'existant.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Aucun disque assez grand n'a été trouvé. Trindle a besoin de 20 Go.

# SAFETY: needs a human review before it ships
# Name and words of the choice to erase a whole disk.
installer-disks-whole = Utiliser tout le disque, en remplaçant tout ce qu'il contient

# SAFETY: needs a human review before it ships
# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = Garder { $what } et partager le disque

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } conservés pour l'existant, { $mine } pour Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Espace vide

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } et { $last }

# Under a disk's name: what is on it.
installer-disk-has = Contient { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Contient { $list }, chiffré

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Contient un système chiffré

# Under a disk's name: there is nothing on it.
installer-disk-empty = Vide

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Contient des fichiers, mais aucun système que Trindle reconnaît

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = Un fichier d'échange de { $gb } Go

# Name of the switch that makes a swap file.
installer-swap-switch = Créer un fichier d'échange

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Le démarrage sécurisé (Secure Boot) est activé : cet ordinateur ne peut donc pas se mettre en veille prolongée, mais le fichier d'échange aide quand la mémoire manque. Il n'est créé que si le disque a largement de la place en réserve.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = De la place pour la veille prolongée, qui garde tout ouvert avec l'alimentation coupée, et un recours quand la mémoire manque. Sans lui, l'ordinateur ne peut pas se mettre en veille prolongée. Il n'est créé que si le disque a largement de la place en réserve.

# Title of the page about encrypting the disk.
installer-privacy-title = Chiffrement du disque

# Sentence under the title of that page.
installer-privacy-help = Un disque chiffré ne peut pas être lu par quiconque le retire de cet ordinateur.

# Head of the card with the three choices.
installer-privacy-locked = Comment le disque est verrouillé

# SAFETY: needs a human review before it ships
# Name and words of the choice not to encrypt the disk.
installer-privacy-none = Ne pas chiffrer le disque

# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = Le chiffrer, et demander le mot de passe au démarrage de l'ordinateur

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = Le chiffrer, et laisser la puce TPM de cet ordinateur le déverrouiller

# Head of the card about the TPM chip.
installer-privacy-about = À propos de la puce

# SAFETY: needs a human review before it ships
# In the card about the chip, on a computer that has one.
installer-privacy-chip = Cet ordinateur a une puce TPM. Le premier démarrage après l'installation demande une fois le mot de passe, le temps que la puce apprenne cet ordinateur ; ensuite elle déverrouille le disque d'elle-même et rien n'est saisi. Gardez le mot de passe : c'est le seul moyen d'entrer si la puce est remplacée ou si le disque est déplacé vers un autre ordinateur.

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = Cet ordinateur n'a aucune puce TPM que Trindle puisse utiliser : le mot de passe sera donc demandé à chaque démarrage. Sur un ordinateur qui en a une, la puce peut déverrouiller le disque toute seule.

# Title of the disk password page.
installer-disk-title = Le mot de passe du disque

# Sentence under the title of the disk password page.
installer-disk-help = Il est saisi avant toute autre chose au démarrage de l'ordinateur.

# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = Mot de passe du disque

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Suggérer une phrase secrète

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Encore une fois, à l'identique

# Red line under the password boxes when the two do not match.
installer-mismatch = Les deux mots de passe ne correspondent pas.

# SAFETY: needs a human review before it ships
# Head of the card that tells the person to write the password down.
installer-disk-write-down = Notez-le

# SAFETY: needs a human review before it ships
# Words in that card.
installer-disk-write-down-text = Notez ce mot de passe sur papier et gardez-le en lieu sûr, loin de l'ordinateur. Sans lui, le disque ne peut jamais être ouvert, ni par vous ni par personne d'autre. Il n'y a aucun moyen de revenir en arrière. Quatre mots ordinaires forment une phrase secrète facile à saisir et difficile à deviner.

# Title of the account page.
installer-you-title = À propos de vous

# Sentence under the title of the account page.
installer-you-help = C'est le compte avec lequel vous vous connectez.

# Head of the card with the name boxes.
installer-account = Votre compte

# Label and name of the box for the person's name.
installer-your-name = Votre nom

# Label and name of the box for the sign-in name.
installer-user-name = Identifiant de connexion

# Label and name of the box for the computer's name.
installer-host-name = Nom de cet ordinateur

# Head of the card with the password boxes.
installer-your-password = Votre mot de passe

# Label and name of the first password box.
installer-password = Mot de passe

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Difficulté à deviner

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Un identifiant de connexion est nécessaire.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Ce nom est trop long.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Commencez par une lettre minuscule.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Lettres minuscules, chiffres, - et _ uniquement.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Ce nom est déjà utilisé par le système.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Donnez un nom à cet ordinateur.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Lettres, chiffres et - uniquement.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Un ordinateur devinerait ceci immédiatement.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] Un ordinateur devinerait ceci en { $n } minute environ.
        [many] Un ordinateur devinerait ceci en { $n } minutes environ.
       *[other] Un ordinateur devinerait ceci en { $n } minutes environ.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] Un ordinateur devinerait ceci en { $n } heure environ.
        [many] Un ordinateur devinerait ceci en { $n } heures environ.
       *[other] Un ordinateur devinerait ceci en { $n } heures environ.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] Un ordinateur devinerait ceci en { $n } jour environ.
        [many] Un ordinateur devinerait ceci en { $n } jours environ.
       *[other] Un ordinateur devinerait ceci en { $n } jours environ.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] Un ordinateur mettrait { $n } an environ à deviner ceci.
        [many] Un ordinateur mettrait { $n } ans environ à deviner ceci.
       *[other] Un ordinateur mettrait { $n } ans environ à deviner ceci.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Un ordinateur mettrait plus longtemps que toute l'histoire connue à deviner ceci.

# Title of the Ready page.
installer-ready-title = Prêt à installer

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Rien n'a encore été modifié

# Head of the card about security updates.
installer-ready-security = Mises à jour de sécurité

# In the card about security updates.
installer-ready-security-text = { $name } installe tout seul chaque jour les correctifs de sécurité de Debian. Cela ne peut pas être désactivé.

# SAFETY: needs a human review before it ships
# Sentence under the title when a whole disk will be replaced.
installer-ready-replaced = Tout ce qui se trouve sur { $disk } sera remplacé.

# SAFETY: needs a human review before it ships
# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Tout ce qui se trouve sur { $part } est conservé, mais faites d'abord une sauvegarde : la réduire peut faire perdre des fichiers en cas de coupure de courant.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Aucun disque n'a encore été choisi.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the whole disk will be used.
installer-ready-whole = Tout { $disk }

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-whole-text = Tout ce qu'il contient est remplacé par { $name }.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = À côté de { $part }

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-beside-text = { $part } est réduite, jusqu'à { $size }, et { $name } s'installe dans l'espace libéré.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Aucun disque

# Words of that row.
installer-ready-no-disk-text = Aucun disque choisi.

# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = Non chiffré

# Words of that row.
installer-ready-plain-text = Quiconque retire le disque peut le lire.

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Chiffré

# Words of that row when a password is typed at each start.
installer-ready-locked-password = Vous saisissez son mot de passe à chaque démarrage de l'ordinateur.

# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = Le mot de passe une fois, puis la puce TPM de cet ordinateur le déverrouille.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Clavier { $keyboard }, fuseau horaire { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = Installation de { $name }

# Head of the card that holds the short tour.
installer-tour = Une courte visite

# Title of the last page.
installer-done-title = { $name } est installé

# Sentence under the title of the last page.
installer-done-help = Retirez la clé USB, puis redémarrez.

# SAFETY: needs a human review before it ships
# Head of the card that repeats the warning about the disk password.
installer-done-keep = Gardez le mot de passe du disque en lieu sûr

# SAFETY: needs a human review before it ships
# In that card when the TPM chip unlocks the disk.
installer-keep-chip = Le disque est chiffré. Le premier démarrage demande une fois le mot de passe du disque, le temps que la puce apprenne cet ordinateur ; ensuite il s'ouvre tout seul. Notez ce mot de passe sur papier et gardez-le en lieu sûr, loin de cet ordinateur. Sans lui, le disque ne peut jamais être ouvert, ni par vous ni par personne d'autre.

# SAFETY: needs a human review before it ships
# In that card when the password is typed at each start.
installer-keep-password = Le disque est chiffré. Notez dès maintenant le mot de passe du disque sur papier, si ce n'est pas déjà fait, et gardez-le en lieu sûr, loin de cet ordinateur. Sans lui, le disque ne peut jamais être ouvert, ni par vous ni par personne d'autre.

# Title of the first slide.
installer-slide-ring = Tout dans un seul cercle

# Words of the first slide.
installer-slide-ring-text = Déplacez le pointeur au bord de l'écran et double-cliquez : l'anneau s'ouvre, avec vos programmes tout autour, l'heure et les interrupteurs au milieu.

# Title of the second slide.
installer-slide-snapshots = Rétablir ce qu'une mise à jour a cassé

# Words of the second slide.
installer-slide-snapshots-text = Un instantané du système est pris avant et après chaque changement de logiciel. Les paramètres remettent tout le système dans n'importe lequel d'entre eux, et le menu de démarrage peut le faire quand l'ordinateur ne démarre plus.

# Title of the third slide.
installer-slide-apps = Des applications qui ne gênent pas

# Words of the third slide.
installer-slide-apps-text = Applications Flathub, celles de Debian, AppImages et sources de logiciels au même endroit, avec des sauvegardes que vous pouvez emporter sur un autre ordinateur.

# Title of the fourth slide.
installer-slide-yours = À vous, rapidement

# Words of the fourth slide.
installer-slide-yours-text = Clair ou sombre, vos propres sons, et un presse-papiers qui se souvient de ce que vous avez copié.

# Title of the fifth slide.
installer-slide-small = Petit et silencieux

# Words of the fifth slide.
installer-slide-small-text = Le bureau est léger en mémoire et démarre en quelques secondes : un ordinateur portable ancien paraît neuf.

# Title of the tour slide about the ring.
installer-tour-ring = L'anneau, c'est tout le bureau

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Placez le pointeur dans ce coin et l'anneau s'ouvre

# Note on the ring picture, at the time.
installer-tour-ring-time = L'heure et la date, au milieu

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Vos programmes sont placés tout autour

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Son, écran, toutes vos applications, maintien éveillé et extinction

# Title of the tour slide about Settings.
installer-tour-settings = Les paramètres, en mots simples

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Saisissez ici pour trouver un réglage sans le chercher

# Note on the Settings picture, at Network.
installer-tour-settings-network = Réseau, c'est là que vous rejoignez un réseau Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Clair ou sombre, quand vous voulez

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Vingt couleurs, ou saisissez la vôtre

# Title of the tour slide about Apps.
installer-tour-apps = Les applications, pour tout le reste

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Recherchez un programme par son nom ou par ce qu'il fait

# Note on the Apps picture, at the list.
installer-tour-apps-many = Des milliers de programmes, gratuits, de Flathub et de Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Des mises à jour pour tout, y compris Trindle lui-même

# Title of the tour slide about voice control.
installer-tour-voice = Vous pouvez lui parler à la place

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = La commande vocale se trouve ici : Paramètres, puis Anneau

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Appuyez une fois sur Ajouter la voix. Cela représente environ 150 Mo et prend quelques minutes

# Note on the voice picture, about what to say.
installer-tour-voice-say = Touchez ensuite le milieu de l'anneau et dites le nom d'une application, « plus fort » ou « paramètres Wi-Fi »

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Ce que vous dites est compris sur cet ordinateur et ne va nulle part ailleurs

# Title of the tour slide about snapshots.
installer-tour-snapshots = Un retour en arrière, avant que rien ne change

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = L'ordinateur prend un instantané avant chaque mise à jour

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Si quelque chose casse, revenez à l'état précédent depuis le menu de démarrage

# Name of the picture of the ring, and the head of its card.
installer-live-caption = L'anneau, où tout se trouve

# Sentence under the title of the welcome from a stick.
installer-live-help = Vous utilisez la clé USB : rien n'a donc été modifié sur cet ordinateur.

# Head of the card with the three first things to do.
installer-live-begin = Pour commencer

# Name of the first thing to do.
installer-live-open = Ouvrir l'anneau

# Line under it.
installer-live-open-line = Double-cliquez au bord de l'écran, ou appuyez sur Super.

# Name of the second thing to do.
installer-live-speak = Lui parler

# Line under it when voice control is there.
installer-live-speak-line = Touchez le milieu de l'anneau et dites ce que vous voulez.

# Line under it when voice control has to be added.
installer-live-speak-add = Ajoutez la voix dans Paramètres, Anneau, puis touchez le milieu et parlez.

# Name of the third thing to do.
installer-live-tab = Tout dans l'anneau

# Line under it.
installer-live-tab-line = Ce qui est ouvert, vos notifications et le presse-papiers sont tous là.

# The choice of the dark look.
installer-live-dark = Sombre

# The choice of the light look.
installer-live-light = Clair

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Clair ou sombre, pour tout le bureau

# Name of the round button that closes this window and opens the ring.
installer-live-around = Regarder d'abord autour

# Name of the main button that opens the installer.
installer-live-install = Installer { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = Création d'espace

# Foot of the install page while the disk is prepared.
installer-step-prepare = Préparation du disque

# Foot of the install page while a place is made beside another system.
installer-step-place = Création d'une place pour Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = Vérification de ce qui se trouve déjà sur le disque

# Foot of the install page while another system is made smaller.
installer-step-smaller = Réduction de l'autre système

# Foot of the install page while the disk is encrypted.
installer-step-lock = Verrouillage du disque

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Configuration de la puce de l'ordinateur

# Foot of the install page while the system is copied.
installer-step-copy = Copie de Trindle sur le disque

# Foot of the install page while settings are written.
installer-step-settings = Écriture des réglages

# Foot of the install page while the person's account is made.
installer-step-account = Création de votre compte

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Retrait des éléments du système autonome

# Foot of the install page while the start-up files are made.
installer-step-startup = Création des fichiers de démarrage

# Foot of the install page while the boot menu is set up.
installer-step-boot = Configuration du menu de démarrage

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Configuration des instantanés

# Foot of the install page at the very end.
installer-step-finishing = Finalisation

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } n'a pas pu démarrer : { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } ne s'est pas terminé. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = { $path } n'a pas pu être écrit : { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } n'est pas un disque.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } n'est pas un disque de cet ordinateur.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Aucun disque n'a été choisi.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Cet identifiant de connexion n'est pas utilisable par Trindle.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Ce nom d'ordinateur n'est pas utilisable par Trindle.

# Red line at the foot: the language cannot be used.
installer-bad-language = Cette langue n'est pas utilisable par Trindle.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Ce clavier n'est pas utilisable par Trindle.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Ce fuseau horaire n'est pas utilisable par Trindle.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Ce nom contient quelque chose que Trindle ne peut pas écrire.

# Red line at the foot: a password has a line break in it.
installer-bad-password = Un mot de passe ne peut pas contenir de saut de ligne.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = La seconde moitié de l'installateur doit s'exécuter en tant que root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = Ce disque n'a pas de partition de démarrage à partager : Trindle ne peut donc pas s'installer à côté de l'existant.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = La nouvelle partie du disque est introuvable.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = La taille de { $part } n'a pas pu être lue.

# SAFETY: needs a human review before it ships
# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle ne peut pas réduire une partition { $fs }.

# SAFETY: needs a human review before it ships
# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle ne peut pas réduire une partition inconnue.

# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = Un mot de passe est nécessaire pour chiffrer le disque.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Le système n'a pas pu être copié : { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Le système n'a pas pu être copié. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Pas assez de place pour un fichier d'échange.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = L'emplacement du fichier d'échange sur le disque n'a pas pu être lu.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Les nouvelles partitions n'ont pas pu être identifiées.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Le fuseau horaire n'a pas pu être défini : { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Le menu de démarrage n'a pas pu être installé sur cet ordinateur.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Sûr ?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Ce qui est ouvert, le presse-papiers, et un bouton qui vide l'écran

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } est la clé depuis laquelle Trindle fonctionne.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } est utilisé. Fermez ce qui l'utilise, puis réessayez.

# Said beside how far the install has got, once it has started.
installer-started = Installation lancée

# The button that ends an install under way.
installer-halt = Arrêter

# Beside Stop after its first press.
installer-halt-sure = Arrêter l'installation ? L'ancien contenu du disque a déjà disparu.

# While an install is being stopped.
installer-halting = Arrêt en cours

# When Stop could not let go of the disk.
installer-halt-stuck = Le disque n'a pas pu être libéré. Redémarrez l'ordinateur pour réessayer.

# The copy is done and the person is still answering.
installer-step-waiting = Copie terminée. En attente de vos réponses.



installer-already-running = Un autre installateur est déjà en cours sur cet ordinateur.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle a besoin d'au moins 1 Go de mémoire, et cet ordinateur en a moins.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Un disque ne peut être chiffré que sur un ordinateur qui démarre en UEFI, et celui-ci ne le fait pas.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows sur { $part } a été mis en veille prolongée ou laissé en démarrage rapide, donc la réduire pourrait lui faire perdre ses fichiers. Démarrez Windows, choisissez Redémarrer, puis réessayez.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle est installé, mais les instantanés n'ont pas pu être configurés : { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle est installé, mais le menu de démarrage n'a pas pu être réglé pour lister les instantanés : { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Tout n'a pas été configuré

