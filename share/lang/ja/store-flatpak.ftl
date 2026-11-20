# machine translated (Japanese), not yet reviewed by a person.

# The page shown before a Flatpak program is installed, with what it will be able to reach.
# Headline over the page; the source of the program is given.
store-fp-headline-from = { $host } から { $name } をインストールしますか?
# Headline over the page for a program that has been downloaded.
store-fp-headline = { $name } をインストールしますか?
# Name of the card that lists what the program will be able to reach.
store-fp-card = アクセスできるもの
# Tooltip of the round button that leaves without installing.
store-fp-cancel = インストールしない
# The main button.
store-fp-install = インストール
# Said while Flatpak is being asked what the program wants.
store-fp-reading = 求めているものを読み取っています…
# Red words when what the program asks for could not be read.
store-fp-unread = Trindle は、求めているものを読み取れませんでした。
# Red words after Install was pressed once for a program whose list could not be read.
store-fp-unread-again = 進めるには、もう一度「インストール」を押してください。
# Said when the program is already on this computer; the name is given.
store-fp-have = { $name } はすでにインストールされています。
# Said when the program asks to reach nothing outside its own box.
store-fp-none = 専用の箱の外には、何もアクセスしません。
# Each line below is one thing the program will be able to reach.
store-fp-network = インターネットとローカルネットワーク
store-fp-x11 = 他のアプリのウィンドウと、そこでの入力内容
store-fp-sound = 音声とマイク
store-fp-devices = カメラと、接続されているその他すべての機器
store-fp-input = ゲームコントローラーなどの入力機器
store-fp-kvm = 高速な仮想マシン
store-fp-shm = 他のアプリと共有するメモリー
store-fp-usb = USB 機器
store-fp-canbus = CAN バス機器
store-fp-cups = プリンター
store-fp-multiarch = 32 ビットのプログラムの実行
store-fp-escape = このコンピューターの任意のプログラムの実行
# A reference that also adds a second place to get apps from, named by its address.
store-fp-adds-source = アプリの入手先に { $host } を追加
# Last line, when the program asks for more than the lines above say.
store-fp-other = ほかの設定も含まれます
store-fp-bluetooth = Bluetooth 機器
store-fp-smartcard = スマートカード
store-fp-ssh = SSH キー
store-fp-gpg = GPG キー
store-fp-devel = デバッグ用ツール（自分が起動したプログラムを監視・操作できます）
store-fp-session-bus = セッションで動いているすべてのプログラム
store-fp-system-bus = このコンピューターのすべてのサービス
# A place the program can open and change; the place is given, as below.
store-fp-files = { $what }を開いて変更
# The same for a place it can only look at.
store-fp-files-read = { $what }を見る
store-fp-where-host = このコンピューターのすべてのファイル
store-fp-where-system = このコンピューターのシステムファイル
store-fp-where-home = 隠しファイルを含む、ホームフォルダ全体
store-fp-where-path = フォルダ { $path }
# A program on your session that it can talk to; the name is given.
store-fp-talk = { $name } と通信
# A service the program runs on your session.
store-fp-own = サービス { $name } を実行
store-fp-talk-system = システムサービス { $name } と通信
store-fp-own-system = システムサービス { $name } を実行
# Said in red when Install failed and taking the program out again failed too.
store-fp-undo-failed = 一部だけインストールされている可能性があります。
# Said in red when a bundle could not be copied for a safe read.
store-fp-copy-failed = 安全に読み取るためのファイルのコピーができませんでした。
