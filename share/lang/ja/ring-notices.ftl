# machine translated (Japanese), not yet reviewed by a person.

# The ring's notices and notification circles: battery card, snooze choices, crash notices, the microphone switch and the magnifier. Part of the ring's own words (see ring.ftl).

## Battery card (src/ring/battery_alert.rs)

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = バッテリー残量 { $pct }%
# Line under the low battery warning's title.
ring-battery-plug-in = 早めに充電器をつないでください。
# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = 充電中
# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = 充電器が外れました
# Line under the title when the charger comes out.
ring-battery-running = バッテリーで動作中、残量 { $pct }%

## Notification circles (src/ring/circles.rs)

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 分
# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 時間
# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 時間
# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = 翌朝
# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = 10 分後に再通知
# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = 1 時間後に再通知
# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = 6 時間後に再通知
# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = 朝に再通知
# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = メッセージに戻る
# Tip of the pin button when the notification is already pinned.
ring-act-unpin = 固定を外す
# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = 固定
# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = 後で通知
# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = すべて消去
# Tip of the copy button for a moment after the message was copied.
ring-act-copied = コピーしました
# Tip of the button on a notification circle that copies its message.
ring-act-copy = メッセージをコピー

## Crash notices (src/ring/crashes.rs)

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = クリックすると、アシスタントに渡します。
# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle
# Name of the desktop part in the crash notice.
ring-crash-desktop = Trindle のデスクトップ
# Name of the login screen in the crash notice.
ring-crash-greeter = Trindle のログイン画面
# Name of the lock screen in the crash notice.
ring-crash-lock = Trindle のロック画面
# Name of Settings in the crash notice.
ring-crash-settings = Trindle の設定
# Name of Apps in the crash notice.
ring-crash-apps = Trindle のアプリ
# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = Trindle の { $name }
# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } が予期せず停止しました

## Drives (src/ring/drives.rs)

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = そのドライブはもうありません
# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = { $name } を開けませんでした

## Closing the lid (src/ring/lid.rs)

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = スリープ
# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = 休止状態
# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = 画面をロック
# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = 何もしない
# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = シャットダウン
# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = ふたを閉じたときの動作は、設定の電源で選べます。

## Magnifier (src/ring/magnify.rs)

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = 画面のどこかを押し続けると拡大します
# Screen reader name of the magnifier's glass.
ring-magnify-name = 拡大鏡
# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = 拡大をやめる

## Microphone switch (src/ring/mic.rs)

# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = すべてのプログラムでマイクをオフにする
# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = マイクをオンにする
# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = マイクを再びオンにできるのは、あなただけです。
# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = Trindle はマイクのスイッチを切り替えられませんでした: { $why }

## Clicking a notification (src/ring/note_goes.rs)

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = プログラムが見つかりません
# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = 別のプログラムからの通知です
# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = 起動できませんでした

## Notification stack (src/ring/notes/)

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = アシスタント用にコピーしました
# Screen reader name of a notification circle.
ring-notes-name = 通知
# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = 通知を移動中です。放すと置き、Escape を押すと元に戻ります。
# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = 移動しました
# Said when Escape put a held notification back.
ring-notes-put-back = 元に戻しました
# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = 移動するには固定してください

## Notification circles (src/ring/notify.rs)

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
       *[other] { $n } 秒
    }
# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = 閉じるまで
# Time under a notification from yesterday.
ring-notify-yesterday = 昨日 { $clock }
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = 右下
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = 左下
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = 左上
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = 右上

## Plugins in the ring (src/ring/plugins.rs)

# Title of the notice when a plugin that asks for permissions has been given none.
ring-plugin-needs = { $name } には許可が必要です
# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
ring-plugin-cannot-work =
    { $asks ->
       *[other] まだ動作できません。求められている { $asks } 件の許可を、1 件も許可していません。
    }

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = コピーしました。{ $said }

## Programs outside a box (src/ring/outside.rs)

# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = ネットワークに接続しました
# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = ファイルを開いています
# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
       *[other] { $names } ほか { $more } 件
    }

# Said under the magnifier when the screen picture it took has the wrong size.
ring-magnify-wrong-size = 画像のサイズが正しくありませんでした。
