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
polkit-not-accepted = Essa senha não está certa. Tente de novo.

# Name a screen reader speaks for the password question window.
polkit-spoken = Sua senha

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Outro pedido está esperando.
       *[other] { $n } outros pedidos estão esperando.
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Unidades precisa da senha de um administrador para fazer um instantâneo antes de liberar espaço.

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Unidades precisa da senha de um administrador para desligar o arquivo de swap enquanto libera espaço.

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Unidades precisa da senha de um administrador para ligar o arquivo de swap de novo.
