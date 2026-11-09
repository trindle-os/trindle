
# machine translated (German), not yet reviewed by a person.

# Window title when the keyring is started for the first time, with no keyring yet.
keyring-title-start = Schlüsselbund starten

# Window title when the keyring is there and needs its password to open.
# SAFETY: needs a human review before it ships
keyring-title-unlock = Schlüsselbund entsperren

# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
keyring-password = Passwort

# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
keyring-unlock = Entsperren

# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
keyring-cancel = Abbrechen

# Shown under the password box when Unlock is pressed with the box empty.
# SAFETY: needs a human review before it ships
keyring-type-first = Gib zuerst das Passwort ein

# Shown under the password box while the password is being checked.
keyring-checking = Wird geprüft …

# Under the title when no keyring exists yet and a program wants to keep a password.
# SAFETY: needs a human review before it ships
keyring-why-none = Ein Programm möchte ein Passwort sicher aufbewahren. Gib dein Anmeldepasswort ein, um den Schlüsselbund zu starten. Künftig öffnet er sich bei der Anmeldung von selbst.

# Under the title when the keyring was made with a different password from the login one.
# SAFETY: needs a human review before it ships
keyring-why-old = Dein Anmeldepasswort ist nicht das, mit dem dieser Schlüsselbund angelegt wurde. Gib einmal das alte ein. Danach öffnet er sich bei der Anmeldung von selbst.

# Under the title when no password was typed at log in, for example with a fingerprint or automatic log in.
# SAFETY: needs a human review before it ships
keyring-why-no-login = Bei der Anmeldung wurde kein Passwort eingegeben (Fingerabdruck oder automatische Anmeldung), daher konnte sich der Schlüsselbund nicht von selbst öffnen. Gib dein Anmeldepasswort ein.

# Under the title when a program asks for a saved password and the keyring is shut.
# SAFETY: needs a human review before it ships
keyring-why-wanted = Ein Programm möchte ein gespeichertes Passwort. Gib dein Anmeldepasswort ein, um den Schlüsselbund zu entsperren.

# Shown under the box when the keyring has stopped before it could check the password.
keyring-stopped = Der Schlüsselbund wurde angehalten

# Shown under the box when the password does not open the keyring.
# SAFETY: needs a human review before it ships
keyring-not-the-keyring-password = Das ist nicht das Passwort des Schlüsselbunds

# Shown under the box when the keyring file is damaged and has been left as it is.
keyring-damaged-left = Die Schlüsselbund-Datei ist beschädigt und wurde unverändert gelassen

# Shown under the box when the password is not the person's login password.
# SAFETY: needs a human review before it ships
keyring-not-login-password = Das ist nicht dein Anmeldepasswort

# Shown in Settings when the password typed there is not the person's password.
# SAFETY: needs a human review before it ships
keyring-not-your-password = Das ist nicht dein Passwort.

# Shown in Settings when the old keyring is still running after its time to stop has run out.
keyring-would-not-stop = Der alte Schlüsselbund ließ sich nicht beenden.

# Said when the keyring is locked and cannot be used.
# SAFETY: needs a human review before it ships
keyring-error-locked = Der Schlüsselbund ist gesperrt

# Said when the password given does not open the keyring.
# SAFETY: needs a human review before it ships
keyring-error-wrong-password = Dieses Passwort öffnet den Schlüsselbund nicht

# Said when the collection or item asked for is not there.
keyring-error-no-such = Keine solche Sammlung und kein solches Element

# Said when the keyring file cannot be read as a keyring.
keyring-error-damaged = Die Schlüsselbund-Datei ist beschädigt


# Under the reason, naming the program that asked, when the keyring can find its name.
keyring-asked-by = Angefragt von { $program }

# Told to a program that asked while the keyring was taking over from the one before and many calls already waited.
keyring-starting = Der Schlüsselbund startet. Frage erneut an.

# The program in "Asked for by", for a program on the bus: $name is the name it gives itself, which anything can claim.
keyring-named-itself = ein Programm, das sich { $name } nennt

