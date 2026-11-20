# machine translated (Japanese), not yet reviewed by a person.

# Apps: lines of the lists of apps, where things came from and how long is left (src/store/updates.rs, listing.rs, look.rs, origin.rs, progress.rs, reviews.rs, key.rs).

## Where an installed thing came from (src/store/updates.rs)

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Flatpak ファイル
# On an installed thing's line: it came from a .deb file.
store-from-deb = .deb ファイル
# On an installed thing's line: it is a plugin.
store-from-plugin = プラグイン
# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }
# The kind of a pre-release version written rc.
store-version-rc = リリース候補

## A tile of the list of apps (src/store/listing.rs)

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = { $name } を削除中
# Tip of the round button on an app's tile while it is being installed.
store-list-installing = { $name } をインストール中
# Tip of the round button on an app's tile that opens the app.
store-list-open = { $name } を開く
# Tip of the round button on an app's tile that installs the app.
store-list-install = { $name } をインストール
# Line on an installed app's tile when the app has no line of its own.
store-list-installed = インストール済み

## Tabs and times (src/store/look.rs, progress.rs)

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }
# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = 表示

## Where a download came from (src/store/origin.rs)

# Credit under a package's page for the list its site was checked against.
store-origin-credit = HaGeZi's Threat Intelligence リストと照合しました
# Where a download came from: the site and the page that led to it.
store-origin-both = { $referrer } から移動して、{ $site } からダウンロードしました。
# Where a download came from: the site.
store-origin-site = { $site } からダウンロードしました。
# Where a download came from: the page that led to it.
store-origin-referrer = { $referrer } からのリンクです。
# Said when nothing tells where a download came from.
store-origin-unknown = Trindle は、これの入手元を判断できません。
# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } { $name } は、有害なダウンロードや詐欺で知られるサイトを集めた HaGeZi's Threat Intelligence リストに載っています。入手元をよく確かめてからインストールしてください。
# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } このサイトは HaGeZi's Threat Intelligence リストにありません。ただし、このリストがすべてを網羅しているわけではないので、安全とは限りません。
# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } HaGeZi's Threat Intelligence リストを取得できなかったため、Trindle はサイトを確認できませんでした。

## Reviews and the GitHub key's keyring (src/store/reviews.rs, key.rs)

# Name shown for a review whose writer gave none.
store-review-someone = 名無しさん
# Red words when the keyring could not be reached.
store-key-unreached = Trindle はキーリングに接続できませんでした: { $why }。
# Red words when the key kept in the keyring is not text.
store-key-not-text = 保存されているキーはテキストではありません。
# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } を削除できませんでした: { $why }
# Red words when a file that held the key could not be read.
store-key-not-read = { $path } を読み込めませんでした: { $why }
