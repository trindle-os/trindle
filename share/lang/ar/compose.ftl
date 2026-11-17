
# machine translated (Arabic), not yet reviewed by a person.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = مكتوم

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = الميكروفون متوقف

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = برنامج

# Notice shown when the ring stopped working.
compose-stopped-ring = توقفت الحلقة. اضغط على Super لتشغيلها مرة أخرى.

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = توقف التقاط الشاشة وأُغلق.

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = حاول مرة أخرى

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = الشاشة غير مقفلة.

# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = لم يُعيَّن رقم PIN.

# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = انتهى وقت رقم PIN.

# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = محاولات كثيرة جدًا.

# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = رقم PIN من 4 أرقام.

# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
        [zero] رقم PIN خاطئ. لا محاولات متبقية.
        [one] رقم PIN خاطئ. بقيت محاولة واحدة.
        [two] رقم PIN خاطئ. بقيت محاولتان.
        [few] رقم PIN خاطئ. بقيت { $left } محاولات.
        [many] رقم PIN خاطئ. بقيت { $left } محاولة.
       *[other] رقم PIN خاطئ. بقيت { $left } محاولة.
    }
