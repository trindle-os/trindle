# machine translated (Japanese), not yet reviewed by a person.

## Screenshots, the eye dropper and Trindle's own portals (src/screenshot): the picture window, what the desktop asks through the portals, and the file chooser

## The screenshot window (src/screenshot/mod.rs, toolbar.rs, marks.rs, pin.rs)

# Name of the screenshot overlay that covers the screen (what a screen reader calls it).
screenshot-name = スクリーンショット
# Note on the overlay before any part is chosen.
screenshot-drag = 残したい部分をドラッグしてください
# Name of the round button that keeps the whole screen.
screenshot-whole = 画面全体
# Name of the round button that closes the overlay or a pinned picture.
screenshot-close = 閉じる
# Word on the button, shown when the report window asked for the picture, that hands the chosen part to the report.
screenshot-add-to-report = レポートに追加
# Name of the round button that gives the screenshot up.
screenshot-cancel = キャンセル
# Name of the round button that takes back the last mark.
screenshot-undo = 元に戻す
# Name of the round button that puts back the last mark taken away.
screenshot-redo = やり直す
# Name of the round button that copies the picture to the clipboard, and of the copy button of a colour.
screenshot-copy = コピー
# Name of a colour's button after it has been copied.
screenshot-copied = コピーしました
# Word on the button that saves the picture in the Screenshots folder.
screenshot-save = 保存
# Word on the button that asks where to save the picture.
screenshot-save-as = 名前を付けて保存
# Name of the round button that keeps the picture on the screen as a small window.
screenshot-pin = 画面に固定
# Name of the drawing tool that draws freehand.
screenshot-tool-pen = ペン
# Name of the drawing tool that draws a straight line.
screenshot-tool-line = 線
# Name of the drawing tool that draws an arrow.
screenshot-tool-arrow = 矢印
# Name of the drawing tool that draws a box.
screenshot-tool-box = 四角
# Name of the drawing tool that draws a circle.
screenshot-tool-circle = 円
# Name of the drawing tool that highlights.
screenshot-tool-highlighter = 蛍光ペン
# Name of the drawing tool that writes words.
screenshot-tool-words = 文字
# Name of the drawing tool that puts numbered spots.
screenshot-tool-numbers = 番号
# Name of the drawing tool that blurs a patch.
screenshot-tool-blur = ぼかし
# Name of the round button that draws in red.
screenshot-colour-red = 赤
# Name of the round button that draws in blue.
screenshot-colour-blue = 青
# Name of the round button that draws in green.
screenshot-colour-green = 緑
# Name of the round button that draws in orange.
screenshot-colour-orange = オレンジ
# Name of the round button that draws in purple.
screenshot-colour-purple = 紫
# Name of the round button that draws in white.
screenshot-colour-white = 白
# Name of the round button that draws in black.
screenshot-colour-black = 黒
# Name of the round button that draws thin lines.
screenshot-thickness-thin = 細い
# Name of the round button that draws lines of middle thickness.
screenshot-thickness-medium = 普通
# Name of the round button that draws thick lines.
screenshot-thickness-thick = 太い
# Name the screen reader gives to a screenshot pinned on the screen.
screenshot-pinned = 固定したスクリーンショット
# Title of the window that asks where to save the screenshot.
screenshot-save-title = スクリーンショットを保存
# Title of the notice that says the screenshot could not be saved.
screenshot-not-saved = スクリーンショットを保存できませんでした
# Title of the notice that says the screenshot could not be copied.
screenshot-not-copied = スクリーンショットをコピーできませんでした
# Words of that notice.
screenshot-clipboard-refused = Trindle はコピーできませんでした。
# Words of the notice that the screenshot could not be saved: the picture was not made.
screenshot-no-picture = スクリーンショットを撮れませんでした。
# Words of the notice that the screenshot could not be saved: no bus to ask on.
screenshot-no-bus = Trindle は保存先を尋ねられません。
# Words of the notice that the screenshot could not be saved: the file chooser could not be asked.
screenshot-no-chooser = ファイル選択を開けませんでした。
# Words of the notice that the screenshot could not be saved: every file name was taken.
screenshot-no-free-name = 空いている名前がありません
# Words of the notice that the screenshot could not be saved: the path and why it failed.
screenshot-write-failed = { $path }: { $why }

## The eye dropper (src/screenshot/colours.rs)

# Name of the eye dropper's window, and its title.
screenshot-eye-title = スポイト
# Words that float beside the i of the eye dropper's window.
screenshot-eye-help = スポイトを押してから、画面のどこかをクリックすると、ポインターの下の色を取り込みます。Escape を押すと中止します。取り込んだ色はすぐに16進コードでコピーされ、下のリストに新しい順に追加されるので、後でもう一度コピーできます。
# Name of the i button beside the title.
screenshot-eye-about = スポイトについて
# Name of the main button that takes a colour.
screenshot-eye-take = 色を選ぶ
# Sentence under the title while no colour has been taken.
screenshot-eye-none = 画面から色を選んでください。
# Sentence under the title once colours have been taken.
screenshot-eye-taken =
    { $n ->
       *[other] { $n } 色を選びました。最新は { $newest } です。
    }
# Shown in the big card while no colour has been taken.
screenshot-eye-latest-empty = まだ色を選んでいません。
# Name of the card that holds the latest colour.
screenshot-eye-latest = 最新の色
# Name of the round button that clears the list of colours.
screenshot-eye-clear = リストを消す
# Head of the card with the list of colours taken.
screenshot-eye-list = 選んだ色
# Shown in the list card while no colour has been taken.
screenshot-eye-list-empty = まだ色を選んでいません。
# Name of the round button that takes a colour out of the list.
screenshot-eye-remove = 外す
# What the saved list of colours is called when it is found damaged and set aside.
screenshot-eye-damaged = スポイトの色

## The picker for sharing the screen (src/screenshot/cast_pick.rs)

# Title of the window that asks what an app may share.
screenshot-pick-title = { $app } と画面を共有しますか?
# Sentence under the title when several things can be picked.
screenshot-pick-many = 1つ以上選んでください。アプリが共有をやめるか閉じるまで、選んだものだけが共有されます。
# Sentence under the title when one thing can be picked.
screenshot-pick-one = 1つ選んでください。アプリが共有をやめるか閉じるまで、選んだものだけが共有されます。
# Heading over the screens that can be shared.
screenshot-pick-screens = 画面
# Heading over the windows that can be shared.
screenshot-pick-windows = ウィンドウ
# Shown when there is nothing to share.
screenshot-pick-nothing = 共有できるものがありません。
# Name of the choice that shares every screen at once.
screenshot-pick-desktop = デスクトップ全体
# Words under that choice.
screenshot-pick-desktop-detail = すべての画面を配置のまま共有
# Words under a screen's choice: its size in pixels.
screenshot-pick-size = { $width } × { $height }
# What the screen reader says for a choice that is picked.
screenshot-pick-chosen = { $name }、選択済み
# Name of the round button that refuses to share.
screenshot-pick-refuse = 共有しない
# Name of the main button that shares what is picked.
screenshot-pick-share = 共有

## The portals (src/screenshot/portal_*.rs)

# Name of the button that says no in a question from a program, and of its answer to the program.
screenshot-portal-deny = 拒否
# Name of the main button that says yes in a question from a program.
screenshot-portal-allow = 許可
# Name of the round button that gives up a window from a program.
screenshot-portal-cancel = キャンセル
# Name of the main button that opens, and the title of a window that opens a file.
screenshot-portal-open = 開く
# Title of the window that saves a file, and the word of its main button.
screenshot-portal-save = 保存
# Name of the box where a name is typed.
screenshot-portal-name = 名前
# Heading when an unknown program asks who the person is.
screenshot-account-asks-unknown = プログラムがあなたの情報を求めています
# Heading when a program asks who the person is.
screenshot-account-asks = { $who } があなたの情報を求めています
# Name the screen reader gives to the person's picture.
screenshot-account-picture = あなたの画像
# Words under the person's name.
screenshot-account-signed-in = { $id } でログイン中
# Name of the search box in the window that asks which app should open a file.
screenshot-apps-search = アプリを探す
# Shown when no app can open the file.
screenshot-apps-none = 開けるアプリがありません
# Name of the switch that remembers the choice.
screenshot-apps-always = この種類のファイルには常にこのアプリを使う
# Title of the window that asks which app should open a file.
screenshot-apps-open-with = { $name } を開くアプリ
# Title of that window when the file's name is not known.
screenshot-apps-choose = アプリを選ぶ
# Name of the screen sharing that other programs see, and the name Trindle gives itself to PipeWire.
screenshot-cast-name = Trindle 画面共有
# What other programs show for the shared screen.
screenshot-cast-node = アプリと共有中の画面
# Name used for a program that cannot be named, in the window that asks what to share.
screenshot-cast-an-app = アプリ
# Title of the window that asks what to share.
screenshot-cast-window = 画面を共有
# Title of the window that saves several files into a folder.
screenshot-files-save-many = フォルダに保存
# Title of the window that chooses a folder.
screenshot-files-folder = フォルダを選ぶ
# What a program holding the computer awake is called when it gave no name.
screenshot-inhibit-who = プログラム
# Heading when a program wants to add an app to the person's apps and cannot be named.
screenshot-launcher-ask = これをアプリに追加しますか?
# Heading when a named program wants to add an app to the person's apps.
screenshot-launcher-asks = { $who } がこれをアプリに追加しようとしています
# Name the screen reader gives to the app's icon.
screenshot-launcher-icon = そのアイコン
# Words under the name, when the app opens a web address.
screenshot-launcher-opens = { $target } を開きます
# Title of the window that asks to add an app.
screenshot-launcher-title = アプリを追加
# Name of the main button that adds the app.
screenshot-launcher-add = 追加
# Heading when a program that cannot be named offers a wallpaper.
screenshot-wallpaper-ask = この画像を壁紙にしますか?
# Heading when a program offers a wallpaper.
screenshot-wallpaper-asks = { $who } が壁紙を設定しようとしています
# Name the screen reader gives to the picture offered.
screenshot-wallpaper-picture = 新しい壁紙
# Name of the main button that sets the wallpaper.
screenshot-wallpaper-set = 壁紙にする

## Printing (src/screenshot/portal_print.rs)

# Red line under the printer box when no printer is set up.
screenshot-print-no-printer = プリンターが設定されていません。設定の「プリンター」で追加してください。
# Red line under the printer box when the printing system does not answer.
screenshot-print-not-answering = 印刷が応答していません。
# Red line under the printer box when printing is not installed.
screenshot-print-not-installed = このコンピューターには印刷がインストールされていません。
# Red line under the printer box when the chosen printer is paused.
screenshot-print-paused = { $name } は一時停止中です。
# Label and name of the box where the printer is chosen.
screenshot-print-printer = プリンター
# Label and name of the choice of which pages are printed.
screenshot-print-pages = ページ
# The choice to print all the pages.
screenshot-print-all = すべて
# The choice to print a range of pages.
screenshot-print-range = 範囲
# Label and name of the box for the number of copies.
screenshot-print-copies = 部数
# Label and name of the choice of printing on both sides.
screenshot-print-sides = 両面
# Label and name of the choice of colour, and the choice for colour.
screenshot-print-colour = カラー
# The choice to print in black and white.
screenshot-print-mono = 白黒
# Label and name of the choice of paper size.
screenshot-print-paper = 用紙
# Label and name of the choice of how the page stands.
screenshot-print-page = 向き
# Name of the main button that prints, and the title when nothing else is known.
screenshot-print-go = 印刷
# Title of the print window for a named document.
screenshot-print-doc = { $doc } を印刷
# Title of the print window when the program has no document name.
screenshot-print-from = { $who } から印刷
# Title of the notice that says a print did not go.
screenshot-print-failed = 印刷できませんでした
# Words of that notice when the choices made were not kept.
screenshot-print-lost = 設定を保存できませんでした。もう一度印刷してください。

## The file chooser (src/screenshot/file_chooser.rs)

# Line in the menu on a file or folder: change its name.
screenshot-chooser-rename = 名前を変更
# Line in the menu on a file or folder: cut it.
screenshot-chooser-cut = 切り取り
# Line in the menu: paste what was cut or copied.
screenshot-chooser-paste = 貼り付け
# Line in the menu and name of the round button: make a new folder.
screenshot-chooser-new-folder = 新しいフォルダ
# Line in the menu: move to the Trash.
screenshot-chooser-trash = ごみ箱に移動
# Line in the menu on a file or folder: delete it for good, after asking.
screenshot-chooser-delete = 完全に削除
# Question before deleting for good.
screenshot-chooser-delete-ask = 完全に削除しますか?
# Line in the menu: copy the place of the thing as text.
screenshot-chooser-copy-location = 場所をコピー
# Line in the menu on a folder: pin it to Folders' side bar.
screenshot-chooser-pin = サイドバーに固定
# Line in the menu on a folder pinned already.
screenshot-chooser-unpin = 固定を外す
# Red line after a pin could not be kept.
screenshot-chooser-pin-not-kept = 固定を保存できませんでした。
# Red line after some items could not be deleted.
screenshot-chooser-not-deleted =
    { $n ->
       *[other] Trindle は { $n } 件の項目を削除できませんでした: { $names }。
    }
# What the screen reader says for the list of places down the left of the file chooser.
screenshot-chooser-places = 場所
# What the screen reader says for the list of files and folders in the file chooser.
screenshot-chooser-files = ファイル
# Name of the first place in the side bar, and of the only part of the path when Recent is shown.
screenshot-chooser-recent = 最近
# Name of the person's home folder in the side bar.
screenshot-chooser-home = ホーム
# Name the screen reader gives to the path along the top.
screenshot-chooser-folder = フォルダ
# Shown when a folder has nothing in it.
screenshot-chooser-empty = 何もありません
# Name of the choice of which kind of file is shown.
screenshot-chooser-kind = ファイルの種類
# Said where a place on another computer is not answering.
screenshot-chooser-not-answering = 相手のコンピューターが応答していません。
# What the saved list of last folders is called when it is found damaged and set aside.
screenshot-chooser-damaged = 選択ダイアログのフォルダ
# Words under the box when the folder a name points into is not there.
screenshot-chooser-no-folder = そのフォルダはありません。
# Words under the box when saving would replace a file.
screenshot-chooser-replace-ask = { $name } はすでにあります。置き換えるには「置き換える」を押すか、もう一度 Enter を押してください。
# Words under the box when a key is pressed on Recent, which is not a folder.
screenshot-chooser-recent-not-folder = 「最近」はフォルダではありません。先にフォルダを開いてください。
# Words under the box when a key is pressed with nothing picked.
screenshot-chooser-nothing-picked = 何も選ばれていません。
# Words under the box while a file by that name is already there.
screenshot-chooser-there = { $name } はすでにあります。
# Name of the round button that hides the hidden files.
screenshot-chooser-hide-hidden = 隠しファイルを隠す
# Name of the round button that shows the hidden files.
screenshot-chooser-show-hidden = 隠しファイルを表示
# Words of the main button when the program gave none, to choose a folder.
screenshot-chooser-choose-folder = フォルダを選ぶ
# Head of the question when a folder of that name is already there.
screenshot-chooser-clash-folder = { $name } というフォルダはすでにあります
# Head of the question when a file of that name is already there.
screenshot-chooser-clash-file = { $name } はすでにあります
# Name of the round button that leaves a pasted thing out.
screenshot-chooser-skip = スキップ
# Name of the round button that keeps both under different names.
screenshot-chooser-both = 両方残す
# Word on the button that replaces what is there.
screenshot-chooser-replace = 置き換える
# What the screen reader says for the button that replaces a folder.
screenshot-chooser-replace-spoken = 置き換える。今あるフォルダはごみ箱に移されます。
# Name of the main button that merges two folders.
screenshot-chooser-merge = 統合
# Words in the question when folders are being pasted.
screenshot-chooser-rest-folders = すでにあるすべてのフォルダに同じ操作をします。
# Words in the question when files are being pasted.
screenshot-chooser-rest-files = すでにあるすべてのファイルに同じ操作をします。
# Name of the switch that gives the same answer to the rest.
screenshot-chooser-same = 残りにも同じ操作をする
# Words under the box after Cut.
screenshot-chooser-was-cut = 移動の準備ができました: { $what }
# Words under the box after Copy.
screenshot-chooser-was-copied = コピーの準備ができました: { $what }
# Words under the box when Paste is pressed and there is nothing to paste.
screenshot-chooser-nothing-cut = 切り取りもコピーもされていません。
# Words under the box when Paste is pressed while a paste is still going.
screenshot-chooser-paste-busy = Trindle はまだ貼り付け中です。
# Red line after some items could not be moved.
screenshot-chooser-not-moved =
    { $n ->
       *[other] Trindle は { $n } 件の項目を移動できませんでした: { $names }。
    }
# Red line after some items could not be copied.
screenshot-chooser-not-copied =
    { $n ->
       *[other] Trindle は { $n } 件の項目をコピーできませんでした: { $names }。
    }
# Red line after some items could not be put in the Trash.
screenshot-chooser-not-trashed =
    { $n ->
       *[other] Trindle は { $n } 件の項目をごみ箱に移せませんでした: { $names }。
    }
# Name of the box where a file or folder is renamed.
screenshot-chooser-new-name = { $old } の新しい名前
# What several things are called in a line such as Cut.
screenshot-chooser-items =
    { $n ->
       *[other] { $n } 件
    }

## Email (src/screenshot/portal_email.rs)

# Title of the notice when an email could not be started.
screenshot-mail-failed = メールを作成できませんでした
# Why: no mail program is chosen or installed.
screenshot-mail-no-app = メールのプログラムが選ばれていません。設定の「既定のアプリ」で選んでください。
# Why: a program asked to attach more files than allowed.
screenshot-mail-too-many = 添付するファイルが多すぎます。
# Why: a file, or all the files together, are too big.
screenshot-mail-too-big = ファイルが大きすぎて添付できません。
# Why: a file could not be read as a plain file.
screenshot-mail-bad-file = ファイルを添付できませんでした。
# Why: an address was not a plain address.
screenshot-mail-bad-address = アドレスが正しくありません。
# Why: the subject was too long or had odd characters.
screenshot-mail-bad-subject = 件名が正しくありません。
# Why: the words were too long or had odd characters.
screenshot-mail-bad-words = 本文が正しくありません。
# Why: no private place to keep the files.
screenshot-mail-no-place = ファイルを安全に保管できませんでした。
# Why: the mail program did not start.
screenshot-mail-not-started = メールのプログラムが起動しませんでした。
