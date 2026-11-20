# machine translated (Japanese), not yet reviewed by a person.

# Trindle's administrator password question, the box that asks for the
# password when a step needs the administrator (polkit's agent in src/polkit).
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The password question (src/polkit/mod.rs)

# Heading of the card asking for the administrator's password.
polkit-title = 管理者のパスワードが必要です
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = { $user } のパスワード
# Button that lets the step go ahead once the password is typed.
polkit-allow = 許可
# Button that stops the step and closes the question.
polkit-cancel = キャンセル
# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = パスワードが違います。もう一度入力してください。
# Name a screen reader speaks for the password question window.
polkit-spoken = あなたのパスワード
# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
       *[other] ほかに { $n } 件の要求が待っています。
    }

## Why Drives asks (src/polkit/mod.rs)

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Drivesが空きを作る前にスナップショットを取るため、管理者のパスワードが必要です。
# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Drivesが空きを作る間スワップファイルをオフにするため、管理者のパスワードが必要です。
# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Drivesがスワップファイルをもう一度オンにするため、管理者のパスワードが必要です。
