# machine translated (French), not yet reviewed by a person.

# Name of the Settings page about event sounds, in the list and as its heading.
settings-sounds-title = Sons

# Words a person might type into the Settings search to find the sounds page; separate them with spaces.
settings-sounds-keywords = son sons carillon démarrage connexion ouverture session déconnexion extinction arrêt chargeur branché débranché batterie alerte bip désactivé silence silencieux personnel perso ajouter fichier wav ogg mp3

# Introduction under the heading of the sounds page, and the words behind the page i.
settings-sounds-help = Choisissez un son pour chaque moment, par exemple l'ouverture de session. Vous pouvez aussi ajouter vos propres sons.

# First choice in each moment's list of sounds: that moment makes no sound.
settings-sounds-off = Désactivé

# Sentence at the top of the page when no moment makes a sound.
settings-sounds-top-none = Tout est silencieux.

# Sentence at the top of the page when every moment makes a sound.
settings-sounds-top-all = Tous les sons sont activés.

# Sentence at the top of the page when some moments make a sound; { $n } is how many of the { $total } do.
settings-sounds-top-some =
    { $n ->
        [one] { $n } son sur { $total } est activé. Les autres sont désactivés.
       *[other] { $n } sons sur { $total } sont activés. Les autres sont désactivés.
    }

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-open = Ouverture de Trindle

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-close = Fermeture de Trindle

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunrise = Lever du soleil

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunset = Coucher du soleil

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-rising = Montant

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-falling = Descendant

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-alert = Alerte

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-bell = Cloche

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-glass = Verre

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-pop = Pop

# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-login = Ouverture de session

# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-logout = Fermeture de session, redémarrage et extinction

# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-plug = Chargeur branché

# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-unplug = Chargeur débranché

# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-battery = Batterie faible

# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-notification = Une notification apparaît

# Heading of the logging out line.
settings-sounds-moment-logout-short = Fermeture de session

# Heading of the notifications line.
settings-sounds-moment-notification-short = Notifications

# Small words under the logging out line.
settings-sounds-moment-logout-about = Aussi au redémarrage et à l'extinction.

# Heading and slider name for the volume of the logging in and out sounds.
settings-sounds-volume-coming = Volume de l'ouverture et de la fermeture de session

# Heading and slider name for the volume of notification sounds.
settings-sounds-volume-notifications = Volume des notifications

# Heading and slider name for the volume of the charger and battery sounds.
settings-sounds-volume-power = Volume des sons d'alimentation

# Name of the round button that plays the sound chosen for a moment, or an added sound.
settings-sounds-hear = Écouter

# Name of the button that opens a window to add sound files.
settings-sounds-add = Ajouter des fichiers son

# Name of the button that opens the folder where added sounds are kept.
settings-sounds-open-folder = Ouvrir le dossier des sons ajoutés

# Red words under the add sounds line when the folder could not be opened.
settings-sounds-folder-not-opened = Impossible d'ouvrir le dossier.

# Red words under the add sounds line when the folder for added sounds could not be made.
settings-sounds-folder-not-made = Trindle n'a pas pu créer le dossier de vos sons.

# Name of the button that plays the sound being added, when one of the same name is already there.
settings-sounds-hear-new = Écouter le nouveau son

# Name of the button that plays the sound already there, when one of the same name is being added.
settings-sounds-hear-old = Écouter le son existant

# Heading of the line for adding sounds.
settings-sounds-add-heading = Ajouter des sons

# Name of the bin button that removes a sound added by the person.
settings-sounds-remove = Retirer ce son

# Heading of the line shown when no sounds have been added.
settings-sounds-none-yet = Aucun ajouté pour l'instant

# Red words under a moment's line when its chosen sound file is gone.
settings-sounds-gone = Ce son n'est plus sur cet ordinateur. Choisissez-en un autre.

# Red words under a line when a sound would not play.
settings-sounds-would-not-play = Le son n'a pas été joué. Vérifiez vos haut-parleurs dans Audio.

# Title of the window for choosing sound files.
settings-sounds-chooser-title = Choisir des fichiers son

# Name of the filter in the window for choosing sound files that shows only sounds.
settings-sounds-chooser-filter = Sons

# Red words under the add sounds line when a chosen file is not a sound; { $name } is the file name.
settings-sounds-not-playable = Trindle ne peut pas jouer { $name }, car il ne reconnaît pas le type de fichier.

# Red words under the add sounds line when a file could not be copied in; { $name } is the file name.
settings-sounds-not-copied = Impossible de copier { $name }.

# Green words under the add sounds line after sounds were added; { $n } is how many.
settings-sounds-added =
    { $n ->
        [one] { $n } son ajouté. Vous pouvez maintenant le choisir pour n'importe quel moment ci-dessus.
       *[other] { $n } sons ajoutés. Vous pouvez maintenant les choisir pour n'importe quel moment ci-dessus.
    }

# Red words under the question about a sound of the same name when it could not be replaced; { $name } is the file name.
settings-sounds-not-replaced = Impossible de remplacer { $name }.

# Green words after one sound took the place of the one there; { $name } is the file name.
settings-sounds-replaced-one = { $name } remplacé.

# Green words after several sounds took the place of the ones there; { $n } of { $total } were done.
settings-sounds-replaced-some = { $n } sons remplacés sur { $total }.

# Red words under a sound's line when it could not be removed.
settings-sounds-not-removed = Impossible de le retirer.

# Red words under a moment's line when its sound was removed and the moment could not be set back to its own sound.
settings-sounds-not-set-back = Trindle a retiré le son, mais n'a pas pu remettre le son habituel pour ce moment.

# Title of the card with the sounds for logging in and out and notifications.
settings-sounds-coming-title = Ouverture et fermeture de session

# Words behind the i of that card.
settings-sounds-coming-about = Chaque volume va de 0 à 100 par pas de dix. Vous entendez le son une fois quand vous relâchez.

# Title of the card with the charger and battery sounds.
settings-sounds-power-title = Alimentation

# Words behind the i of that card.
settings-sounds-power-about = Les sons du branchement et du débranchement du chargeur, et de la batterie faible. Le volume va de 0 à 100 par pas de dix. Vous entendez le son une fois quand vous relâchez.

# Title of the card with the sounds the person added.
settings-sounds-own-title = Vos propres sons

# Words behind the i of that card.
settings-sounds-own-about = WAV, OGG, Opus, FLAC ou MP3. Trindle les copie dans votre propre dossier, vous pouvez donc déplacer ou supprimer les originaux. Vous pouvez ensuite les choisir pour n'importe quel moment, comme les autres. Si vous en retirez un, tout moment qui l'utilisait retrouve son son habituel.
