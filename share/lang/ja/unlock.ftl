# machine translated (Japanese), not yet reviewed by a person.

## The disk-password screen (src/unlock): shown while the computer starts, before any desktop. Its words are built into that small program on their own.

# Heading of the box when the question that came with it was empty.
unlock-title-default = ディスクのパスワード
# Heading of the box in place of cryptsetup's own wording ("Please unlock disk ...", "Enter passphrase").
unlock-title-encryption = ディスクのパスワードを入力
# Key hint under the box: Tab, or the eye, shows the typed password.
unlock-show = 表示
# Key hint under the box: Tab, or the eye, hides the typed password again.
unlock-hide = 隠す
# Key hint under the box: F1 restarts the computer.
unlock-restart = 再起動
# Key hint under the box: F2 turns the computer off.
unlock-turn-off = 電源を切る
# Red line inside the card when Enter is pressed with nothing typed.
unlock-type-first = パスワードを入力してから Enter を押してください
# Line under the box when the disk is set up to be opened by the computer's TPM security chip and the chip did not open it this start (a firmware update, a BIOS reset or a changed Secure Boot setting can cause it).
unlock-chip-refused = セキュリティチップがディスクのロックを解除しませんでした。BIOS の更新が原因のことがあります。
# Second line under unlock-chip-refused: how to have the chip unlock the disk again. "Settings", "Hardware" and "Use the chip again" must match the names in Settings.
unlock-chip-refused-fix = 元に戻すには、設定、ハードウェア、チップをもう一度使う、の順に進みます。
# Line under the box when the disk is set up to be opened by the computer's TPM security chip and no chip could be found this start.
unlock-chip-missing = セキュリティチップが見つかりません。BIOS でオフになっている可能性があります。
# Second line under unlock-chip-missing: how to have the chip unlock the disk again.
unlock-chip-missing-fix = BIOS でオンに戻すと、パスワードなしで起動できます。
