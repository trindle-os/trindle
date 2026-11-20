# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Default apps": which program opens each kind of file or link.

## Page (src/settings/panels/default_apps.rs)

# Name of the Settings page about default apps, in the list and as its heading.
settings-default-apps-title = 既定のアプリ
# Words a person might type into the Settings search to find the default apps page; separate them with spaces.
settings-default-apps-keywords = ブラウザ メール 電子メール ファイル 写真 画像 音楽 動画 pdf カレンダー 開く ターミナル browser mail email files pictures music video calendar terminal
# Introduction under the heading of the default apps page, and the words behind the page i.
settings-default-apps-help = Webリンク、メール、写真、カレンダーなどを開くプログラムを選びます。ターミナルは ~/.config/xdg-terminals.list で選びます。選んでいないときは、インストール済みの最初のものを使います。

## Kinds of file

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-web = Web
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-mail = メール
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-folders = フォルダ
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-text = テキスト
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pictures = 写真
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-music = 音楽
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-video = 動画
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pdf = PDFドキュメント
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-calendar = カレンダー
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-web-in-sentence = Web
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-mail-in-sentence = メール
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-folders-in-sentence = フォルダ
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-text-in-sentence = テキスト
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pictures-in-sentence = 写真
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-music-in-sentence = 音楽
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-video-in-sentence = 動画
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pdf-in-sentence = PDFドキュメント
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-calendar-in-sentence = カレンダー

## Cards

# Title of a card on the default apps page.
settings-default-apps-card-internet = インターネット
# Words behind the i of a card on the default apps page.
settings-default-apps-card-internet-about = Webリンク、メールアドレス、カレンダーの招待を開くプログラムを選びます。
# Title of a card on the default apps page.
settings-default-apps-card-files = ファイルとドキュメント
# Words behind the i of a card on the default apps page.
settings-default-apps-card-files-about = フォルダ、テキストファイル、PDFドキュメントを開くプログラムを選びます。
# Title of a card on the default apps page.
settings-default-apps-card-media = 写真とサウンド
# Words behind the i of a card on the default apps page.
settings-default-apps-card-media-about = 写真、音楽、動画を開くプログラムを選びます。

## Page

# Sentence at the top of the page when every kind has a program; { $web } is the program that opens web links.
settings-default-apps-all-set-web = すべてに開くプログラムがあり、Webリンクは { $web } で開きます。
# Sentence at the top of the page when every kind has a program and the one for web links is not known.
settings-default-apps-all-set = すべてに開くプログラムがあります。
# Sentence at the top of the page when one kind has no program chosen; { $kind } is its name in lower case.
settings-default-apps-one-missing = { $kind }を開くプログラムを選んでください。
# Sentence at the top of the page when several kinds have no program chosen; { $kinds } is their names as a list joined with "or".
settings-default-apps-many-missing = { $kinds }を開くプログラムがまだ選ばれていません。
# Small words under a kind that no installed program can open.
settings-default-apps-none-installed = これを開けるプログラムは、まだインストールされていません。
# First choice in a list when no program is chosen for that kind yet.
settings-default-apps-not-chosen = まだ選んでいません
# Said in green under a kind's list after the choice was kept.
settings-default-apps-saved = 保存しました
