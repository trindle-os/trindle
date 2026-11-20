# machine translated (Japanese), not yet reviewed by a person.

# Apps: the window for a .deb file, and programs kept in a box (src/store/deb.rs, boxed/, parts.rs, loose.rs).

## The window for a .deb file (src/store/deb.rs)

# Window title of the window opened for a .deb file.
store-deb-title = パッケージをインストール
# Words at the top of the window while the package is read.
store-deb-reading = パッケージを読み込んでいます…
# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = このファイルはインストールできる Debian パッケージではありません。ダウンロードが終わっていないのかもしれません。
# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } を専用のフォルダにインストールできます。
# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = このパッケージは { $arch } のコンピューター用なので、このコンピューターにはインストールできません。
# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = これは Trindle 本体です。ファイルからではなく、アプリの更新ページから更新してください。
# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } はドライバーまたはバックグラウンドサービスです。専用のフォルダでは動かないため、ここにはインストールできません。
# Words on the caution card above every package.
store-deb-caution = Debian と Trindle の外から来たパッケージは、専用のフォルダにインストールされます。そこでは制限されますが、完全に閉じ込められるわけではありません。信頼できる作者のパッケージだけをインストールしてください。
# Words at the top of the window once the package is installed.
store-deb-installed = { $name } は専用のフォルダにインストールされています。
# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = 入手元を確認しています…
# Heading in the card that says where the file came from.
store-deb-where = 入手元
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = 削除して、データも消す
# Word beside the ring while the package goes in.
store-deb-installing = インストール中
# Main button that installs the package.
store-deb-install = インストール
# Main button that installs the same version again.
store-deb-reinstall = 再インストール
# Main button that brings the installed package up to this version.
store-deb-update = 更新
# Heading of the card for a package not installed yet.
store-deb-goes-in = 専用の制限付きフォルダにインストールされます
# Under that heading.
store-deb-goes-in-note = パスワードは要りません。あなたが許可するまで、インターネットや音を使えず、ファイルも見られません。
# Heading of the card for a package installed at this version.
store-deb-same-version = バージョン { $version } が専用のフォルダにインストールされています
# Heading of the card for a package installed at another version.
store-deb-other-version = バージョン { $version } が専用のフォルダにインストールされています
# Under that heading: the version this brings.
store-deb-other-version-note = バージョン { $version } に更新します。保存したものはそのまま残ります。
# Facts line: how big the package is once installed.
store-deb-fact-size = インストール後のサイズ
# Facts line: the computer kind it is made for.
store-deb-fact-arch = 対応機種
# Text of the About card when the package has no description.
store-deb-no-description = このパッケージには説明がありません。
# Words at the top of the window while the package goes in.
store-deb-installing-note = 専用のフォルダにインストールしています。必要な部品のダウンロードに数分かかることがあります。
# Words at the top of the window once the package is installed.
store-deb-done = 専用のフォルダにインストールしました。リングにあります。
# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = 専用のフォルダにインストールしましたが、次の部品がないため、起動しないかもしれません：{ $parts }。
# Words at the top of the window once the package is removed.
store-deb-removed = 専用のフォルダから削除しました。保存したものもすべて消えました。

## Programs kept in a box (src/store/boxed/, parts.rs, loose.rs)

# Red words when a file is not a Debian package.
store-boxed-not-deb = このファイルはインストールできる Debian パッケージではありません。
# Step shown while a package is unpacked.
store-boxed-unpacking = { $name } を展開しています
# Step shown while a part a package needs is added.
store-boxed-adding = { $part } を追加しています
# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } はインストールできる Debian パッケージではありません。
# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path } を展開できませんでした。
# Red words when the program is not one kept in a box.
store-loose-not-boxed = そのプログラムは箱の中では動きません。
# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = インターネットなしで開きました
# Body of that notice: why, and what the program was given.
store-downloaded-why = ダウンロードしたものなので、プログラムはそのファイル1つだけを読めます。
# Name of Trindle's own terminal in the ring.
store-boxed-terminal = ターミナル
# Title of the notice when older programs are off.
store-boxed-x-off = 古いプログラムはオフです
# Body of that notice: where to turn them on.
store-boxed-x-off-where = 設定、権限
# Title of the notice when too many older programs are open.
store-boxed-x-many = 古いプログラムが多すぎます
# Body of that notice.
store-boxed-x-many-close = 1つ閉じてからやり直してください
# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = 古いプログラムを開けません
# Body of that notice: what to do.
store-boxed-x-unasked-why = 一度ログアウトして、ログインし直してから、やり直してください
# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } を開けませんでした
# Body of the notice the first time a terminal opens outside its box.
store-boxed-terminal-told = ターミナルは、他のアプリより多くの部分に触れられます。内容がわかるコマンドだけを、信頼できる人からのものに限って入力してください。
# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = ローカルネットワークなし
# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } はネットワークを使えません：{ $why }
# Why a plugin allowed the local network has none: Trindle could not find its own program.
store-boxed-home-why-self = Trindle のプログラムが見つからない
# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare がインストールされていない
# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta がインストールされていない
# The same: the network room was not made.
store-boxed-home-why-not-made = ネットワークを用意できなかった
# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = ネットワークがすでにあった
# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle が安全フィルターを用意できなかった
# The same: the room's rules would not load.
store-boxed-home-why-rules = ネットワークの規則を読み込めなかった
# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } を起動できなかった：{ $why }
# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta の応答：{ $why }
# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta をつなげられなかった

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = 最上位のフォルダ { $name } がリンクなので、このパッケージは使えません。
# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } はドライバーまたはバックグラウンドサービスです。専用のフォルダでは動かないため、インストールしませんでした。
