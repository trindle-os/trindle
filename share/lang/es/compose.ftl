
# machine translated (Spanish), not yet reviewed by a person.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = Silenciado

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = Micrófono apagado

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = Un programa

# Notice shown when the ring stopped working.
compose-stopped-ring = El anillo se detuvo. Pulsa Super para volver a iniciarlo.

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = La captura de pantalla se detuvo y se cerró.

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = Reintentar

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = La pantalla no está bloqueada.

# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = No hay ningún PIN configurado.

# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = Se acabó el tiempo del PIN.

# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = Demasiados intentos.

# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = Un PIN tiene 4 dígitos.

# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
        [one] PIN incorrecto. Queda 1 intento.
       *[other] PIN incorrecto. Quedan { $left } intentos.
    }
