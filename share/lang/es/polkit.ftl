
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card asking for the administrator's password.
polkit-title = Se necesita la contraseña de administrador

# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = Contraseña de { $user }

# SAFETY: needs a human review before it ships
# Button that lets the step go ahead once the password is typed.
polkit-allow = Permitir

# SAFETY: needs a human review before it ships
# Button that stops the step and closes the question.
polkit-cancel = Cancelar

# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = No se aceptó esa contraseña. Inténtalo de nuevo.

# Name a screen reader speaks for the password question window.
polkit-spoken = Autenticación

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Hay otra solicitud esperando a que termine esta.
       *[other] Hay { $n } solicitudes más esperando a que termine esta.
    }

