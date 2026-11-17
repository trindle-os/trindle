
# machine translated (German), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = Administratorpasswort erforderlich

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = Passwort für { $user }

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = Erlauben

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = Abbrechen

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = Das Passwort war nicht richtig. Versuche es noch einmal.

# Name a screen reader speaks for the password question window.
polkit-spoken = Dein Passwort

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Eine weitere Anfrage wartet.
       *[other] { $n } weitere Anfragen warten.
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Laufwerke braucht das Passwort eines Administrators, um vor dem Platzschaffen einen Schnappschuss zu erstellen.

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Laufwerke braucht das Passwort eines Administrators, um die Auslagerungsdatei beim Platzschaffen auszuschalten.

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Laufwerke braucht das Passwort eines Administrators, um die Auslagerungsdatei wieder einzuschalten.
