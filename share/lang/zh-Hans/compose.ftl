
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = 已静音

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = 麦克风已关闭

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = 某个程序

# Notice shown when the ring stopped working.
compose-stopped-ring = 圆环已停止。按 Super 键可重新启动。

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = 截图工具已停止并关闭。

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = 重试

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = 屏幕没有锁定。

# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = 没有设置 PIN。

# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = PIN 的有效时间已过。

# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = 尝试次数太多。

# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = PIN 是 4 位数字。

# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
       *[other] PIN 错误。还剩 { $left } 次机会。
    }
