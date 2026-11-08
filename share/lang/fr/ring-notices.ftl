
# machine translated (French), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Batterie à { $pct } %

# Line under the low battery warning's title.
ring-battery-plug-in = Branchez bientôt le chargeur.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = En charge

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Chargeur débranché

# Line under the title when the charger comes out.
ring-battery-running = Fonctionne sur batterie, à { $pct } %

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 minutes

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 heure

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 heures

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = Demain matin

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = Reporter de 10 minutes

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = Reporter d'1 heure

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = Reporter de 6 heures

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Reporter à demain matin

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = Retour au message

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Détacher

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Épingler

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Reporter

# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = Tout effacer

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Copié

# Tip of the button on a notification circle that copies its message.
ring-act-copy = Copier le message

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Cliquez pour le transmettre à votre assistant.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = Le bureau de Trindle

# Name of the login screen in the crash notice.
ring-crash-greeter = L'écran de connexion de Trindle

# Name of the lock screen in the crash notice.
ring-crash-lock = L'écran de verrouillage de Trindle

# Name of Settings in the crash notice.
ring-crash-settings = Les paramètres de Trindle

# Name of Apps in the crash notice.
ring-crash-apps = Les applications de Trindle

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = { $name } de Trindle

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } s'est arrêté de manière inattendue

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = Ce disque n'est plus là

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = { $name } n'a pas pu être ouvert

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Veille

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Veille prolongée

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Verrouiller l'écran

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Rien

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Éteindre

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = La fermeture du couvercle fait ce qu'indiquent les paramètres, Alimentation

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Maintenez n'importe où pour agrandir

# Screen reader name of the magnifier's glass.
ring-magnify-name = Loupe

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Arrêter l'agrandissement

# SAFETY: needs a human review before it ships
# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = Couper le microphone pour tous les programmes

# SAFETY: needs a human review before it ships
# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = Réactiver le microphone

# SAFETY: needs a human review before it ships
# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = Seul votre propre appui peut réactiver le microphone.

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = L'interrupteur du microphone n'a pas pu être modifié : { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = Aucun programme trouvé

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Envoyée par un autre programme

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Impossible de le lancer

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Copié pour votre assistant

# Screen reader name of a notification circle.
ring-notes-name = Notification

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Déplacement en cours. Relâchez pour déposer, Échap pour remettre en place.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Déplacée

# Said when Escape put a held notification back.
ring-notes-put-back = Remise en place

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Épinglez-la pour la déplacer

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
        [one] { $n } seconde
        [many] { $n } secondes
       *[other] { $n } secondes
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Jusqu'à sa fermeture

# Time under a notification from yesterday.
ring-notify-yesterday = Hier { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = En bas à droite

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = En bas à gauche

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = En haut à gauche

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = En haut à droite

# SAFETY: needs a human review before it ships
# Title of the notice when a plugin that asks for permissions has been given none.
ring-plugin-needs = { $name } a besoin de votre autorisation

# SAFETY: needs a human review before it ships
# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
ring-plugin-cannot-work = Il ne peut pas encore fonctionner : 0 autorisation sur { $asks } accordée.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Copié. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = A accédé au réseau

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = A vos fichiers ouverts

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
        [one] { $names } et un autre
       *[other] { $names } et { $more } autres
    }

