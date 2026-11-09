# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = A senha do administrador é necessária

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = Senha de { $user }

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = Permitir

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = Cancelar

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = Essa senha não foi aceita. Tente de novo.

# Name a screen reader speaks for the password question window.
polkit-spoken = Autenticação

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Outro pedido está esperando por este.
        [many] { $n } pedidos a mais estão esperando por este.
       *[other] { $n } pedidos a mais estão esperando por este.
    }
