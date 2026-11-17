
# machine translated (French), not yet reviewed by a person.

# Said under the ring when a radio is switched on by speaking; the radio's name comes first.
voice-radio-on = { $name } activé

# Said under the ring when a radio is switched off by speaking; the radio's name comes first.
voice-radio-off = { $name } désactivé

# Said under the ring after "brighter" turns the screen up.
voice-brighter = Plus lumineux

# Said under the ring after "dimmer" turns the screen down.
voice-dimmer = Plus sombre

# Said under the ring when the request is to let the screen rest again.
voice-keep-awake-off = L'écran peut se mettre en veille de nouveau

# Said under the ring when the screen is kept awake.
voice-keep-awake-on = Reste éveillé

# Said under the ring when the request is to look for updates.
voice-updates = Recherche de mises à jour

# Said under the ring when nothing was heard at all.
voice-nothing-heard = Trindle n'a rien entendu. Touchez le milieu de l'anneau et parlez.

# Said under the ring when the volume goes up.
voice-louder = Plus fort

# Said under the ring when the volume goes down.
voice-quieter = Plus doux

# Said under the ring when the sound is switched back on.
voice-sound-on = Son activé

# Said under the ring when the sound is muted.
voice-muted = Muet

# Said under the ring when the wallpaper changes to the next one.
voice-next-wallpaper = Fond d'écran suivant

# Said under the ring after a screenshot is taken; Pictures is the folder's name.
voice-screenshot-saved = Capture d'écran enregistrée dans Pictures

# Said under the ring when the request is to show every app.
voice-all-apps = Toutes les applications

# SAFETY: needs a human review before it ships
# Said under the ring before shutting down, which waits for a click to confirm.
voice-confirm-power-off = Éteindre ? Cliquez pour confirmer

# SAFETY: needs a human review before it ships
# Said under the ring before restarting, which waits for a click to confirm.
voice-confirm-reboot = Redémarrer ? Cliquez pour confirmer

# SAFETY: needs a human review before it ships
# Said under the ring before logging out, which waits for a click to confirm.
voice-confirm-logout = Se déconnecter ? Cliquez pour confirmer

# Said under the ring when the request is to find an app in Apps; the words the person said for it come in.
voice-looking-for-app = Recherche de { $what } dans Apps

# Said under the ring when the request opens Apps (the store).
voice-apps = Apps

# Said under the ring when the request opens the disks page of Settings; Drives is the page's name.
voice-drives = Drives

# Said under the ring when the request opens Settings.
voice-settings = Réglages

# Said under the ring when the request closes the window in front.
voice-closing-window = Fermeture de la fenêtre

# Said under the ring when an open app is closed; the app's name comes in.
voice-closing-app = Fermeture de { $name }

# Said under the ring when no app has the name the person said; the name they said comes in.
voice-no-app = Aucune application nommée { $name }

# Said under the ring when a window is brought to the front; the app's name comes in.
voice-showing-app = Affichage de { $name }

# Said under the ring when an app is opened; the app's name comes in.
voice-opening-app = Ouverture de { $name }

# Said under the ring when the request was heard but not understood; what was heard comes in.
voice-heard-unknown = Trindle n'a pas reconnu la commande « { $heard } »

# Said in the ring when voice has not been added yet.
voice-optional = La voix est un extra facultatif. Ajoutez-la dans Réglages, sur la page Anneau.

# Said in the ring when voice has been added but is switched off.
voice-switched-off = La voix est désactivée. Activez-la dans Réglages, sur la page Anneau.

# Said in the ring when the microphone cannot be opened; the reason comes after it.
voice-mic-failed = Trindle n'a pas pu accéder au microphone : { $why }

# Said in the ring when the microphone's recorder would not start.
voice-mic-not-started = Trindle n'a pas pu accéder au microphone car le programme d'enregistrement n'a pas démarré

# Said in the ring when listening stopped and nothing was recorded.
voice-nothing-recorded = Rien n'a été enregistré.

# Said in the ring when the recording had no speech in it.
voice-nothing-heard-after = Trindle n'a rien entendu. Touchez le milieu de l'anneau et parlez.

# Said in the ring when the speech engine failed and gave no reason.
voice-whisper-failed = Whisper a échoué

# Progress line while the build tools are added for the build; the names of the packages come in.
voice-installing-tools = Installation temporaire des outils de compilation : { $names }

# Progress line while whisper.cpp is built; its version comes in.
voice-building = Compilation de whisper.cpp { $version } pour la commande vocale. Cela prend une minute ou deux.

# Progress line while the speech model is downloaded (148 MB).
voice-fetching-model = Téléchargement du modèle de parole (148 Mo)

# Progress line while the build tools are taken away again.
voice-removing-tools = Suppression des outils de compilation

# Said once voice is added and ready to use.
voice-ready = La commande vocale est prête. Ouvrez l'anneau, touchez le milieu, et parlez.

# Said once voice is removed.
voice-removed = Commande vocale supprimée.

# Shown in the terminal window that stays open, asking for Enter to close it.
voice-press-enter = Appuyez sur Entrée pour fermer.

# Shown when the command is run with neither install nor remove.
voice-usage = utilisation : trindle-voice-setup install | remove

# Said when a step of the voice set-up fails. { $program } is the program that was run, with its first word.
voice-step-failed = { $program } n'a pas fonctionné.

# Said when the whisper.cpp download isn't the one Trindle checks for, so nothing is built from it.
voice-whisper-fingerprint = Le téléchargement de whisper.cpp n'est pas celui que Trindle attendait, il n'a donc pas été compilé.

# Said when the speech model download isn't the one Trindle checks for, so it isn't installed.
voice-model-fingerprint = Le téléchargement du modèle vocal n'est pas celui que Trindle attendait, il n'a donc pas été installé.

# The reason after "Trindle couldn't access the microphone:" when the session has no private folder to keep a recording in.
voice-no-private-folder = aucun dossier privé n'est disponible.
