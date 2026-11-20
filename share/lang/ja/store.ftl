# machine translated (Japanese), not yet reviewed by a person.

# Apps: the window, its side bar and the front pages (src/store/mod.rs, catalogue.rs, dropped.rs).

## The window and its side bar (src/store/mod.rs)

# Title of the Apps window, and the program's name.
store-title = アプリ
# Typing box at the top of Apps, shown quietly while empty.
store-search-placeholder = アプリを検索
# Side bar place: the front page of apps to try.
store-nav-discover = 見つける
# Side bar place: the apps that are installed.
store-nav-installed = インストール済み
# Side bar place: updates, when none are waiting.
store-nav-updates = アップデート
# Side bar place: updates, with how many are waiting.
store-nav-updates-waiting = アップデート ({ $n })
# Side bar place: plugins.
store-nav-plugins = プラグイン
# Side bar place and title: the AppImages AppImageHub lists.
store-nav-browse = AppImage を探す
# Side bar place and title: the one detailed search over Flathub, AppImages and GitHub.
store-nav-github = 検索
# Heading of the Flatpaks a detailed search found on Flathub.
store-gh-flathub = Flathub の Flatpak ({ $n })
# Side bar place: where software comes from.
store-nav-sources = ソフトウェアの入手元
# Side bar place, title of the page and its way back: moving apps and settings to another computer.
store-move = アプリを移す
# Small heading over the side bar's first group.
store-group-apps = アプリ
# Small heading over the pages from Settings that are about apps.
store-group-settings = アプリの設定
# Small heading over the categories of apps.
store-group-categories = カテゴリ
# Title of a page of search results; the words searched for are in the quotes.
store-grid-results = 「{ $text }」の検索結果
# Title of the front page of apps, and of a page before the catalogue has come.
store-grid-discover = 見つける
# Heading of the apps most people want, on the front page.
store-grid-popular = 人気
# Title of the page of apps from a source, before the catalogue has come.
store-grid-apps = アプリ
# Red words under the search box when an app asked for is not in the catalogue.
store-not-in-catalogue = { $id } は Flathub のカタログにありません。

## Categories of apps (src/store/catalogue.rs)

# Side bar category: music, film and sound.
store-cat-audio-video = 音楽と動画
# Side bar category: drawing and photographs.
store-cat-graphics = 画像と写真
# Side bar category: documents and spreadsheets.
store-cat-office = オフィス
# Side bar category: browsers and chat.
store-cat-network = インターネット
# Side bar category: programming tools.
store-cat-development = 開発
# Side bar category: teaching and learning.
store-cat-education = 学習
# Side bar category: games.
store-cat-game = ゲーム
# Side bar category: science.
store-cat-science = 科学
# Side bar category: small everyday tools.
store-cat-utility = ユーティリティ
# Side bar category: tools for the computer itself.
store-cat-system = システム
# Red words when the list of apps could not be read.
store-catalogue-unreadable = Trindle はカタログを読み込めませんでした: { $why }
# Red words when Flathub's list of apps could not be fetched. { $why } is what Flatpak said.
store-catalogue-not-fetched = Trindle は Flathub からアプリの一覧を取得できませんでした。インターネットの接続を確認してください。{ $why }

## Dropping a file on the window (src/store/dropped.rs)

# Red words under the search box when nothing that was dropped on the window is a kind Apps takes.
store-dropped-only = ここにドロップできるのは .deb、.AppImage、.flatpakref のファイルだけです。

## Why an app was not updated (src/store/mod.rs)

# Red words under an app that Flatpak did not update; what Flatpak said follows.
store-not-updated-said = Flatpak は更新しませんでした: { $why }
# Red words under an app that Flatpak did not update and gave no reason for.
store-not-updated-none = 更新されませんでした。理由は伝えられていません。
