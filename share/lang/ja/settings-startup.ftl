# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Start at login": apps that open by themselves when someone logs in.

## Page (src/settings/panels/startup.rs)

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = ログイン時に起動
# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = 自動起動 スタートアップ 起動 ログイン 自動 遅延 autostart startup login boot delay
# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = ログインしたときに自動で開くアプリです。ネットワークや別のアプリを先に待てるように、遅延を設定できます。

## Top sentence

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = ログイン時に自動で起動するアプリはありません。
# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } はログイン時に自動で起動します。
# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
       *[other] { $n } 個のアプリがログイン時に自動で起動します。
    }
# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
       *[other] ほかに { $n } 個がオフになっています。
    }

## Delays

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = すぐに
# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = 1分後
# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = 2分後
# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
       *[other] { $n } 秒後
    }

## Each app's line

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = { $name } の遅延
# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = ログイン時に起動する
# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = ログイン時に起動しない
# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Trindle は削除できなかったため、ログイン時に起動したままです。

# Under the name of a launcher that comes with the system, which can only be switched on or off; { $command } is what it runs, cut short.
settings-startup-system = システム標準: { $command }

## Adding

# Red words under the add card when something could not be added.
settings-startup-not-added = 追加できませんでした。
# Placeholder of the search box above the list of installed apps.
settings-startup-find = アプリを探す
# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = ログイン時に起動に追加
# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = アプリがインストールされていないため、Trindle は追加できませんでした。
# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = syncthing --no-browser
# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = プログラムを選ぶ
# Heading of the line for adding a command.
settings-startup-command = コマンド
# Red words under the command line when nothing was typed.
settings-startup-command-empty = コマンドを入力するか、プログラムを選んでください。
# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = ログイン時に起動するプログラムを選ぶ

## Cards

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = 自動で起動するものはありません
# Title of the card listing what starts at login.
settings-startup-list-title = ログイン時に起動
# Words behind the i of the card listing what starts at login.
settings-startup-about = ネットワークや別のアプリが先に使えるように、ログイン後にしばらく待ってから起動できます。
# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } が実行: { $command }
# Name of the button that opens the list of installed apps.
settings-startup-open-apps = インストール済みのアプリから選ぶ
# Name of the button that opens the box for typing a command.
settings-startup-open-command = コマンドを入力するか、プログラムを選ぶ
# Heading of the line for adding an installed app.
settings-startup-app-heading = インストール済みのアプリ
# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = コマンドまたはプログラム
# Words behind the i of the Add an app card.
settings-startup-add-about = インストール済みのアプリを選ぶか、ターミナルと同じように実行するコマンドを入力します。ディスク上のプログラムも選べます。次のログインから起動します。一覧に入ったら、遅延を選べます。
# Title of the card for adding an app.
settings-startup-add-title = アプリを追加
