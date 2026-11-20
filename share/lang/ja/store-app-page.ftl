# machine translated (Japanese), not yet reviewed by a person.

# Apps: the page for one app (src/store/app_page.rs).

## The page of one app (src/store/app_page.rs)

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }。
# Under the app's name when only the maker is known.
store-app-line-by = 作者: { $by }
# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }(作者: { $by })
# The state beside the dot while the app is being taken off.
store-app-state-removing = 削除中
# The state beside the dot while the app is being fetched.
store-app-state-downloading = ダウンロード中
# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = インストール中
# The state beside the dot while the app's own update goes in.
store-app-state-updating = アップデート中
# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = インストール済み、アップデート保留中
# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = インストール済み、バージョン { $version } でアップデート保留中
# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = インストール済み、{ $waits }
# The same, with the installed version.
store-app-state-waiting-version = インストール済み、バージョン { $version }、{ $waits }
# The state beside the dot for an installed app that is up to date.
store-app-state-current = インストール済み、最新です
# The same, with the installed version.
store-app-state-current-version = インストール済み、バージョン { $version }、最新です
# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = 未インストール
# The same, with the size of the download.
store-app-state-absent-size = 未インストール、ダウンロード { $size }
# Word beside the spinner while the app is being taken off.
store-app-busy-removing = 削除中
# Word beside the spinner while the app is being put in.
store-app-busy-installing = インストール中
# Tip and spoken name of the round button that gives up a download.
store-app-stop = 中止
# Tip and spoken name of the round button that takes the app off.
store-app-remove = 削除
# Round button beside Update or Release that opens the app.
store-app-open-round = 開く
# Main button that releases an app whose updates are held back.
store-app-release = アップデートを許可
# Main button that updates the app.
store-app-update = アップデート
# Main button that opens the app.
store-app-open = 開く
# Main button that installs the app.
store-app-install = インストール
# Button that opens the app's website.
store-app-website = Webサイト
# In the picture frame when the app has no pictures.
store-app-no-pictures = このアプリには画像がありません。
# In the picture frame when the picture could not be fetched.
store-app-picture-missed = Trindle はこの画像を取得できませんでした。
# In the picture frame while the picture is being fetched.
store-app-picture-fetching = 画像を取得中…
# In the picture frame when the computer has no internet.
store-app-picture-offline = 画像を見るにはインターネットに接続してください。
# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = 画像 { $at } / { $n }
# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = 画像 { $at } / { $n }: { $caption }
# Tip of the round button that shows the picture before.
store-app-picture-earlier = 前の画像
# Tip of the round button that shows the picture after.
store-app-picture-later = 次の画像
# Licence shown in Facts for an app that is not open source.
store-app-proprietary = プロプライエタリ
# Name of the Facts card.
store-app-facts = 詳細情報
# Facts line: the size of the download.
store-app-fact-download = ダウンロードサイズ
# Facts line: where the app comes from.
store-app-fact-source = 入手元
# Facts line: the version.
store-app-fact-version = バージョン
# Facts line: the licence of an open source app.
store-app-fact-licence = ライセンス
# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = ライセンス(オープンソースではありません)
# Facts line: who made the app.
store-app-fact-made-by = 作者
# Facts line: when the app's project was made.
store-app-fact-created = 作成日
# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  { $n ->
       *[other] { $count } 件の評価
    }
# Under the stars when nobody has rated the app.
store-app-rating-none = まだ評価はありません。
# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = 表示できる評価はありません。
# Under the stars while the ratings are being fetched.
store-app-rating-looking = 評価を探しています…
# Last line of the Rating card: where the ratings come from.
store-app-rating-source = GNOME のオープンなレビューサービスからです。
# Last line of the Rating card when there is no internet.
store-app-rating-offline = このコンピューターはインターネットに接続されていません。
# Name of the Rating card.
store-app-rating = 評価
# Text of the About card when the app says nothing of itself.
store-app-no-description = このアプリには説明がありません。
# Name of the About card.
store-app-about = このアプリについて
# Heading of a review that has no title.
store-app-review-no-title = タイトルなし
# Under What people say when nobody has written.
store-app-people-none = このアプリについて書いた人は、まだいません。
# Under What people say when there is no internet.
store-app-people-offline = レビューを読むにはインターネットに接続してください。
# Under What people say while reviews are being fetched.
store-app-people-looking = レビューを読み込み中…
# Name of the card of reviews.
store-app-people = レビュー
