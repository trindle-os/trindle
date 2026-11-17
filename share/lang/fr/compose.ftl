# machine translated (French), not yet reviewed by a person.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = Son coupé

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = Microphone coupé

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = Un programme

# Notice shown when the ring stopped working.
compose-stopped-ring = L'anneau s'est arrêté. Appuyez sur Super pour le relancer.

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = La capture d'écran s'est arrêtée et a été fermée.

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = Réessayer

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = L'écran n'est pas verrouillé.

# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = Aucun code PIN n'est défini.

# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = Le temps accordé pour le code PIN est écoulé.

# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = Trop d'essais.

# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = Un code PIN compte 4 chiffres.

# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
        [one] Code PIN incorrect. 1 essai restant.
       *[other] Code PIN incorrect. { $left } essais restants.
    }
