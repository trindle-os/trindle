# machine translated (Japanese), not yet reviewed by a person.

# Settings: words shared by the pages (src/settings/ui.rs).

## Shared by the pages (src/settings/ui.rs)

# Said beside a line once its change is saved.
settings-common-saved = 保存しました
# Said beside a button that worked, when nothing more is known.
settings-common-done = 完了
# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = 保存しました|完了|使用中|変更しました|設定しました|使用済み|追加しました|解放しました|削除しました|保存済み|インストールしました|更新しました|参加しました|コピーしました|既定にしました|再生しました|送信しました|接続しました|接続中|オン|オフ|忘れました|80%で止まります|満充電まで充電します|ログイン時に開始|保持しますが開始しません|最新です
# Said beside a button that failed, when nothing more is known.
settings-common-not-done = できませんでした
# Tip of the information button at the top of every Settings page.
settings-common-about-page = このページについて
# Tip of the information button on a card: { $name } is the card's heading as it is written.
settings-common-about-card = { $name } について
# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } はすでにあります
# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
       *[other] 残りも同じようにする (あと { $n } 個)
    }
# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = 残りも同じようにする
# Tip of the button that keeps the one already there.
settings-common-skip = スキップして、今あるものを残す
# Button that replaces the one already there.
settings-common-replace = 置き換える
# Spoken name of the Replace button.
settings-common-replace-spoken = 今あるものを新しいもので置き換える
# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = 新しいもの
# Heading of the line that shows the one already there.
settings-common-already-line = すでにあるもの
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = すでにあるものを置き換えますか?
# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
       *[other] { $left } 秒後に元に戻します。
    }
# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
       *[other] { $left } 秒後に元に戻します。{ $more }
    }
# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = 今すぐ元に戻す
# Button at the foot of the window that keeps the change on trial.
settings-common-keep = 保持
# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } は使えません
# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = インストールするには、次を実行します: sudo apt install { $package }
# Red words when a web page could not be opened.
settings-common-no-browser = Trindleはウェブブラウザーを起動できなかったため、ウェブページを開けませんでした。
# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } を開けませんでした。
# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = 保存できませんでした。
# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = システムの一部が応答しないため、Trindleはそれを実行できませんでした。コンピューターを再起動してから、もう一度お試しください。
# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = ログインの一部が応答しないため、Trindleはそれを実行できませんでした。ログアウトして、もう一度ログインしてから、お試しください。
# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Trindleはファイルやフォルダーを選ぶウィンドウを開けませんでした。ログアウトして、もう一度ログインしてから、お試しください。
# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = ディスクがいっぱいです。
# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = 今、Trindleはディスクに書き込めません。
# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = これを行うシステムの部分が動いていません。
# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = 応答に時間がかかりすぎました。もう一度お試しください。
# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = パスワードが入力されませんでした。
# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = このアカウントでは実行できません。
# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = 必要なものがこのコンピューターにありません。
# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }
# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest }、{ $last }
# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest }、または { $last }
# What goes between names in a list, before the last two.
settings-common-list-comma = { "、" }

## The picture of the ring (src/settings/pictures.rs)

# Word drawn in the middle of that picture.
settings-common-ring-name = リング

# Words shared by Settings and other programs, from src/common: the charge limit and the virtual machine note.

## Charge limit (src/common/charge.rs)

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Trindleはノートパソコンの充電の制御にアクセスできませんでした: { $error }
# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = ノートパソコンが充電の上限を受け付けませんでした。
# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = このコンピューターでは、途中で充電を止められません。
# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = { $path } を設定できませんでした: { $error }

## Virtual machine note (src/common/graphics.rs)

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = 仮想マシンでTrindleを試す
# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindleは { $machine } で動いています。問題なく動きますが、仮想マシンは本物のコンピューターほど速くも滑らかにもなりません。速さは本物のハードウェアで判断してください。
# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindleは3Dグラフィックスのない { $machine } で動いています。本来はグラフィックスチップが描くものを、すべてプロセッサーが描かなければなりません。ウィンドウ、動画、ウェブページは本物のコンピューターより遅く感じるため、Trindleはアニメーションを省いています。動かしているコンピューターが対応していれば、仮想マシンのディスプレイ設定で3Dアクセラレーションをオンにしてください。大きな違いが出ます。本物のコンピューターにTrindleをインストールする場合、これらは当てはまりません。
