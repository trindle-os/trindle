# machine translated (German), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
# SAFETY: needs a human review before it ships
key-enrol-title = Den Schlüssel eines Treibers beim Neustart bestätigen

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol schlüssel blauer bildschirm passwort kennwort treiber nvidia virtualbox kamera dkms mokutil schwarzer bildschirm

# First paragraph before the steps when the driver is NVIDIA's own.
# SAFETY: needs a human review before it ships
key-enrol-why-nvidia = Der eigene Treiber von NVIDIA wird auf diesem Computer gebaut. Secure Boot lädt ihn deshalb erst, wenn du seinen Schlüssel bestätigst. Tu das vor dem Neustart, sonst bleibt der Bildschirm eventuell schwarz.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
# SAFETY: needs a human review before it ships
key-enrol-why-addon = Ein Zusatztreiber wird auf diesem Computer gebaut. Secure Boot lädt ihn deshalb erst, wenn du seinen Schlüssel bestätigst. Tu das vor dem Neustart.

# Step 1: the password, made before the restart.
# SAFETY: needs a human review before it ships
key-enrol-step1 = Denke dir ein Passwort aus, das nur aus Buchstaben und Ziffern besteht. Der blaue Bildschirm hat eine amerikanische Tastatur, Sonderzeichen liegen dort daher auf anderen Tasten. Schreib es auf; du brauchst es nur einmal.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Starte neu. Vor dem Start von Trindle erscheint ein blauer Bildschirm mit der Überschrift Enroll MOK. Er gibt nach etwa 10 Sekunden auf, sei also bereit, eine Taste zu drücken.

# Step 3: the first two screens.
key-enrol-step3 = Wähle Enroll MOK, dann Continue.

# Step 4: the third screen and the password.
# SAFETY: needs a human review before it ships
key-enrol-step4 = Wähle Yes und gib das Passwort ein, das du dir ausgedacht hast.

# Step 5: the last screen.
key-enrol-step5 = Wähle Reboot. Der Treiber wird geladen, sobald der Computer wieder läuft.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Wenn du den blauen Bildschirm verpasst, startet der Computer wie bisher ohne den Treiber. Nichts ist kaputt; führe sudo mokutil --import /var/lib/dkms/mok.pub aus und starte neu, um erneut gefragt zu werden.
