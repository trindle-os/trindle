# machine translated (Japanese), not yet reviewed by a person.

## Words of the compositor (trindle-compose): the level pill, the notice that a part stopped and the name of a program holding the mouse and keyboard.

# Shown beside the volume bar when the speakers are muted.
compose-level-muted = ミュート中

# Shown beside the microphone bar when the microphone is muted.
compose-level-mic-off = マイクはオフ

# Shown in the corner dot's tip when a program that has no name of its own holds the mouse and keyboard.
compose-capture-unknown-program = あるプログラム

# Notice shown when the ring stopped working.
compose-stopped-ring = リングが止まりました。Super キーを押すと、もう一度始まります。

# Notice shown when the screenshot tool stopped working and was closed.
compose-stopped-shot = スクリーンショットが止まったため、閉じました。

# Tip of the round button on that notice, which starts the stopped part again.
compose-stopped-try-again = やり直す

# Sent to Flow when you try its PIN and the screen isn't locked.
compose-flow-not-locked = 画面はロックされていません。
# Sent to Flow when you try its PIN and no PIN is set.
compose-flow-no-pin = PIN が設定されていません。
# Sent to Flow when you try its PIN after the time allowed for it has run out.
compose-flow-late = PIN の制限時間が過ぎました。
# Sent to Flow when the wrong PIN has been tried too many times.
compose-flow-too-many = 試行回数が多すぎます。
# Sent to Flow when what was typed isn't a PIN of 4 digits.
compose-flow-not-a-pin = PIN は 4 桁です。
# Sent to Flow when the PIN is wrong. { $left } is the tries left.
compose-flow-wrong-pin =
    { $left ->
       *[other] PIN が違います。あと { $left } 回試せます。
    }
