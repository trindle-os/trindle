# machine translated (Japanese), not yet reviewed by a person.

# Apps: the fence AI programs run in, and the window that asks the person (src/store/fence.rs, fence_window.rs).

## The fence's settings (src/store/fence.rs)

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = プログラム名として正しくありません。
# Red words when a fence setting could not be written to its file.
store-fence-not-saved = 保存できませんでした。
# Red words when the folder chosen to share does not exist.
store-fence-no-folder = 存在するフォルダを選んでください。
# Red words when the whole home folder is chosen to share.
store-fence-whole-home = ホームフォルダ全体は非公開のままです。
# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = このフォルダにはホームフォルダが含まれるため、非公開のままです。
# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = 共有できるのは、ホームフォルダの中のフォルダだけです。
# Red words when a hidden folder is chosen to share.
store-fence-hidden = 隠しフォルダは常に非公開です。
# Red words when a folder that keeps passwords or mail is chosen to share.
store-fence-passwords = このフォルダにはパスワードやメールがあるため、非公開のままです。
# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = そのプログラムには使える設定フォルダがありません。
# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = サイト名を入力してください。例: api.example.com。その名前の下のすべてのサイトなら *.example.com。
# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = コンピューター名を入力してください。例: nas.local。
# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = 1 行ずつ NAME=value の形で書いてください。
# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = 1 行ずつ NAME=value の形で、名前は大文字で書いてください。

## The window that asks about an AI program (src/store/fence_window.rs)

# Words above the question when an AI program asks to use a folder.
store-fence-ask-share = AI プログラムがこのフォルダを使いたがっています。今は見えません。
# Words above the question when an AI program asks to run something as the administrator.
store-fence-ask-admin = AI プログラムが、これを管理者として実行したがっています。フェンスの外で動き、あなたが見られるものはすべて見えます。
# Small heading of the window that asks about an AI program.
store-fence-ai-program = AI プログラム
# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = このフォルダの読み取りだけを許可
# Note under the question about a folder.
store-fence-sees-next = 次にプログラムを起動したときから、このフォルダが見えます。
# Tip of the round button that refuses a folder.
store-fence-no = いいえ
# Tip of the round button that shares a folder until the session ends.
store-fence-share-session = このセッションだけ共有
# Main button that shares a folder for good.
store-fence-share-always = 常に共有
# Tip of the round button that refuses to run something as the administrator.
store-fence-dont-allow = 許可しない
# Main button that lets something run as the administrator.
store-fence-allow = 許可

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = そのコマンドは長すぎて全体を表示できないため、実行しませんでした。

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = 別の要求が、ユーザーの返事を待っています。

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = ユーザーはこれを許可しませんでした。

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } はフェンスの中では使えません。

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = フェンスの外で起動できませんでした。

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = フォルダの完全なパスを指定してください。

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = ユーザーはフォルダを共有しませんでした。

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = 共有しました。このプログラムを次に起動したときから使えます。

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = 接続先のコンピューター名を指定してください。

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = フェンスの中の AI プログラムは、そのコンピューターに ssh で接続できません。

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = AI プログラム用の鍵を作れませんでした: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = 実行するコマンドを指定してください。

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } はここでは使えません。

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } はフェンスの中では使えません。フェンスの外で実行するよう頼んでください。

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: フェンスの要求用の接続がありません。

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: 要求を送れませんでした。

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock が拒否しました: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = フェンスを起動できませんでした: { $why }。

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = フェンスを起動できませんでした: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = ここでは、AI プログラムはあなたとは別に、自分のログインを持ちます。求められたら、プログラムの中でログインしてください。

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = あなたのフォルダは AI プログラムから見えません。そこで作業するには、設定の AI プログラムで共有してください。

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = フェンスを起動できませんでした: システムコールのフィルターを作れませんでした。

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = フェンスを起動できませんでした: ネットワークの制限を設定できませんでした。

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = フェンスを起動できませんでした: bubblewrap が動きませんでした: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: フェンスの外に見つかりません
