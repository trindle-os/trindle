# machine translated (Korean), not yet reviewed by a person.

## The disk-password screen (src/unlock): shown while the computer starts, before any desktop. Its words are built into that small program on their own.

# Heading of the box when the question that came with it was empty.
unlock-title-default = 디스크 비밀번호
# Heading of the box in place of cryptsetup's own wording ("Please unlock disk ...", "Enter passphrase").
unlock-title-encryption = 디스크 비밀번호 입력
# Key hint under the box: Tab, or the eye, shows the typed password.
unlock-show = 표시
# Key hint under the box: Tab, or the eye, hides the typed password again.
unlock-hide = 숨기기
# Key hint under the box: F1 restarts the computer.
unlock-restart = 다시 시작
# Key hint under the box: F2 turns the computer off.
unlock-turn-off = 끄기
# Red line inside the card when Enter is pressed with nothing typed.
unlock-type-first = 비밀번호를 입력한 다음 Enter를 누르세요
# Line under the box when the disk is set up to be opened by the computer's TPM security chip and the chip did not open it this start (a firmware update, a BIOS reset or a changed Secure Boot setting can cause it).
unlock-chip-refused = 보안 칩이 디스크를 열지 못했어요. BIOS 업데이트 때문일 수 있어요.
# Second line under unlock-chip-refused: how to have the chip unlock the disk again. "Settings", "Hardware" and "Use the chip again" must match the names in Settings.
unlock-chip-refused-fix = 다시 켜려면 설정, 하드웨어, 칩 다시 사용으로 가세요.
# Line under the box when the disk is set up to be opened by the computer's TPM security chip and no chip could be found this start.
unlock-chip-missing = 보안 칩을 찾지 못했어요. BIOS에서 꺼져 있을 수 있어요.
# Second line under unlock-chip-missing: how to have the chip unlock the disk again.
unlock-chip-missing-fix = 비밀번호 없이 시작하려면 BIOS에서 다시 켜세요.
