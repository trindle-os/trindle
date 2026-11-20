# machine translated (Japanese), not yet reviewed by a person.

# The Settings window itself: its title, the side bar with its search box and group headings, and the words printed by its command line options. Each page's own words are in the settings-<page>.ftl files.

## Window (src/settings/mod.rs)

# Title of the Settings window, spoken by a screen reader and shown in the window list.
settings-window-title = 設定
# Typing box at the top of the Settings side bar; it filters the pages.
settings-window-search = 設定を検索
# Small grey heading over the side bar's first group of pages: how it looks and sounds.
settings-window-group-personal = 個人用
# Small grey heading over the side bar's group of pages for the computer's parts.
settings-window-group-devices = デバイス
# Small grey heading over the side bar's group of pages for networks.
settings-window-group-network = ネットワーク
# Small grey heading over the side bar's group of pages for the system.
settings-window-group-system = システム
# Small grey heading over the side bar's last group of pages: about Trindle, thanks and help.
settings-window-group-about = 情報
# Small grey heading over pages a search finds that are in the Apps program.
settings-window-in-apps = Apps 内
# Small grey heading over the row a search finds that opens the Folders program's settings.
settings-window-in-folders = Folders 内
# The side bar row that opens the Folders program at its own settings.
settings-window-in-folders-name = Folders 自体の設定
# Words the Settings search matches the Folders settings row by, in lower case, separated by spaces.
settings-window-in-folders-keywords =
    folders files file manager start pane home recent drives usb stick sd card plugged in open eject list endings extensions names tick boxes checkboxes picking selecting rows compact normal roomy フォルダー ファイル ファイルマネージャー ホーム 最近 ドライブ メモリー カード 取り出す 拡張子 名前 チェックボックス 選択 行 コンパクト
# Quiet line in the side bar when a search finds no page.
settings-window-no-page = 一致するページがありません。もっと短い言葉を試すか、ヘルプを見てください。

## Command line (src/settings/mod.rs)

# Printed in a terminal by trindle-settings --themes for a theme that cannot be used; id is its short name, name its full name, why the reason.
settings-window-theme-refused = {"  "}{ $id }  { $name }(使用できません: { $why })
# Printed in a terminal when trindle-settings --theme is given no theme name.
settings-window-theme-name = テーマ名を入力してください。例: trindle-settings --theme default
# Printed in a terminal after trindle-settings --theme worked; theme is the theme's name.
settings-window-theme-now = テーマを { $theme } にしました。
# The name printed for the built-in theme in the line above.
settings-window-theme-own = Trindle 標準
# Printed in a terminal when the theme could not be chosen; error says why and ends with a full stop.
settings-window-themes-list = { $error } 一覧を見るには trindle-settings --themes を実行してください。
# Printed in a terminal when trindle-settings --keys is not given the word default.
settings-window-keys-how = すべてのキーとボタンを元に戻すには、次を実行してください: trindle-settings --keys default
# Printed in a terminal after every key and button was put back.
settings-window-keys-back = すべてのキーとボタンを元に戻しました。
# Printed in a terminal when the keys could not be put back; error says why.
settings-window-keys-failed = Trindle はキーを元に戻せませんでした: { $error }
