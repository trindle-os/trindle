
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Heading of the box when the question that came with it was empty.
# SAFETY: needs a human review before it ships
unlock-title-default = 磁盘密码

# Heading of the box in place of cryptsetup's own wording ("Please unlock disk ...", "Enter passphrase").
# SAFETY: needs a human review before it ships
unlock-title-encryption = 输入磁盘密码

# Key hint under the box: Tab, or the eye, shows the typed password.
unlock-show = 显示

# Key hint under the box: Tab, or the eye, hides the typed password again.
unlock-hide = 隐藏

# Key hint under the box: F1 restarts the computer.
unlock-restart = 重启

# Key hint under the box: F2 turns the computer off.
unlock-turn-off = 关机

# Red line inside the card when Enter is pressed with nothing typed.
# SAFETY: needs a human review before it ships
unlock-type-first = 请先输入密码，再按回车键

# Line under the box when the disk is set up to be opened by the computer's TPM security chip and the chip did not open it this start (a firmware update, a BIOS reset or a changed Secure Boot setting can cause it).
unlock-chip-refused = 安全芯片没有解锁磁盘。BIOS 更新可能导致这种情况。

# Second line under unlock-chip-refused: how to have the chip unlock the disk again. "Settings", "Hardware" and "Use the chip again" must match the names in Settings.
unlock-chip-refused-fix = 要重新启用，请打开“设置”，进入“硬件”，点“再次使用芯片”。

# Line under the box when the disk is set up to be opened by the computer's TPM security chip and no chip could be found this start.
unlock-chip-missing = 没有找到安全芯片。可能在 BIOS 中被关闭了。

# Second line under unlock-chip-missing: how to have the chip unlock the disk again.
unlock-chip-missing-fix = 在 BIOS 中重新打开它，就不用输入密码即可启动。
