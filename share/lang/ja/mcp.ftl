# machine translated (Japanese), not yet reviewed by a person.

# Trindle's assistant access (MCP): the question window that asks the person
# before an assistant changes anything, and the words of its answer buttons.
# What the assistant reads back (its tool descriptions and errors) is written
# for the assistant, not the person, so it is not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The question window (src/mcp/ask.rs)

# Window title of the question an assistant's request raises.
mcp-title-ask = アシスタントからの依頼
# Heading over the words of what the assistant wants to do.
mcp-ask-heading = Trindle に接続したアシスタントが、次のことをしようとしています:
# Button that refuses the change the assistant asked for.
mcp-no = いいえ
# Button that lets the change the assistant asked for go ahead.
mcp-yes = はい、実行します

## What the question says, one line for each kind of change (src/mcp/tools.rs)

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = { $name } を前面に出す
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = 設定 { $key } を { $from } から { $to } に変える
# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = { $look } テーマを使う
# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = { $look } テーマを使い、アクセントカラーを { $accent } にする
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = { $id } を Flathub からインストールする
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = { $id } を削除する(ファイルは残します)
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = Wi-Fi ネットワーク { $name } に接続する
# The question for pairing a Bluetooth device that has no name; its address comes in.
mcp-ask-pair = Bluetooth デバイス { $address } とペアリングする
# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = Bluetooth デバイス { $name } ({ $address }) とペアリングする
# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = { $name } を閉じる
# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = { $name } を開く
# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
mcp-ask-allow = { $id } に { $what } の使用を許可する
# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = { $id } による { $what } の使用を止める
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = 設定とアプリの権限を、次の操作の前の状態に戻す: { $note }
# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = 設定、アプリの権限、{ $id } のファイルを、次の操作の前の状態に戻す: { $note }。{ $id } が開いている場合は、先に閉じます
# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = { $id } を閉じて、もう一度起動する。保存していない内容は失われます
# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = { $id } の { $file } で、{ $key } を { $from } から { $to } に変える。{ $id } が開いている場合は、先に閉じます
# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = { $id } の { $file } に、{ $key } を { $to } として追加する。{ $id } が開いている場合は、先に閉じます
# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = { $id } のキャッシュを消去する。{ $id } が開いている場合は先に閉じ、元に戻せるようキャッシュのコピーを残します
