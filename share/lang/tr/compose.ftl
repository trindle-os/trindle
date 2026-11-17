
# machine translated (Turkish), not yet reviewed by a person.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = Sessiz

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = Mikrofon kapalı

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = Bir program

# Notice shown when the ring stopped working.
compose-stopped-ring = Halka durdu. Yeniden başlatmak için Super tuşuna basın.

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = Ekran görüntüsü aracı durdu ve kapatıldı.

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = Yeniden dene

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = Ekran kilitli değil.

# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = Hiçbir PIN ayarlanmamış.

# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = PIN için tanınan süre doldu.

# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = Çok fazla deneme yapıldı.

# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = Bir PIN 4 rakamdan oluşur.

# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
        [one] PIN yanlış. 1 deneme hakkı kaldı.
       *[other] PIN yanlış. { $left } deneme hakkı kaldı.
    }
