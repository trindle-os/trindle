## The disk-password screen (src/unlock): shown while the computer starts, before any desktop. Its words are built into that small program on their own.

# Heading of the box when the question that came with it was empty.
unlock-title-default = Disk password
# Heading of the box in place of cryptsetup's own wording ("Please unlock disk ...", "Enter passphrase").
unlock-title-encryption = Enter disk password
# Key hint under the box: Tab, or the eye, shows the typed password.
unlock-show = Show
# Key hint under the box: Tab, or the eye, hides the typed password again.
unlock-hide = Hide
# Key hint under the box: F1 restarts the computer.
unlock-restart = Restart
# Key hint under the box: F2 turns the computer off.
unlock-turn-off = Switch off
# Red line inside the card when Enter is pressed with nothing typed.
unlock-type-first = Type the password, then press Enter
# Line under the box when the disk is set up to be opened by the computer's TPM security chip and the chip did not open it this start (a firmware update, a BIOS reset or a changed Secure Boot setting can cause it).
unlock-chip-refused = The security chip didn't unlock the disk. A BIOS update can cause this.
# Second line under unlock-chip-refused: how to have the chip unlock the disk again. "Settings", "Hardware" and "Use the chip again" must match the names in Settings.
unlock-chip-refused-fix = To turn it back on, go to Settings, Hardware, Use the chip again.
# Line under the box when the disk is set up to be opened by the computer's TPM security chip and no chip could be found this start.
unlock-chip-missing = The security chip wasn't found. It may be switched off in the BIOS.
# Second line under unlock-chip-missing: how to have the chip unlock the disk again.
unlock-chip-missing-fix = Switch it back on in the BIOS to start without the password.
