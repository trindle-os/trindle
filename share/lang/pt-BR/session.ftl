
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Heading of the card when the PIN is the way in.
session-lock-enter-pin = Digite seu PIN

# Heading of the card when the password is the way in.
session-lock-locked = Bloqueado

# Under the heading of the password card: what to do.
session-lock-type-password = Digite sua senha e pressione Enter

# Button under the PIN card that changes to typing the password.
session-lock-use-password = Usar senha

# Button under the password card that changes to typing a PIN.
session-lock-use-pin = Usar PIN

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = Usar celular

# Shown while the typed password or PIN is being checked.
session-lock-checking = Verificando...

# SAFETY: needs a human review before it ships
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = O PIN expirou; digite sua senha.

# SAFETY: needs a human review before it ships
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = PINs incorretos demais; digite sua senha.

# SAFETY: needs a human review before it ships
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [one] PIN incorreto. Resta { $n } tentativa.
        [many] PIN incorreto. Restam { $n } tentativas.
       *[other] PIN incorreto. Restam { $n } tentativas.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Consultando seu celular...

# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Seu celular não desbloqueou. Use seu PIN.

# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Seu celular não desbloqueou. Digite sua senha.

# SAFETY: needs a human review before it ships
# Shown when the password typed is not the person's password.
session-lock-wrong-password = Essa senha não está correta.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = Um momento, por favor

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = Esta imagem é maior que a sua tela e usa memória extra.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Papéis de parede animados usam mais memória e bateria.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = maior que 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = maior que a sua tela

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = acima de 20 MB (200 MB se animado)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = acima de 20 MB

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = acima de 200 MB

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } ou { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count } deixados de fora: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = Demorou demais. Tente novamente.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = Reiniciar a área de trabalho inteira?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Todas as janelas abertas serão fechadas, e o que nelas não foi salvo será perdido.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = Cancelar

# Main button that restarts the whole desktop.
session-restart-go = Reiniciar

