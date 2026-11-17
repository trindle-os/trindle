## Words of the compositor (trindle-compose): the level pill, the notice that a part stopped and the name of a program holding the mouse and keyboard.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = Muted

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = Microphone off

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = A program

# Notice shown when the ring stopped working.
compose-stopped-ring = The ring stopped. Press Super to start it again.

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = Screenshot stopped and was closed.

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = Try again

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = The screen isn't locked.
# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = No PIN is set.
# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = The PIN time is over.
# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = Too many tries.
# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = A PIN is 4 digits.
# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
        [one] Wrong PIN. 1 try left.
       *[other] Wrong PIN. { $left } tries left.
    }
