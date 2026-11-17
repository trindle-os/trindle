
# machine translated (German), not yet reviewed by a person.

# Said under the ring when a radio is switched on by speaking; the radio's name comes first.
voice-radio-on = { $name } an

# Said under the ring when a radio is switched off by speaking; the radio's name comes first.
voice-radio-off = { $name } aus

# Said under the ring after "brighter" turns the screen up.
voice-brighter = Heller

# Said under the ring after "dimmer" turns the screen down.
voice-dimmer = Dunkler

# Said under the ring when the request is to let the screen rest again.
voice-keep-awake-off = Der Bildschirm darf wieder in den Ruhezustand

# Said under the ring when the screen is kept awake.
voice-keep-awake-on = Bleibt wach

# Said under the ring when the request is to look for updates.
voice-updates = Suche nach Updates

# Said under the ring when nothing was heard at all.
voice-nothing-heard = Trindle hat dich nicht gehört. Tippe auf die Mitte des Rings und sprich.

# Said under the ring when the volume goes up.
voice-louder = Lauter

# Said under the ring when the volume goes down.
voice-quieter = Leiser

# Said under the ring when the sound is switched back on.
voice-sound-on = Ton an

# Said under the ring when the sound is muted.
voice-muted = Stumm

# Said under the ring when the wallpaper changes to the next one.
voice-next-wallpaper = Nächstes Hintergrundbild

# Said under the ring after a screenshot is taken; Pictures is the folder's name.
voice-screenshot-saved = Bildschirmfoto in Bilder gespeichert

# Said under the ring when the request is to show every app.
voice-all-apps = Alle Apps

# Said under the ring before shutting down, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-power-off = Herunterfahren? Zum Bestätigen klicken

# Said under the ring before restarting, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-reboot = Neu starten? Zum Bestätigen klicken

# Said under the ring before logging out, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-logout = Abmelden? Zum Bestätigen klicken

# Said under the ring when the request is to find an app in Apps; the words the person said for it come in.
voice-looking-for-app = Suche { $what } in Apps

# Said under the ring when the request opens Apps (the store).
voice-apps = Apps

# Said under the ring when the request opens the disks page of Settings; Drives is the page's name.
voice-drives = Laufwerke

# Said under the ring when the request opens Settings.
voice-settings = Einstellungen

# Said under the ring when the request closes the window in front.
voice-closing-window = Fenster wird geschlossen

# Said under the ring when an open app is closed; the app's name comes in.
voice-closing-app = { $name } wird geschlossen

# Said under the ring when no app has the name the person said; the name they said comes in.
voice-no-app = Keine App namens { $name }

# Said under the ring when a window is brought to the front; the app's name comes in.
voice-showing-app = { $name } wird gezeigt

# Said under the ring when an app is opened; the app's name comes in.
voice-opening-app = { $name } wird geöffnet

# Said under the ring when the request was heard but not understood; what was heard comes in.
voice-heard-unknown = Trindle hat den Befehl „{ $heard }“ nicht erkannt

# Said in the ring when voice has not been added yet.
voice-optional = Sprache ist ein optionales Extra. Füge sie in den Einstellungen auf der Seite „Ring“ hinzu.

# Said in the ring when voice has been added but is switched off.
voice-switched-off = Sprache ist ausgeschaltet. Schalte sie in den Einstellungen auf der Seite „Ring“ ein.

# Said in the ring when the microphone cannot be opened; the reason comes after it.
voice-mic-failed = Trindle konnte nicht auf das Mikrofon zugreifen: { $why }

# Said in the ring when the microphone's recorder would not start.
voice-mic-not-started = Trindle konnte nicht auf das Mikrofon zugreifen, weil das Aufnahmeprogramm nicht gestartet ist

# Said in the ring when listening stopped and nothing was recorded.
voice-nothing-recorded = Es wurde nichts aufgenommen.

# Said in the ring when the recording had no speech in it.
voice-nothing-heard-after = Trindle hat dich nicht gehört. Tippe auf die Mitte des Rings und sprich.

# Said in the ring when the speech engine failed and gave no reason.
voice-whisper-failed = Whisper ist fehlgeschlagen

# Progress line while the build tools are added for the build; the names of the packages come in.
voice-installing-tools = Build-Werkzeuge werden vorübergehend installiert: { $names }

# Progress line while whisper.cpp is built; its version comes in.
voice-building = whisper.cpp { $version } wird für die Sprachsteuerung gebaut. Das dauert ein bis zwei Minuten.

# Progress line while the speech model is downloaded (148 MB).
voice-fetching-model = Sprachmodell wird heruntergeladen (148 MB)

# Progress line while the build tools are taken away again.
voice-removing-tools = Die Build-Werkzeuge werden jetzt entfernt

# Said once voice is added and ready to use.
voice-ready = Die Sprachsteuerung ist bereit. Öffne den Ring, tippe auf die Mitte und sprich.

# Said once voice is removed.
voice-removed = Sprachsteuerung entfernt.

# Shown in the terminal window that stays open, asking for Enter to close it.
voice-press-enter = Zum Schließen die Eingabetaste drücken.

# Shown when the command is run with neither install nor remove.
voice-usage = Aufruf: trindle-voice-setup install | remove

# Said when a step of the voice set-up fails. { $program } is the program that was run, with its first word.
voice-step-failed = { $program } hat nicht funktioniert.

# Said when the whisper.cpp download isn't the one Trindle checks for, so nothing is built from it.
voice-whisper-fingerprint = Der Download von whisper.cpp ist nicht der, den Trindle erwartet hat. Deshalb wurde nichts daraus gebaut.

# Said when the speech model download isn't the one Trindle checks for, so it isn't installed.
voice-model-fingerprint = Der Download des Sprachmodells ist nicht der, den Trindle erwartet hat. Deshalb wurde es nicht installiert.

# The reason after "Trindle couldn't access the microphone:" when the session has no private folder to keep a recording in.
voice-no-private-folder = kein privater Ordner verfügbar.
