# machine translated (Japanese), not yet reviewed by a person.

## Text (src/text): the plain text and Markdown editor, its menus, its find bar, its spelling and its settings.

## The window, the tabs and the menus (src/text/mod.rs)

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Text
# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = 無題
# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = 開く
# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = 新規 (Ctrl+N)
# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = 開く (Ctrl+O)
# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = 最近のファイル
# Button in the menu that saves the file as it is.
text-save = 保存
# The words a screen reader gives the Save button, with its shortcut.
text-save-key = 保存 (Ctrl+S)
# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = 名前を付けて保存
# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = 名前を付けて保存 (Ctrl+Shift+S)
# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = このファイルは、開いたあとに変更されました。
# Button beside that line: save the words here over the changed file.
text-save-anyway = それでも保存
# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = 再読み込み
# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = 元に戻す (Ctrl+Z)
# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = やり直す (Ctrl+Shift+Z)
# Button in the menu that opens the find bar, with its shortcut.
text-find-key = 検索 (Ctrl+F)
# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = 検索と置換 (Ctrl+H)
# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = 行へ移動 (Ctrl+G)
# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = プレビュー (Ctrl+E)
# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Web ページとしてエクスポート
# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = PDF としてエクスポート
# Button in the menu that turns on wrapping of long lines.
text-wrap = 長い行を折り返す
# Button in the menu that shows or hides the line numbers.
text-line-numbers = 行番号
# Button in the menu that opens the Text settings.
text-settings = 設定
# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = { $name } を閉じる
# Tip of the button that closes the open bar.
text-close-bar = バーを閉じる
# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = プレビュー
# Menu item that copies everything in the text.
text-copy-all = テキストをすべてコピー
# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = 切り取り
text-copy = コピー
text-paste = 貼り付け
text-select-all = すべて選択
# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = スペルチェック
# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ は { $encoding } では書き込めません。ステータスバーで別のエンコーディングを選んでください。
# The words on the status bar: the line and column of the caret.
text-line-column = { $line } 行、{ $col } 列
# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
       *[other] { $words } 語
    }、{ $letters ->
       *[other] { $letters } 文字
    }
# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
       *[other] { $words } 語
    }、{ $letters ->
       *[other] { $letters } 文字
    } を選択中
# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows の改行
# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = エンコーディング: { $name }
# Words in place of the file's name when the text has never been saved to a file.
text-no-file = このテキストはまだファイルになっていません。

## The find bar (src/text/mod.rs)

# Words on the find bar: the find box's label and placeholder.
text-find = 検索
# Words on the find bar: a switch to find only the same capitals.
text-match-case = 大文字と小文字を区別
# Words on the find bar: a switch to find only whole words.
text-whole-words = 単語全体
# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = 前へ (Shift+F3)
# Tip of the button that goes to the next find, with its shortcut.
text-next-key = 次へ (F3)
# Words on the replace bar: the replace box's label.
text-replace-with = 置換後
# Button on the replace bar that replaces the word found.
text-replace = 置換
# The words a screen reader gives the Replace button.
text-replace-spoken = 見つかった語を置換
# Button on the replace bar that replaces every word found.
text-replace-all = すべて置換
# The words a screen reader gives the Replace all button.
text-replace-all-spoken = 見つかったすべての語を置換
# Words on the find bar when a find has been asked for and has no match.
text-none-found = 一致なし
# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $n } 件中 { $at } 件目
# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } 件見つかりました
# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
       *[other] { $n } 件を置換しました
    }
# Words on the go to line bar when the number typed is not a line number.
text-type-line = 行番号を入力してください
# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 から { $lines }
# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = / { $lines }
# Button on the go to line bar that goes to the line typed.
text-go = 移動
# Words on the go to line bar: the box's label.
text-go-to-line = 行へ移動
# Button that goes back from a bar, a settings page or a list to the text.
text-back = テキストに戻る

## The spelling bar (src/text/mod.rs)

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = をあなたの単語に追加しました
# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = は辞書にありません。候補もありません
# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = は辞書にありません。次のどれですか?
# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = このつづりを使う
# Button on the spelling bar that adds the word to the person's own words.
text-add-word = 自分の単語に追加
# Heading over the list of suggested spellings.
text-suggestions = つづりの候補

## Recent files (src/text/mod.rs)

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = 開いたり保存したりしたファイルがここに表示されます。
# Button that clears the list of recent files.
text-clear-list = 一覧を消去

## The Text settings (src/text/mod.rs)

# Heading of the Text settings page.
text-settings-title = Text の設定
# Heading of the group on the settings page for when Text starts.
text-starting = 起動時
# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Text を開いたとき
# First choice: Text opens the files it had open last time.
text-reopen = 前回のファイルを開く
# Second choice: Text opens with an empty page.
text-start-blank = 空の状態で始める
# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = どちらを選んでも、保存していない作業は残り、復元されます。
# Heading of the group on the settings page for how the text looks.
text-showing = テキストの表示
# Label of the size slider: how big the text is.
text-size = サイズ
# Name of the size slider, read out by a screen reader.
text-text-size = 文字サイズ
# Heading of the group on the settings page for spelling.
text-spelling = スペルチェック
# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Hunspell の辞書がインストールされていません。
# Label of the list of spelling languages.
text-language = 言語
# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = スペルチェックの言語
# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = 自動 ({ $lang })
# Heading of the font list on the settings page.
text-font = フォント
# The first font in the list: the font Trindle ships with.
text-own-font = Trindle 標準

## Saving and the file dialogs (src/text/mod.rs)

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = ファイルを次の形式で読み込む
# Heading over the encodings a file is saved in.
text-save-in = 保存形式
# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = 次に保存するときから、ファイルはこのエンコーディングになります。
# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } に未保存の変更があります。
# Words saying the text has changes and no file yet.
text-not-saved = このテキストはファイルに保存されていません。
# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = { $name } を保存しますか?
# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after =
    { $left ->
       *[other] 保存するタブがあと { $left } 個あります。
    }
# Button in the question before closing that closes the tab without saving it.
text-close-no-save = 保存せずに閉じる
# Button in the question before closing that keeps the tab open.
text-keep-open = 開いたままにする
# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }
# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } はフォルダーです。
# Shown when a link in the finished look is pressed and is not a web address, or no browser started.
text-link-not-opened = ここから開けるのは Web アドレスだけです。

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } は大きすぎて、ここでは開けません。
# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = Trindle は { $name } を開けませんでした: { $reason }
# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = Trindle は { $name } に書き込めませんでした: { $reason }
# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = Trindle は { $name } を保存できませんでした: { $reason }
# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } を読み込めませんでした: { $reason }
# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Trindle は { $folder } に作業内容を保存できませんでした: { $reason }
# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = 設定を保存できませんでした: { $reason }
# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = Trindle は { $word } をあなたの辞書に追加できませんでした: { $reason }
# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Trindle は一覧を消去できませんでした: { $reason }
# Shown when the file chooser could not be opened.
text-chooser-failed = ファイル選択を開けませんでした。
# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Trindle は今、ファイル選択を開けません。
# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = { $lang } という辞書はありません。
# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Trindle は { $lang } の辞書を読み込めませんでした: { $reason }
# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }。元のテキストは { $path } に保存されています。
# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } 保存前のテキストは { $path } にあります。
# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = Text は { $name } の保存中に終了し、Trindle は保存された部分を読み込めません。
# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = Text は { $name } の保存中に終了しました。そのあとファイルが変更されたため、Trindle はそのままにしました。
# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = Text は { $name } の保存中に終了し、Trindle はファイルを元の状態に戻せませんでした: { $reason }。保存前のテキストは { $path } にあります。

# Quiet word beside what was just saved, exported or changed.
text-done = 完了

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = 無題.txt

# The misspelt word, shown in quotation marks on the spelling bar. { $word } is the word.
text-spell-word = ‘{ $word }’
