# machine translated (Korean), not yet reviewed by a person.

## Words of the compositor (trindle-compose): the level pill, the notice that a part stopped and the name of a program holding the mouse and keyboard.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = 음소거

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = 마이크 꺼짐

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = 어떤 프로그램

# Notice shown when the ring stopped working.
compose-stopped-ring = 링이 멈췄어요. Super를 누르면 다시 시작해요.

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = 스크린샷이 멈춰서 닫혔어요.

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = 다시 시도

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = 화면이 잠겨 있지 않아요.
# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = PIN이 설정되어 있지 않아요.
# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = PIN을 입력할 시간이 지났어요.
# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = 시도 횟수가 너무 많아요.
# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = PIN은 4자리예요.
# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
       *[other] PIN이 틀렸어요. { $left }번 남았어요.
    }
