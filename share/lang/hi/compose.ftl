# machine translated (Hindi), not yet reviewed by a person.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = म्यूट

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = माइक्रोफ़ोन बंद

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = एक प्रोग्राम

# Notice shown when the ring stopped working.
compose-stopped-ring = रिंग रुक गई। इसे फिर चालू करने के लिए Super दबाएँ।

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = स्क्रीनशॉट रुक गया और बंद कर दिया गया।

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = फिर कोशिश करें

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = स्क्रीन लॉक नहीं है।

# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = कोई PIN सेट नहीं है।

# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = PIN का समय खत्म हो गया।

# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = बहुत ज़्यादा कोशिशें हो गईं।

# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = PIN 4 अंकों का होता है।

# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
        [one] PIN गलत है। 1 कोशिश बाकी है।
       *[other] PIN गलत है। { $left } कोशिशें बाकी हैं।
    }
