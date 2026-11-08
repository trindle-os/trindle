
# machine translated (Spanish), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = Usuarios

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = cuentas personas contraseña imagen foto administrador

# Help line at the foot of the Users page.
# SAFETY: needs a human review before it ships
settings-users-help = Añadir personas, cambiar contraseñas y eliminar cuentas necesita la contraseña de un administrador.

# Beside a change that is waiting for the administrator's password to be typed.
settings-users-waiting = Esperando la contraseña...

# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = Aquí nadie es administrador, así que no se puede instalar ni cambiar nada para todos.

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = Solo tú usas este equipo, y eres su administrador.

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = Solo una persona usa este equipo.

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [one] { $n } persona usa este equipo, y tú eres el administrador.
        [many] { $n } personas usan este equipo, y tú eres el administrador.
       *[other] { $n } personas usan este equipo, y tú eres el administrador.
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [one] { $n } persona usa este equipo, y una de ellas es el administrador.
        [many] { $n } personas usan este equipo, y una de ellas es el administrador.
       *[other] { $n } personas usan este equipo, y una de ellas es el administrador.
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [one] { $n } persona usa este equipo, y { $a } son administradores.
        [many] { $n } personas usan este equipo, y { $a } son administradores.
       *[other] { $n } personas usan este equipo, y { $a } son administradores.
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = Eres tú.

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = Su nombre de inicio de sesión.

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = Elegir una imagen

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = Imágenes

# Tip of the round button on a person's card that changes their password.
# SAFETY: needs a human review before it ships
settings-users-change-password = Cambiar la contraseña

# Tip of the round button on a person's card that deletes them and their files.
# SAFETY: needs a human review before it ships
settings-users-delete = Eliminar, con sus archivos

# Said beside a person once they are deleted.
# SAFETY: needs a human review before it ships
settings-users-deleted = Eliminado

# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
# SAFETY: needs a human review before it ships
settings-users-administrator = Administrador

# Under the Administrator line: what an administrator can do.
# SAFETY: needs a human review before it ships
settings-users-admin-can = Puede instalar y cambiar cosas para todos.

# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
settings-users-cannot-take-away = No puedes quitarte esto a ti mismo.

# Said beside a person after they are made an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-now = Ahora es administrador

# Said beside a person after they stop being an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-no-longer = Ya no es administrador

# Help text for a person's card (the information button).
# SAFETY: needs a human review before it ships
settings-users-person-about = Su imagen, su contraseña y, para cualquiera que no seas tú, eliminarlo con sus archivos son los botones redondos junto a su nombre. Un administrador puede instalar programas y cambiar cosas para todos; no puedes quitarte esto a ti mismo. Cada cambio pide la contraseña de un administrador.

# Typing box in the change password form, for your own current password.
# SAFETY: needs a human review before it ships
settings-users-current-password = Contraseña actual

# Typing box in the change password form for the new password.
# SAFETY: needs a human review before it ships
settings-users-new-password = Contraseña nueva

# Typing box in the change password form where the new password is typed again.
settings-users-again = Repetir

# Button in the change password form that saves the new password.
settings-users-save = Guardar

# Spoken name of the Save button in the change password form.
settings-users-save-new = Guardar la contraseña nueva

# Tip of the button that closes a form on the Users page without saving.
# SAFETY: needs a human review before it ships
settings-users-cancel = Cancelar

# Red words in the change password form when your current password was left empty.
settings-users-need-current = También tu contraseña actual, para que tus inicios de sesión guardados sigan funcionando.

# Red words in the change password form when the new password is too short.
settings-users-at-least-8 = Al menos 8 caracteres, por favor.

# Red words in the change password form when the two new passwords differ.
settings-users-no-match = Las dos no coinciden.

# Said beside a person once their password is changed.
settings-users-password-changed = Contraseña cambiada

# Red words after a password change when the saved logins could not take the new password.
settings-users-not-rekeyed = Contraseña cambiada, pero los inicios de sesión guardados siguen abriéndose con la anterior.

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = PNG, JPG o WebP, por favor.

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = No se pudo cambiar la imagen.

# Said beside a person once their picture is saved.
settings-users-saved = Guardado

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = La imagen se muestra al iniciar sesión, pero no se pudo dar a tus otros programas.

# Heading of the form for adding a person.
settings-users-add-someone = Añadir a alguien

# Help text for the Add someone form (the information button).
settings-users-add-about = Una cuenta nueva para otra persona: su nombre completo, un nombre de inicio de sesión con letras minúsculas y números, que empiece por una letra, y una contraseña de al menos 8 caracteres. Puede iniciar sesión enseguida. Un administrador puede instalar programas y cambiar cosas para todos.

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = Cuenta nueva

# Under the New account heading.
settings-users-new-account-sub = Puede iniciar sesión enseguida con esta contraseña.

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = Nombre completo

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = Nombre de inicio de sesión, en minúsculas

# Typing box in the Add someone form for the first password.
# SAFETY: needs a human review before it ships
settings-users-password = Contraseña

# Tip of the button that adds the new person.
# SAFETY: needs a human review before it ships
settings-users-add-this = Añadir a esta persona

# Red words when the login name typed is not allowed.
settings-users-login-rules = Los nombres de inicio de sesión son letras minúsculas y números, y empiezan por una letra.

# Red words when the new person's password is too short.
settings-users-password-8 = La contraseña necesita al menos 8 caracteres.

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } ya existe.

# Said beside the Add someone form once the person is added.
settings-users-added = Añadido

# Heading of the Saved logins card.
settings-users-saved-logins = Inicios de sesión guardados

# Help text for the Saved logins card (the information button).
# SAFETY: needs a human review before it ships
settings-users-keyring-about = Las contraseñas que los programas guardan por ti, por ejemplo para el correo o el Wi-Fi, se guardan en un llavero que se abre con tu inicio de sesión. Todos pasan solos al llavero de Trindle al iniciar sesión. Si el tuyo no se pudo mover, mover copia cada inicio de sesión guardado, pidiendo la contraseña de cualquier llavero que el tuyo no abra, y los archivos de GNOME se dejan como están.

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Guardado en el llavero de Trindle

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = Guardado en el llavero de GNOME

# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = Se abre con tu inicio de sesión.

# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = Tu inicio de sesión no pudo pasarlos.

# Tip of the button that moves saved logins to Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-move-keyring = Mover al llavero de Trindle

# Typing box for your password in the form for moving saved logins.
# SAFETY: needs a human review before it ships
settings-users-your-password = Tu contraseña

# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = Continuar

# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
# SAFETY: needs a human review before it ships
settings-users-password-for = Contraseña de { $name }

# Red words when the move of saved logins was started without a password.
# SAFETY: needs a human review before it ships
settings-users-your-password-please = Tu contraseña, por favor.

# Said beside the Saved logins line while they are copied.
settings-users-copying = Copiando tus inicios de sesión guardados...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n } copiados

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
settings-users-nothing-moved =
    { $n ->
        [one] Todavía no se movió nada: { $names } necesita su propia contraseña.
        [many] Todavía no se movió nada: { $names } necesitan sus propias contraseñas.
       *[other] Todavía no se movió nada: { $names } necesitan sus propias contraseñas.
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = Se detuvo a medias.

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = Revisando las cuentas…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = Personas

# Under the Administrator line of the placeholder card.
settings-users-looking = Buscando…

