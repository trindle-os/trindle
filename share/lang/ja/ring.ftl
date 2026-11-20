# machine translated (Japanese), not yet reviewed by a person.

# The ring's own words (src/ring): the circle opened from the top of the screen, its pages, the notification circles, the tray and the small notices.
# Keys are ring-<page>-<thing>. Words shared by several pages are ring-<word>.

## Words shared by the ring's pages

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = 閉じる
# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = 開く
# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = 閉じるものはありません
# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = 設定

## Command line (src/ring/mod.rs)

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = 使い方:
# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [OPTION…]
# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = オプション:
# Help for the ring's --say option, printed in a terminal.
ring-cli-say = マイクを使わず、文字で音声コマンドを実行します
# Help for the ring's --background option, printed in a terminal.
ring-cli-background = 見えない状態で起動し、最初からすぐリングが開くようにします
# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = すべてのウィンドウを隠してデスクトップを表示、または元に戻します(Super と D キーでも同じです)
# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = 音声コマンドを待ち受けた状態でリングを開きます。キーボードショートカットと同じです
# Help for the ring's --all option, printed in a terminal.
ring-cli-all = すべてのアプリを周りに表示してリングを開きます
# Help for the ring's --power option, printed in a terminal.
ring-cli-power = リングの電源オプションを開きます: ログアウト、再起動、シャットダウン
# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = 設定のお気に入りのページを開きます
# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = 設定で選んだ見た目をすぐに反映します(ライトかダーク、または新しいアクセントカラー)
# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = 上の角にあるタッチ部分をもう一度設定します
# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = キーボード操作を画面上の通知の丸に移します(Super と N キーでも同じです)
# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = すべてのキーボードショートカットの一覧を表示、または隠します(設定で変えない限り Super と K)
# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = 不明なオプション { $option }
# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = --say に引数がありません

## The ring itself (src/ring/view/)

# Tip of the round button on an app's menu that removes the app.
ring-uninstall = アンインストール
# Line that replaces the uninstall button while the app is being removed.
ring-uninstalling = アンインストール中
# Line that replaces the uninstall button once the app is removed.
ring-uninstalled = アンインストールしました
# Tip on the app menu's red button when removing the app did not work.
ring-uninstall-failed = 削除できませんでした。
# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = リングから外す
# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = 外しています
# Line that replaces that button once the app is off the ring.
ring-taken-off = 外しました
# Name of the ring's round button that opens Apps.
ring-tool-apps = アプリ
# Name of the ring's round button that opens Activity.
ring-tool-activity = アクティビティ
# Name of the ring's round button that opens Drives.
ring-tool-drives = ドライブ
# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = スクリーンショット
# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = クリップボード
# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = 通知
# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = 拡大鏡
# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = スポイト
# Name of the ring's round button that opens the report a problem window.
ring-tool-report = 問題を報告
# Line in the ring when the item that was pressed is not there any more.
ring-gone = それはもうありません。
# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Trindle ではそれを開けません。
# Shorter name of the Sound and video piece of All apps, for when the full name does not fit along it.
ring-category-media-short = サウンド
# Line in the ring when a spoken command, such as turning the sound down, did not work.
ring-voice-failed = うまくいきませんでした。
# Line in the ring while it listens to a spoken command.
ring-listening = 聞いています… コマンドを話してください。話し終わると止まります
# Line in the ring while it works out what was said.
ring-working-out = 聞き取っています…
# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = 閉じるものはありません
# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = { $date } の { $time } です
# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = この画面の明るさはここからは変えられません
# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = スクリーンキーボードがインストールされていません
# Screen reader name of the ring of open windows.
ring-open-windows = 開いているウィンドウ
# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = 元に戻す
# Tip of the button on a window's menu that minimises it.
ring-window-minimise = 最小化
# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = 元のサイズに戻す
# Tip of the button on a window's menu that maximises it.
ring-window-maximise = 最大化
# Tip of the button on a window's menu that stops a program that does not answer.
ring-window-force-close = 強制終了
# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = すべてのアプリ
# Name of the ring's round button that opens the power page.
ring-power = 電源
# Screen reader description of the ring's power button.
ring-power-spoken = ログアウト、再起動、シャットダウン
# Screen reader name of the ring's results while something is typed.
ring-finding = { $typed } を検索中
# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = 日付と時刻
# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = 新しいウィンドウ
# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
       *[other] { $n } 個のウィンドウをすべて閉じる
    }
# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = リングから固定を解除
# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = お気に入りから外す
# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = お気に入りに追加

ring-startup-add = ログイン時に起動

ring-startup-remove = ログイン時に起動しない
# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = なぜですか。
# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    中央には { $n } 個まで置けます。
    1 つ外すと空きができます。
# Choice on the ring's power page: lock the screen.
ring-power-lock = ロック
# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = スクリーンセーバー
# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = スリープ
# Choice on the ring's power page: hibernate.
ring-power-hibernate = ハイバネート
# Choice on the ring's power page: log out.
ring-power-log-out = ログアウト
# Choice on the ring's power page: restart the computer.
ring-power-restart = 再起動
# Choice on the ring's power page: turn the computer off.
ring-power-shut-down = シャットダウン
# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = 「すべてのアプリ」を押し、アプリを右クリックするとここに残せます
# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = クリックして確定してください。
# Line in the ring when something other than the person tried to shut the computer down.
ring-power-only-own = このコンピューターをシャットダウンできるのは、あなただけです。
# Line in the ring when the power page closed because a program pressed a key.
ring-power-closed = プログラムがキーを送ったため、電源のページを閉じました。

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = 再起動しました
# Under the notice above.
ring-restarted-why = リングと補助部品を再起動しました。

# Said under the ring when sleep, log out, restart or shut down is pressed while Trindle is being installed.
ring-power-installing = Trindle をインストールしています。終わるまでお待ちください。
