# machine translated (Japanese), not yet reviewed by a person.

# Trindle's session: the lock screen's words (the password and PIN card, the
# phone check and what it says when it fails), the "One moment, please" card
# shown while the desktop comes up, and the note on wallpapers that cost
# more memory than they need.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The lock screen (src/session/lock.rs)

# Heading of the card when the PIN is the way in.
session-lock-enter-pin = PIN を入力してください
# Heading of the card when the password is the way in.
session-lock-locked = ロック中
# Under the heading of the password card: what to do.
session-lock-type-password = パスワードを入力して、Enter を押してください
# Button under the PIN card that changes to typing the password.
session-lock-use-password = パスワードを使う
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = PIN を使う
# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = スマートフォンを使う
# Shown while the typed password or PIN is being checked.
session-lock-checking = 確認しています…
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = PIN の有効時間が切れました。パスワードを入力してください。
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = PIN の間違いが多すぎます。パスワードを使ってください。
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
       *[other] PIN が違います。あと { $n } 回できます。
    }
# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = スマートフォンに問い合わせています…
# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Trindle はスマートフォンでロックを解除できませんでした。PIN を使ってください。
# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Trindle はスマートフォンでロックを解除できませんでした。パスワードを入力してください。
# Shown when the password typed is not the person's password.
session-lock-wrong-password = パスワードが違います。

## One moment, please (src/session/starting.rs)

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = しばらくお待ちください

## Wallpapers that cost more than they need (src/session/wallpaper_limits.rs)

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = この画像は画面より大きく、余分にメモリーを使います。
# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = 動く壁紙は、メモリーとバッテリーを多く使います。
# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = 4K より大きい
# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = 画面より大きい
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = 20 MB を超えている（動くものは 200 MB）
# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = 20 MB を超えている
# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = 200 MB を超えている
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first }か、{ $second }
# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out =
    { $count ->
       *[other] { $count } 枚の画像は、{ $why }ため、Trindle が除外しました。
    }

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = 時間がかかりすぎました。もう一度お試しください。
# Under the lock screen when Enter is pressed while the last try, given up on, is still being checked.
session-lock-still-checking = 前の入力をまだ確認しています。少し待ってから、もう一度お試しください。
# Under the lock screen after several wrong tries, with how many seconds to wait (1 is one second).
session-lock-wait =
    { $n ->
       *[other] 間違いが多すぎます。{ $n } 秒後にもう一度お試しください。
    }

## The question before everything is restarted (src/session/restart_ask.rs)

# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = デスクトップ全体を再起動しますか?
# Under the heading: what restarting everything does to the open windows.
session-restart-windows = 開いているウィンドウはすべて閉じます。保存していない内容は失われます。
# Tip of the round button that leaves everything as it is.
session-restart-cancel = キャンセル
# Main button that restarts the whole desktop.
session-restart-go = 再起動
