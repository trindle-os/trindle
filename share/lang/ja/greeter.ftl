# machine translated (Japanese), not yet reviewed by a person.

## The login screen (src/greeter): shown by greetd before anyone is logged in

# Name of the login screen's window (what a screen reader and the window list call it).
greeter-window-title = ログイン
# Sentence at the top of the panel, for someone this computer knows.
greeter-welcome-back = おかえりなさい。
# Sentence at the top of the panel, when the name has not been typed yet.
greeter-welcome = ようこそ。
# Under the boxes, while there is nothing else to say.
greeter-ready = パスワードを入力して、Enter を押してください。
# Under the boxes, when a paired phone can stand in for the password.
greeter-ready-phone = パスワードを入力するか、Enter を押してスマートフォンを使ってください。
# Under the boxes, before a name is in.
greeter-ready-name = 名前とパスワードを入力して、Enter を押してください。
# Red line under the boxes after the phone did not log the person in.
greeter-phone-failed = スマートフォンではログインできませんでした。パスワードを入力してください。
# Under the password box for a few seconds, where a phone is paired.
greeter-phone-hint = スマートフォンでログイン
# At the foot of the screen, for when nothing else works.
greeter-trouble = ログインできませんか? Ctrl+Alt+F2 を押すと、テキストでログインできます。
# Beside the light drawing switch, on a computer with no graphics chip.
greeter-no-chip = Trindle はグラフィックスチップを見つけられなかったため、デスクトップを軽く動かせるよう、簡易グラフィックスを使っています。
# The main button's name (a round button with an arrow) that logs in.
greeter-sign-in = ログイン
# Red line under the boxes when the name and password are wrong, or the account is unknown.
greeter-no-match = 名前とパスワードが一致しません。
# Red line under the boxes after too many wrong passwords.
greeter-pam-max-tries = 試行回数が多すぎます。1 分待ってから、もう一度お試しください。
# Red line under the boxes when the account has run out.
greeter-pam-account-expired = このアカウントは期限切れです。
# Red line under the boxes when the password must be changed before logging in.
greeter-pam-password-expired = このパスワードは期限切れです。テキストコンソール (Ctrl+Alt+F2) でログインして変更してください。
# Red line under the boxes, or the sentence at the top, when the login failed and the system gave no reason.
greeter-login-failed = ログインできませんでした。
# Red line under the name box when Enter is pressed with no name.
greeter-name-first = 先に名前を入力してください。
# Sentence at the top while waiting for the phone to answer.
greeter-asking-phone = スマートフォンに確認しています…
# Sentence at the top while the system checks the password.
greeter-logging-in = ログインしています…
# Typing box for the person's name, shown quietly while empty.
greeter-name-box = 名前
# Typing box for the password, shown quietly while empty.
greeter-password-box = パスワード
# Typing box for an extra question the login asked (a code, for one), when the question itself is empty.
greeter-answer-box = 答え
# Name and tip of the round button that logs in with a paired phone.
greeter-use-phone = スマートフォンを使う
# Name of the switch for light drawing on a computer with no graphics chip, and the word under it.
greeter-light-drawing = 簡易グラフィックス
# Tip of the round button that shows or hides the on-screen keyboard.
greeter-keyboard-tip = 画面キーボード
# Word under the on-screen keyboard button.
greeter-keyboard = キーボード
# Tip and word under the round button that turns the screen reader on or off.
greeter-screen-reader = スクリーンリーダー
# Word under a power button after the first press, and its tip.
greeter-press-again = もう一度押す
# Tip and word under the round button that restarts the computer.
greeter-restart = 再起動
# Tip of the restart button after the first press.
greeter-restart-sure = もう一度押すと再起動します
# Tip and word under the round button that turns the computer off.
greeter-shut-down = シャットダウン
# Tip of the shut down button after the first press.
greeter-shut-down-sure = もう一度押すとシャットダウンします
# Red line under the boxes when a power button is pressed and the system has no bus to ask.
greeter-no-bus = Trindle はシステムに接続できません。
# Red line under the boxes when the system refused to restart; the reason follows, as the system worded it.
greeter-restart-failed = 再起動できませんでした: { $why }
# Red line under the boxes when the system refused to turn off; the reason follows, as the system worded it.
greeter-shut-down-failed = シャットダウンできませんでした: { $why }
# Red line under the boxes when the login service could not be spoken to.
greeter-service-no-talk = ログインサービスに接続できませんでした。
# Red line under the boxes when the login service was too slow.
greeter-service-timeout = ログインサービスの応答に時間がかかりすぎました。
# Red line under the boxes when the login service hung up.
greeter-service-closed = greetd が接続を閉じました。
# Red line under the boxes when the login service's answer was far too large.
greeter-service-too-big = ログインサービスの返答が大きすぎて読めませんでした。
# Red line under the boxes when the login screen was started by hand and not by the login service.
greeter-service-not-started = GREETD_SOCK が設定されていないため、greetd がこのログイン画面を起動しませんでした。
# Red line under the boxes when there is no connection to the login service.
greeter-service-no-connection = 接続なし
