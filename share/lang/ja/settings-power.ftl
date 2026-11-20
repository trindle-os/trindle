# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Power": battery, power mode, screen dimming and locking, PIN, log out and shut down; also the PIN time names from src/common/pin.rs.

## Page (src/settings/panels/power.rs)

# Name of the Settings page about power, in the list and as its heading.
settings-power-title = 電源
# Words a person might type into the Settings search to find the power page; separate them with spaces.
settings-power-keywords = バッテリー 電池 スリープ 休止 シャットダウン 電源を切る 再起動 ログアウト 画面オフ 待機 暗くする ロック パスワード PIN テンキー 起きたまま 充電 充電中 上限 80 バッテリーの状態 battery suspend sleep shut down restart log out screen off idle dim lock password pin keypad caffeine keep awake charge charging limit battery health
# Introduction under the heading of the power page, and the words behind the page i.
settings-power-help = 電源モードと、画面を暗くする、ロックする、消すタイミングを選びます。ノートパソコンでは、バッテリー駆動のときと充電器につないだときを別々に設定できます。リングの中のカップは、画面をつけたままにします。ここからロック、再起動、シャットダウンもできます。

## Times in the lists

# Choice in a list of how long to wait.
settings-power-time-never = しない
# Choice in a list of how long to wait.
settings-power-time-1m = 1分
# Choice in a list of how long to wait.
settings-power-time-2m = 2分
# Choice in a list of how long to wait.
settings-power-time-5m = 5分
# Choice in a list of how long to wait.
settings-power-time-10m = 10分
# Choice in a list of how long to wait.
settings-power-time-15m = 15分
# Choice in a list of how long to wait.
settings-power-time-30m = 30分
# Choice in a list of how long to wait.
settings-power-time-45m = 45分
# Choice in a list of how long to wait.
settings-power-time-1h = 1時間
# Choice in a list of how long to wait.
settings-power-time-90m = 1時間30分
# Choice in a list of how long to wait.
settings-power-time-2h = 2時間
# Choice in the lock after list: lock at no other time than when the computer sleeps.
settings-power-time-only-sleep = スリープ時だけ

## Session buttons and lid

# Heading of the line, and name of its round button, that suspends the computer.
settings-power-suspend = スリープ
# Heading of the line, and name of its round button, that restarts the computer.
settings-power-restart = 再起動
# Heading of the line, and name of its round button, that shuts the computer down.
settings-power-shut-down = シャットダウン
# Choice in the list of what closing the lid does: the computer sleeps.
settings-power-lid-sleep = スリープ
# Choice in the list of what closing the lid does: the computer hibernates.
settings-power-lid-hibernate = 休止状態
# Choice in the list of what closing the lid does: the screen locks.
settings-power-lid-lock = 画面をロック
# Choice in the list of what closing the lid does: nothing.
settings-power-lid-nothing = 何もしない
# Choice in the list of what closing the lid does: the computer shuts down.
settings-power-lid-shut-down = シャットダウン

## Idle timers

# Heading of an idle timer line, and the name of its lists.
settings-power-timer-dim = 画面を暗くするまで
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-clock = 動く時計を表示するまで
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-off = 画面を消すまで
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-suspend = スリープするまで

## Warnings and states

# Red words under the keep the screen awake switch when turning it on could not be saved.
settings-power-no-awake = 選択を保存できなかったので、Trindle は画面をつけたままにできませんでした。
# Red words under the keep the screen awake switch when turning it off could not be saved.
settings-power-no-sleep = Trindle は選択を保存できなかったので、画面はつけたままになります。
# Name of the power mode that saves the battery.
settings-power-profile-saver = 省電力
# Name of the balanced power mode.
settings-power-profile-balanced = バランス
# Name of the power mode that favours speed.
settings-power-profile-performance = パフォーマンス
# What the battery is doing: charging.
settings-power-state-charging = 充電中
# What the battery is doing: running on its own charge.
settings-power-state-battery = バッテリー駆動
# What the battery is doing: empty.
settings-power-state-empty = 空
# What the battery is doing: full.
settings-power-state-full = 満充電
# What the battery is doing: the charger is in but it is not charging.
settings-power-state-plugged = 接続中、充電していません
# Time left on the battery; { $h } hours and { $m } minutes. The h and min are short for hours and minutes.
settings-power-hours-minutes = { $h }時間{ $m }分
# Time left on the battery when under an hour; { $m } minutes. The min is short for minutes.
settings-power-minutes = { $m }分

## Top sentence

# Sentence at the top of the page on a computer with no battery.
settings-power-top-no-battery = 電源につながっています。このコンピューターにはバッテリーがありません。
# Sentence at the top of the page on a computer with no battery; { $mode } is the power mode.
settings-power-top-no-battery-mode = 電源につながっています。モード: { $mode }。このコンピューターにはバッテリーがありません。
# Sentence at the top of the page while charging; { $percent } is how full.
settings-power-top-charging = 充電中、{ $percent }%。
# Sentence at the top of the page while charging; { $percent } is how full and { $left } how long until full.
settings-power-top-charging-left = 充電中、{ $percent }%、満充電まで { $left }。
# Sentence at the top of the page when the battery is full.
settings-power-top-full = バッテリーは満充電です。
# Sentence at the top of the page when plugged in but not charging; { $percent } is how full.
settings-power-top-plugged = 電源につながっています、{ $percent }%、充電していません。
# Sentence at the top of the page on battery; { $percent } is how full.
settings-power-top-battery = バッテリー残量 { $percent }%。
# Sentence at the top of the page on battery; { $percent } is how full and { $left } how long is left.
settings-power-top-battery-left = バッテリー残量 { $percent }%、あと約 { $left }。
# Sentence at the top of the page while the battery is being read.
settings-power-top-looking = バッテリーを確認しています…

## Power mode and battery card

# Red words under the power mode line when the mode could not be changed.
settings-power-mode-failed = 電源モードを変更できませんでした。
# Heading of the power mode line, and the name of its list.
settings-power-mode = 電源モード
# Said in green under a line after its choice was kept.
settings-power-saved = 保存しました
# Name of the list of battery levels to warn at.
settings-power-warn-name = バッテリー残量がここまで減ったら警告
# Heading of the line for the battery warning.
settings-power-warn = ここまで減ったら警告
# Choice in the list of battery levels: never warn.
settings-power-never = しない
# Heading of the line, and name of its switch, for stopping charging at 80%.
settings-power-limit = 80%で充電を止める
# Title of the battery card on a laptop.
settings-power-card-battery = バッテリー
# Title of the battery card when there is no battery picture and no power modes.
settings-power-card-looking-after = バッテリーを大切に
# Words behind the i of the battery card.
settings-power-battery-about = 省電力は温度を抑え、バッテリーを長持ちさせます。パフォーマンスはより速く動きます。バランスはその中間です。警告は、バッテリーが減ってきたときに画面の中央に1回表示されます。ほとんど電源につないだまま使うノートパソコンでは、80%で止めるとバッテリーが長い年数もちます。1日コンセントから離れるときは、オフにしてください。
# Words under the charge limit line while the password window is open.
settings-power-limit-waiting = パスワードを待っています…
# Green words under the charge limit line after charging was limited to 80%.
settings-power-limit-on = 80%で止まります
# Green words under the charge limit line after the limit was removed.
settings-power-limit-off = 満充電まで充電します

## Screen cards

# Heading of the keep the screen awake line, and name of its switch.
settings-power-awake = 画面をつけたままにする
# Title of the card about dimming and the moving clock.
settings-power-screen-title = 画面
# Words behind the i of the Screen card.
settings-power-screen-about = 「画面をつけたままにする」がオンの間は、これらは起きません。リングの中のカップでもオンとオフを切り替えられます。暗くできるのは、ノートパソコンのように専用の明るさ調整がある画面だけです。動く時計は画面を暗くして、ゆっくり動く時刻だけを表示します。マウスを動かすかキーを押すと、すべて元に戻ります。ノートパソコンでは、各時間をバッテリー用と充電器用の2回設定します。
# Title of the card about turning the screen off and sleeping.
settings-power-sleep-title = 画面オフとスリープ
# Words behind the i of the Screen off and sleep card.
settings-power-sleep-about = キーを押すかマウスを動かすと、画面がまたつきます。スリープは、起こすまでコンピューターを一時停止します。ノートパソコンでは、各時間をバッテリー用と充電器用の2回設定します。2台目の画面がつながっているときは、カバーを閉じても何も起きないので、大きい画面で作業を続けられます。
# Heading of the line for what closing the lid does.
settings-power-lid = カバーを閉じたとき

## Battery or charger captions

# Caption above a list that applies while the laptop runs on its battery.
settings-power-when-battery = バッテリー駆動のとき
# Caption above a list that applies while the laptop is plugged in.
settings-power-when-charger = 電源につないだとき
# Name of a list that applies on battery; { $title } is the name of its line.
settings-power-named-battery = { $title }: バッテリー駆動のとき
# Name of a list that applies while plugged in; { $title } is the name of its line.
settings-power-named-charger = { $title }: 電源につないだとき

## Locking

# Heading of the lock the screen line, and name of its switch.
settings-power-lock = 画面をロック
# Heading of the lock after line, and the name of its list.
settings-power-lock-after-name = ロックするまで
# Heading of the line, and name of its switch, for locking when the computer sleeps.
settings-power-lock-sleep = スリープ時にロック
# Heading of the line, and name of its button, that locks the screen now.
settings-power-lock-now = 今すぐロック
# Small words under the lock now line.
settings-power-lock-now-about = Super キーと L キーを押しても同じです。
# Red words under the lock now line when the lock screen would not start.
settings-power-lock-now-failed = ロック画面が起動しなかったので、Trindle は画面をロックできませんでした。
# Title of the locking card.
settings-power-lock-title = ロック
# Words behind the i of the locking card.
settings-power-lock-card-about = 席を離れると、画面がロックされます。パスワード、または設定していれば4桁の PIN で解除するまで、何も表示されません。ログインには、いつもパスワードが必要です。「ロックするまで」は、画面やスリープの時間に関係なく、放っておいてからロックされるまでの時間です。それまでは、戻ってきても PIN は要りません。ロックをオフにすると、PIN も削除されます。「PIN が使える時間」を過ぎたとき、または5回間違えたときは、代わりにパスワードが必要です。

## PIN

# Heading of the line for adding a PIN.
settings-power-pin-add = PIN でも解除する
# Heading of the line, and name of its list, for how long a PIN works after the screen locks.
settings-power-pin-works-name = PIN が使える時間
# Name of the bin button that stops using a PIN.
settings-power-pin-stop = PIN を使わない
# Heading of the line shown when a PIN is kept.
settings-power-pin-kept = PIN が設定されています
# Placeholder of the box for typing a new PIN.
settings-power-pin-new = 新しい PIN
# Name of the button that keeps the PIN typed.
settings-power-pin-keep = この PIN を保存
# Placeholder of the box for typing your account password before a PIN is saved.
settings-power-pin-password = パスワード
# Red words under the PIN box when no password was typed.
settings-power-pin-need-password = 先にパスワードを入力してから、PIN を保存してください。
# Red words under the PIN box when the password typed was wrong.
settings-power-pin-wrong-password = パスワードが違うので、Trindle は PIN を保存しませんでした。
# Red words under the PIN line when the PIN could not be removed.
settings-power-pin-not-removed = Trindle は PIN を削除できなかったので、PIN でまだ画面を解除できます。
# Red words under the PIN box when it is not the right length; { $n } is how many digits a PIN has.
settings-power-pin-digits =
    { $n ->
       *[other] { $n } 桁で入力してください。
    }

## Session

# Heading of the log out line, and name of its round button.
settings-power-log-out = ログアウト
# Title of the card with log out, suspend, restart and shut down.
settings-power-session-title = セッション
# Words behind the i of the session card.
settings-power-session-about = ログアウトすると、すべてのウィンドウが閉じて、ログイン画面に戻ります。スリープは、起こすまでコンピューターを一時停止します。リングの中の電源の選択と同じように、どのボタンも確認のためにもう一度押す必要があるので、うっかり押しても何も起きません。
# Red words under a session button when the system bus is not there.
settings-power-no-bus = システムサービスに接続できません。
# Red words under a session button when the computer would not do it.
settings-power-would-not = コンピューターがそれを実行できませんでした。

## Pictures

# Spoken description of the battery picture while the battery is being read.
settings-power-art-looking-tip = バッテリーを確認しています。
# Spoken description of the battery picture; { $percent } is how full and { $state } what it is doing, such as Charging.
settings-power-art-tip = バッテリー残量 { $percent }%。{ $state }
# Word drawn in the empty battery picture while the battery is being read.
settings-power-art-looking = 読み込み中
# Word drawn under the battery figure when its state is not known.
settings-power-art-battery = バッテリー
# Words drawn in the battery picture: { $left } is the time left and { $state } is either empty or full.
settings-power-art-left =
    { $state ->
        [empty] 空まで { $left }
       *[full] 満充電まで { $left }
    }
# Spoken description of the battery range bar when there is no warning; { $top } is the percent it charges to.
settings-power-limits-tip = { $top }%まで充電、警告なし。
# Spoken description of the battery range bar; { $top } is the percent it charges to and { $warn } where a warning comes.
settings-power-limits-tip-warn = { $top }%まで充電、{ $warn }%で警告。

## PIN times (src/common/pin.rs)

# A length of time in the list of how long a PIN works, in minutes; { $n } is the number.
settings-power-pin-minutes =
    { $n ->
       *[other] { $n }分
    }
# A length of time in the list of how long a PIN works, in whole hours; { $n } is the number.
settings-power-pin-hours =
    { $n ->
       *[other] { $n }時間
    }
# A length of time in the list of how long a PIN works, in hours and minutes; { $hours } and { $minutes } are the two parts already written out, such as 1 hour and 15 minutes.
settings-power-pin-hours-minutes = { $hours }{ $minutes }
