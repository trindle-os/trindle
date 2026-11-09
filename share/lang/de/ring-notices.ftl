# machine translated (German), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Akku bei { $pct } %

# Line under the low battery warning's title.
ring-battery-plug-in = Schließe bald das Ladegerät an.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = Wird geladen

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Ladegerät getrennt

# Line under the title when the charger comes out.
ring-battery-running = Läuft mit Akku, bei { $pct } %

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 Minuten

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 Stunde

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 Stunden

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = Nächster Morgen

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = 10 Minuten schlummern

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = 1 Stunde schlummern

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = 6 Stunden schlummern

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Bis zum nächsten Morgen schlummern

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = Zurück zur Nachricht

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Lösen

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Anheften

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Schlummern

# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = Alle entfernen

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Kopiert

# Tip of the button on a notification circle that copies its message.
ring-act-copy = Nachricht kopieren

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Klicke, um es deinem Assistenten zu übergeben.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = Der Desktop von Trindle

# Name of the login screen in the crash notice.
ring-crash-greeter = Der Anmeldebildschirm von Trindle

# Name of the lock screen in the crash notice.
ring-crash-lock = Der Sperrbildschirm von Trindle

# Name of Settings in the crash notice.
ring-crash-settings = Trindle Einstellungen

# Name of Apps in the crash notice.
ring-crash-apps = Die Apps von Trindle

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = Trindle { $name }

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } wurde unerwartet beendet

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = Dieses Laufwerk ist nicht mehr da

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = { $name } konnte nicht geöffnet werden

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Ruhezustand

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Tiefschlaf

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Bildschirm sperren

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Nichts

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Herunterfahren

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = Beim Schließen des Deckels geschieht, was unter Einstellungen, Energie steht

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Halte irgendwo gedrückt, um zu vergrößern

# Screen reader name of the magnifier's glass.
ring-magnify-name = Lupe

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Vergrößerung beenden

# Tip of the switch in the ring that turns the microphone off for every program.
# SAFETY: needs a human review before it ships
ring-mic-off-tip = Mikrofon für alle Programme ausschalten

# Tip of the switch in the ring that turns the microphone back on.
# SAFETY: needs a human review before it ships
ring-mic-on-tip = Mikrofon wieder einschalten

# Line in the ring when something other than the person pressed the microphone switch on.
# SAFETY: needs a human review before it ships
ring-mic-own-press = Nur dein eigener Tastendruck kann das Mikrofon wieder einschalten.

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = Der Mikrofonschalter konnte nicht geändert werden: { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = Kein Programm gefunden

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Von einem anderen Programm gesendet

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Konnte nicht gestartet werden

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Für deinen Assistenten kopiert

# Screen reader name of a notification circle.
ring-notes-name = Benachrichtigung

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Wird bewegt. Loslassen zum Ablegen, Escape zum Zurücklegen.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Verschoben

# Said when Escape put a held notification back.
ring-notes-put-back = Zurückgelegt

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Hefte sie an, um sie zu verschieben

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
        [one] { $n } Sekunde
       *[other] { $n } Sekunden
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Bis du sie schließt

# Time under a notification from yesterday.
ring-notify-yesterday = Gestern { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = Unten rechts

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = Unten links

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = Oben links

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = Oben rechts

# Title of the notice when a plugin that asks for permissions has been given none.
# SAFETY: needs a human review before it ships
ring-plugin-needs = { $name } braucht deine Erlaubnis

# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
# SAFETY: needs a human review before it ships
ring-plugin-cannot-work = Es kann noch nicht arbeiten: 0 von { $asks } erlaubt.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Kopiert. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = Hat das Netzwerk erreicht

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = Hat deine Dateien geöffnet

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
        [one] { $names } und { $more } weitere
       *[other] { $names } und { $more } weitere
    }

