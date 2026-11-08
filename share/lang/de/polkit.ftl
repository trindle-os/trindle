
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
polkit-not-accepted = Das Passwort wurde nicht akzeptiert. Versuche es noch einmal.

# Name a screen reader speaks for the password question window.
polkit-spoken = Authentifizierung

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Eine weitere Anfrage wartet auf diese.
       *[other] { $n } weitere Anfragen warten auf diese.
    }
