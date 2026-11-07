
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card when the PIN is the way in.
session-lock-enter-pin = Introduce tu PIN

# Heading of the card when the password is the way in.
session-lock-locked = Bloqueado

# SAFETY: needs a human review before it ships
# Under the heading of the password card: what to do.
session-lock-type-password = Escribe tu contraseña y pulsa Intro

# SAFETY: needs a human review before it ships
# Button under the PIN card that changes to typing the password.
session-lock-use-password = Usar contraseña

# SAFETY: needs a human review before it ships
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = Usar PIN

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = Usar el teléfono

# Shown while the typed password or PIN is being checked.
session-lock-checking = Comprobando...

# SAFETY: needs a human review before it ships
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = El PIN ha caducado; escribe tu contraseña.

# SAFETY: needs a human review before it ships
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = Demasiados PIN incorrectos; escribe tu contraseña.

# SAFETY: needs a human review before it ships
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [one] PIN incorrecto. Te queda { $n } intento.
        [many] PIN incorrecto. Te quedan { $n } de intentos.
       *[other] PIN incorrecto. Te quedan { $n } intentos.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Consultando a tu teléfono...

# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Tu teléfono no lo desbloqueó. Usa tu PIN.

# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Tu teléfono no lo desbloqueó. Escribe tu contraseña.

# SAFETY: needs a human review before it ships
# Shown when the password typed is not the person's password.
session-lock-wrong-password = Esa contraseña no es correcta.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = Un momento, por favor

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = Esta imagen es más grande que tu pantalla y usa memoria adicional.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Los fondos animados usan más memoria y batería.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = mayor que 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = mayor que tu pantalla

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = más de 20 MB (200 MB si es animado)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = más de 20 MB

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = más de 200 MB

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } o { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count } omitidos: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = Eso tardó demasiado. Inténtalo de nuevo.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = ¿Reiniciar todo el escritorio?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Se cerrarán todas las ventanas abiertas y se perderá lo que no esté guardado en ellas.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = Cancelar

# Main button that restarts the whole desktop.
session-restart-go = Reiniciar

