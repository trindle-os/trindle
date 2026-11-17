
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
polkit-not-accepted = Ce mot de passe n'est pas le bon. Réessayez.

# Name a screen reader speaks for the password question window.
polkit-spoken = Votre mot de passe

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Une autre demande attend.
       *[other] { $n } autres demandes attendent.
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Disques a besoin du mot de passe d'un administrateur pour faire un instantané avant de libérer de la place.

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Disques a besoin du mot de passe d'un administrateur pour désactiver le fichier d'échange pendant qu'il libère de la place.

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Disques a besoin du mot de passe d'un administrateur pour réactiver le fichier d'échange.
