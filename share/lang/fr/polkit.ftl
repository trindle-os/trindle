
# machine translated (French), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card asking for the administrator's password.
polkit-title = Mot de passe administrateur requis

# SAFETY: needs a human review before it ships
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = Mot de passe de { $user }

# SAFETY: needs a human review before it ships
# Button that lets the step go ahead once the password is typed.
polkit-allow = Autoriser

# SAFETY: needs a human review before it ships
# Button that stops the step and closes the question.
polkit-cancel = Annuler

# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = Ce mot de passe n'a pas été accepté. Réessayez.

# Name a screen reader speaks for the password question window.
polkit-spoken = Authentification

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Une autre demande attend celle-ci.
       *[other] { $n } autres demandes attendent celle-ci.
    }
